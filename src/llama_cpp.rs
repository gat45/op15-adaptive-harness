use crate::{
    interaction::assess,
    reasoning_router::{guidance, route},
};
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

const MAX_HTTP_RESPONSE_BYTES: u64 = 8 * 1024 * 1024;

/// Gives local models an explicit public-answer contract. This is a request,
/// not a trust boundary: `public_text` below also removes leaked think blocks.
pub fn public_answer_prompt(question: &str) -> String {
    let assessment = assess(question);
    let reasoning = guidance(&route(question));
    format!(
        "Tu es Adaptive Harness, un assistant local francophone, direct, attentif et utile. \n\
         Le harnais a classé ce tour comme : {}. C'est un signal prudent, pas une vérité sur l'intention. \n\
         {reasoning}\n\
         Avant de répondre, interprète silencieusement la demande littérale, le ton, l'implicite et les \n\
         liens avec l'historique et la mémoire éventuellement inclus. Privilégie l'interprétation la plus \n\
         bienveillante et cohérente avec ce contexte. Une remarque familière, ironique ou émotionnelle \n\
         appelle une réponse humaine et calme, pas un refus automatique. Si l'historique récent ou des souvenirs sont fournis dans la demande, reconnais-les comme le contexte disponible à ce tour : ne prétends pas que chaque échange est indépendant. Distingue toutefois cette mémoire injectée d'une mémoire personnelle ou d'un fait vérifié. \n\
         {}\n\
         Ne transforme jamais une mémoire déclarée par l'utilisateur en fait vérifié. N'invente ni faits \n\
         ni souvenirs absents. N'affiche jamais de raisonnement interne, de balises <think> ou d'étapes \n\
         cachées : donne seulement une réponse finale naturelle en français. Si une donnée manquante change matériellement la réponse, formule l'incertitude, pose une question brève ou propose le plus petit test réversible qui permettrait de la lever. N'affirme jamais qu'un test, une source ou une recherche a été exécuté si le harnais ne l'a pas réellement signalé.\n\nDemande : {question}",
        assessment.mode.label(),
        if assessment.ask_clarification {
            "Le contexte ne suffit pas pour une référence importante : demande une précision brève avant toute action irréversible."
        } else {
            "Réponds directement ; ne demande une précision que si les interprétations plausibles changent matériellement la réponse ou l'action."
        },
    )
}

/// Removes model-internal reasoning from text shown in the UI. An unclosed
/// `<think>` block is discarded rather than exposed as a partial answer.
pub fn public_text(text: &str) -> String {
    let mut visible = text;
    let mut output = String::new();
    loop {
        let Some(start) = visible.find("<think>") else {
            output.push_str(visible);
            break;
        };
        output.push_str(&visible[..start]);
        let after_start = &visible[start + "<think>".len()..];
        let Some(end) = after_start.find("</think>") else {
            break;
        };
        visible = &after_start[end + "</think>".len()..];
    }
    output.trim().to_owned()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Completion {
    pub text: String,
    pub total_tokens: Option<u64>,
    pub completion_tokens: Option<u64>,
    pub prompt_tokens: Option<u64>,
    pub cached_tokens: Option<u64>,
    pub prompt_ms: Option<f64>,
    pub prompt_per_second: Option<f64>,
    pub predicted_per_second: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct LlamaCppClient {
    pub host: String,
    pub port: u16,
    pub model: String,
}

impl LlamaCppClient {
    pub fn local(model: impl Into<String>) -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8080,
            model: model.into(),
        }
    }
    pub fn is_reachable(&self) -> bool {
        (self.host.as_str(), self.port)
            .to_socket_addrs()
            .ok()
            .and_then(|mut addresses| addresses.next())
            .is_some_and(|address| {
                TcpStream::connect_timeout(&address, Duration::from_millis(500)).is_ok()
            })
    }
    pub fn complete(&self, prompt: &str, max_tokens: u16) -> Result<Completion, String> {
        let body = serde_json::to_string(&json!({
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": max_tokens,
            "temperature": 0,
        }))
        .map_err(|error| format!("encode llama.cpp request: {error}"))?;
        let mut stream = TcpStream::connect((self.host.as_str(), self.port))
            .map_err(|e| format!("connect: {e}"))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(60)))
            .map_err(|e| format!("read timeout: {e}"))?;
        stream
            .set_write_timeout(Some(Duration::from_secs(10)))
            .map_err(|e| format!("write timeout: {e}"))?;
        let request = format!(
            "POST /v1/chat/completions HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            self.host,
            body.len(),
            body
        );
        stream
            .write_all(request.as_bytes())
            .map_err(|e| format!("write: {e}"))?;
        let raw = read_limited(&mut stream)?;
        let (status, payload) = raw.split_once("\r\n\r\n").ok_or("invalid HTTP response")?;
        if !status.split_whitespace().any(|part| part == "200") {
            return Err(format!("llama.cpp returned: {payload}"));
        }
        completion_from_payload(payload)
    }
}

fn read_limited(reader: &mut impl Read) -> Result<String, String> {
    let mut bytes = Vec::new();
    reader
        .take(MAX_HTTP_RESPONSE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("read: {e}"))?;
    if bytes.len() as u64 > MAX_HTTP_RESPONSE_BYTES {
        return Err("llama.cpp response exceeds 8 MiB limit".into());
    }
    String::from_utf8(bytes).map_err(|_| "llama.cpp response is not valid UTF-8".into())
}
fn completion_from_payload(payload: &str) -> Result<Completion, String> {
    let value: Value = serde_json::from_str(payload)
        .map_err(|error| format!("invalid llama.cpp JSON response: {error}"))?;
    let text = value
        .pointer("/choices/0/message/content")
        .and_then(Value::as_str)
        .ok_or("chat completion content missing or not text")?
        .to_owned();
    let number = |pointer: &str| value.pointer(pointer).and_then(Value::as_u64);
    let float = |pointer: &str| value.pointer(pointer).and_then(Value::as_f64);
    Ok(Completion {
        text,
        total_tokens: number("/usage/total_tokens"),
        completion_tokens: number("/usage/completion_tokens")
            .or_else(|| number("/timings/predicted_n")),
        prompt_tokens: number("/usage/prompt_tokens").or_else(|| number("/timings/prompt_n")),
        cached_tokens: number("/timings/cache_n"),
        prompt_ms: float("/timings/prompt_ms"),
        prompt_per_second: float("/timings/prompt_per_second"),
        predicted_per_second: float("/timings/predicted_per_second"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_completion() {
        let p = r#"{"choices":[{"message":{"content":"ok\n"}}],"usage":{"total_tokens":12,"completion_tokens":5},"timings":{"prompt_ms":96.5,"predicted_per_second":31.5}}"#;
        let completion = completion_from_payload(p).unwrap();
        assert_eq!(completion.text, "ok\n");
        assert_eq!(completion.total_tokens, Some(12));
        assert_eq!(completion.completion_tokens, Some(5));
        assert_eq!(completion.prompt_ms, Some(96.5));
        assert_eq!(completion.predicted_per_second, Some(31.5));
    }
    #[test]
    fn removes_complete_or_partial_thinking() {
        assert_eq!(public_text("<think>hidden</think> Bonjour"), "Bonjour");
        assert_eq!(public_text("<think>hidden"), "");
    }
    #[test]
    fn public_contract_requires_uncertainty_and_a_reversible_test() {
        let prompt = public_answer_prompt("Pourquoi cela ne marche plus ?");
        assert!(prompt.contains("pose une question brève"));
        assert!(prompt.contains("test réversible"));
        assert!(prompt.contains("N'affirme jamais qu'un test"));
    }
    #[test]
    fn rejects_oversized_response() {
        let binding = vec![b'x'; (MAX_HTTP_RESPONSE_BYTES + 1) as usize];
        let mut bytes = binding.as_slice();
        assert!(read_limited(&mut bytes).is_err());
    }
    #[test]
    fn serde_request_preserves_tabs_without_malformed_json() {
        let request = serde_json::to_string(&json!({"content": "a\tb\u{0001}\n"})).unwrap();
        let decoded: Value = serde_json::from_str(&request).unwrap();
        assert_eq!(decoded["content"], "a\tb\u{0001}\n");
    }
}
