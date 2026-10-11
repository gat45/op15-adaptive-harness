use std::{io::{Read, Write}, net::TcpStream, time::Duration};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion { pub text: String, pub total_tokens: Option<u64> }

#[derive(Debug, Clone)]
pub struct LlamaCppClient { pub host: String, pub port: u16, pub model: String }

impl LlamaCppClient {
    pub fn local(model: impl Into<String>) -> Self { Self { host: "127.0.0.1".into(), port: 8080, model: model.into() } }
    pub fn complete(&self, prompt: &str, max_tokens: u16) -> Result<Completion, String> {
        let body = format!(r#"{{"model":"{}","prompt":"{}","max_tokens":{},"temperature":0}}"#, escape(&self.model), escape(prompt), max_tokens);
        let mut stream = TcpStream::connect((self.host.as_str(), self.port)).map_err(|e| e.to_string())?;
        stream.set_read_timeout(Some(Duration::from_secs(60))).map_err(|e| e.to_string())?;
        let request = format!("POST /v1/completions HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", self.host, body.len(), body);
        stream.write_all(request.as_bytes()).map_err(|e| e.to_string())?;
        let mut raw = String::new(); stream.read_to_string(&mut raw).map_err(|e| e.to_string())?;
        let payload = raw.split_once("\r\n\r\n").map(|(_, p)| p).ok_or("invalid HTTP response")?;
        if !raw.starts_with("HTTP/1.1 200") { return Err(payload.to_owned()); }
        Ok(Completion { text: json_string_after(payload, "\"text\":").ok_or("completion text missing")?, total_tokens: json_number_after(payload, "\"total_tokens\":") })
    }
}

fn escape(value: &str) -> String { value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n") }
fn json_string_after(input: &str, key: &str) -> Option<String> { let rest = input.split_once(key)?.1.trim_start(); let content = rest.strip_prefix('"')?; let mut out = String::new(); let mut escaped = false; for c in content.chars() { if escaped { out.push(match c { 'n' => '\n', 'r' => '\r', 't' => '\t', other => other }); escaped = false; } else if c == '\\' { escaped = true; } else if c == '"' { return Some(out); } else { out.push(c); } } None }
fn json_number_after(input: &str, key: &str) -> Option<u64> { input.split_once(key)?.1.trim_start().chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok() }

#[cfg(test)] mod tests { use super::*; #[test] fn parses_completion() { let p = r#"{"choices":[{"text":"ok\n"}],"usage":{"total_tokens":12}}"#; assert_eq!(json_string_after(p, "\"text\":"), Some("ok\n".into())); assert_eq!(json_number_after(p, "\"total_tokens\":"), Some(12)); } }


