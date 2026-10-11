use op15_adaptive_harness::{
    loop_runner::TaskLoop,
    verifier::{Verdict, VerificationRequest, Verifier},
    Budget, BudgetUse, LlamaCppClient, Task,
};
use serde::Serialize;
use std::{net::{TcpStream, ToSocketAddrs}, path::PathBuf, process::Command, time::Duration};
use tauri::Manager;

struct UnverifiedVerifier;
impl Verifier for UnverifiedVerifier {
    fn verify(&self, _: &VerificationRequest) -> Verdict {
        Verdict { passed: false, oracle: "no_oracle_configured".into(), artifact: String::new(), verified_claim: None, summary: "No independent oracle is configured for desktop chat.".into() }
    }
}

#[derive(Serialize)]
struct HarnessStatus { controller_ready: bool, llama_endpoint: String, llama_reachable: bool, verification_rule: String, voice_state: String }
#[derive(Serialize)]
struct TaskResult { decision: String, observation: String, model_error: Option<String>, trace_events: usize, independently_verified: bool }

#[derive(Serialize)]
struct LauncherStatus { bundled: bool, file_exists: bool, llama_reachable: bool, message: String }

#[derive(Serialize)]
struct MemoryStatus { reachable: bool, dashboard_url: String, message: String }

#[tauri::command]
fn harness_status() -> HarnessStatus {
    let client = LlamaCppClient::local("local");
    HarnessStatus { controller_ready: true, llama_endpoint: "http://127.0.0.1:8080/v1/completions".into(), llama_reachable: client.is_reachable(), verification_rule: "A verdict must bind the claim and response.".into(), voice_state: "No STT/TTS adapter installed".into() }
}

#[tauri::command]
fn memory_status() -> MemoryStatus {
    let reachable = ("127.0.0.1", 9749)
        .to_socket_addrs()
        .ok()
        .and_then(|mut addresses| addresses.next())
        .is_some_and(|address| TcpStream::connect_timeout(&address, Duration::from_millis(500)).is_ok());
    MemoryStatus {
        reachable,
        dashboard_url: "http://127.0.0.1:9749/?tab=stats".into(),
        message: if reachable { "Codebase Memory MCP is reachable.".into() } else { "Codebase Memory MCP is not reachable on port 9749.".into() },
    }
}

fn run_task_blocking(prompt: String, model: String) -> Result<TaskResult, String> {
    if prompt.trim().is_empty() { return Err("Enter a request before starting the harness.".into()); }
    let task = Task { id: format!("desktop-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e| e.to_string())?.as_millis()), claim: prompt, tool_available: true, confidence_milli: 700 };
    let client = LlamaCppClient::local(model);
    let result = TaskLoop::run(&task, Budget { wall_ms: 10_000, tokens: 256, ram_mb: 2_048, tool_calls: 1 }, BudgetUse { wall_ms: 1_000, tokens: 128, ram_mb: 32, tool_calls: 1 }, &client, &UnverifiedVerifier, 1);
    Ok(TaskResult { decision: format!("{:?}", result.decision), observation: result.observation, model_error: result.model_error, trace_events: result.trace.0.len(), independently_verified: false })
}

#[tauri::command]
async fn run_task(prompt: String, model: String) -> Result<TaskResult, String> {
    tauri::async_runtime::spawn_blocking(move || run_task_blocking(prompt, model)).await.map_err(|e| format!("harness worker failed: {e}"))?
}

fn bundled_launcher_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .resource_dir()
        .map(|directory| directory.join("resources").join("llama_launcher.exe"))
        .map_err(|error| format!("Unable to resolve bundled launcher: {error}"))
}

#[tauri::command]
fn launcher_status(app: tauri::AppHandle) -> LauncherStatus {
    let llama_reachable = LlamaCppClient::local("local").is_reachable();
    match bundled_launcher_path(&app) {
        Ok(path) if path.is_file() && llama_reachable => LauncherStatus { bundled: true, file_exists: true, llama_reachable, message: "GGUF launcher bundled; llama.cpp is reachable on port 8080.".into() },
        Ok(path) if path.is_file() => LauncherStatus { bundled: true, file_exists: true, llama_reachable, message: "GGUF launcher bundled, but llama.cpp is not reachable on port 8080.".into() },
        Ok(path) => LauncherStatus { bundled: true, file_exists: false, llama_reachable, message: format!("Bundled launcher not found at {}.", path.display()) },
        Err(message) => LauncherStatus { bundled: true, file_exists: false, llama_reachable, message },
    }
}

#[tauri::command]
fn launch_llama_launcher(app: tauri::AppHandle) -> Result<u32, String> {
    if LlamaCppClient::local("local").is_reachable() { return Err("llama.cpp is already reachable on port 8080; do not start a second server.".into()); }
    let path = bundled_launcher_path(&app)?;
    if !path.is_file() { return Err(format!("Bundled GGUF launcher is missing: {}", path.display())); }
    let working_directory = path
        .parent()
        .ok_or("Bundled launcher has no parent directory.")?
        .to_owned();
    Command::new(&path)
        .current_dir(working_directory)
        .spawn()
        .map(|child| child.id())
        .map_err(|error| format!("Unable to start bundled GGUF launcher: {error}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { tauri::Builder::default().plugin(tauri_plugin_opener::init()).invoke_handler(tauri::generate_handler![harness_status, memory_status, run_task, launcher_status, launch_llama_launcher]).run(tauri::generate_context!()).expect("error while running Adaptive Harness desktop"); }
