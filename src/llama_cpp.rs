use std::{io::{Read, Write}, net::{TcpStream, ToSocketAddrs}, time::Duration};

const MAX_HTTP_RESPONSE_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion { pub text: String, pub total_tokens: Option<u64> }

#[derive(Debug, Clone)]
pub struct LlamaCppClient { pub host: String, pub port: u16, pub model: String }

impl LlamaCppClient {
    pub fn local(model: impl Into<String>) -> Self { Self { host: "127.0.0.1".into(), port: 8080, model: model.into() } }
    pub fn is_reachable(&self) -> bool {
        (self.host.as_str(), self.port).to_socket_addrs().ok().and_then(|mut addresses| addresses.next()).is_some_and(|address| TcpStream::connect_timeout(&address, Duration::from_millis(500)).is_ok())
    }
    pub fn complete(&self, prompt: &str, max_tokens: u16) -> Result<Completion, String> {
        let body = format!(r#"{{"model":"{}","prompt":"{}","max_tokens":{},"temperature":0}}"#, escape(&self.model), escape(prompt), max_tokens);
        let mut stream = TcpStream::connect((self.host.as_str(), self.port)).map_err(|e| format!("connect: {e}"))?;
        stream.set_read_timeout(Some(Duration::from_secs(60))).map_err(|e| format!("read timeout: {e}"))?;
        stream.set_write_timeout(Some(Duration::from_secs(10))).map_err(|e| format!("write timeout: {e}"))?;
        let request = format!("POST /v1/completions HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", self.host, body.len(), body);
        stream.write_all(request.as_bytes()).map_err(|e| format!("write: {e}"))?;
        let raw = read_limited(&mut stream)?;
        let payload = raw.split_once("\r\n\r\n").map(|(_, p)| p).ok_or("invalid HTTP response")?;
        if !raw.starts_with("HTTP/1.1 200") { return Err(format!("llama.cpp returned: {payload}")); }
        Ok(Completion { text: json_string_after(payload, "\"text\":").ok_or("completion text missing")?, total_tokens: json_number_after(payload, "\"total_tokens\":") })
    }
}

fn read_limited(reader: &mut impl Read) -> Result<String, String> {
    let mut bytes = Vec::new();
    reader.take(MAX_HTTP_RESPONSE_BYTES + 1).read_to_end(&mut bytes).map_err(|e| format!("read: {e}"))?;
    if bytes.len() as u64 > MAX_HTTP_RESPONSE_BYTES { return Err("llama.cpp response exceeds 8 MiB limit".into()); }
    String::from_utf8(bytes).map_err(|_| "llama.cpp response is not valid UTF-8".into())
}
fn escape(value: &str) -> String { value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r") }
fn json_string_after(input: &str, key: &str) -> Option<String> { let rest = input.split_once(key)?.1.trim_start(); let content = rest.strip_prefix('"')?; let mut out = String::new(); let mut escaped = false; for c in content.chars() { if escaped { out.push(match c { 'n' => '\n', 'r' => '\r', 't' => '\t', '\\' => '\\', '"' => '"', other => other }); escaped = false; } else if c == '\\' { escaped = true; } else if c == '"' { return Some(out); } else { out.push(c); } } None }
fn json_number_after(input: &str, key: &str) -> Option<u64> { input.split_once(key)?.1.trim_start().chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok() }

#[cfg(test)] mod tests { use super::*; #[test] fn parses_completion() { let p = r#"{"choices":[{"text":"ok\n"}],"usage":{"total_tokens":12}}"#; assert_eq!(json_string_after(p, "\"text\":"), Some("ok\n".into())); assert_eq!(json_number_after(p, "\"total_tokens\":"), Some(12)); } #[test] fn rejects_oversized_response() { let binding = vec![b'x'; (MAX_HTTP_RESPONSE_BYTES + 1) as usize]; let mut bytes = binding.as_slice(); assert!(read_limited(&mut bytes).is_err()); } }
