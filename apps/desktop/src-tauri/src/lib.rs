use op15_adaptive_harness::{
    loop_runner::TaskLoop,
    verifier::FileExistsVerifier,
    Budget, BudgetUse, LlamaCppClient, Task,
};
use serde::Serialize;

#[derive(Serialize)]
struct HarnessStatus {
    controller_ready: bool,
    llama_endpoint: String,
    verification_rule: String,
    voice_state: String,
}

#[derive(Serialize)]
struct TaskResult {
    decision: String,
    observation: String,
    trace_events: usize,
    independently_verified: bool,
}

#[tauri::command]
fn harness_status() -> HarnessStatus {
    HarnessStatus {
        controller_ready: true,
        llama_endpoint: "http://127.0.0.1:8080/v1/completions".into(),
        verification_rule: "A verdict must bind the claim and response.".into(),
        voice_state: "No STT/TTS adapter installed".into(),
    }
}

#[tauri::command]
fn run_task(prompt: String, model: String) -> Result<TaskResult, String> {
    if prompt.trim().is_empty() {
        return Err("Enter a request before starting the harness.".into());
    }
    let task = Task {
        id: format!("desktop-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e| e.to_string())?.as_millis()),
        claim: prompt,
        tool_available: true,
        confidence_milli: 700,
    };
    let client = LlamaCppClient::local(model);
    let verifier = FileExistsVerifier { path: "Cargo.toml".into() };
    let result = TaskLoop::run(
        &task,
        Budget { wall_ms: 10_000, tokens: 256, ram_mb: 2_048, tool_calls: 2 },
        BudgetUse { wall_ms: 1_000, tokens: 64, ram_mb: 32, tool_calls: 1 },
        &client,
        &verifier,
        2,
    );
    Ok(TaskResult {
        decision: format!("{:?}", result.decision),
        observation: result.observation,
        trace_events: result.trace.0.len(),
        independently_verified: false,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![harness_status, run_task])
        .run(tauri::generate_context!())
        .expect("error while running Adaptive Harness desktop");
}
