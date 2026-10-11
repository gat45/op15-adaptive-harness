use op15_adaptive_harness::{
    loop_runner::TaskLoop,
    verifier::{Verdict, VerificationRequest, Verifier},
    Budget, BudgetUse, LlamaCppClient, Task,
};
use serde::Serialize;

struct UnverifiedVerifier;
impl Verifier for UnverifiedVerifier {
    fn verify(&self, _: &VerificationRequest) -> Verdict {
        Verdict { passed: false, oracle: "no_oracle_configured".into(), artifact: String::new(), verified_claim: None, summary: "No independent oracle is configured for desktop chat.".into() }
    }
}

#[derive(Serialize)]
struct HarnessStatus { controller_ready: bool, llama_endpoint: String, llama_reachable: bool, verification_rule: String, voice_state: String }
#[derive(Serialize)]
struct TaskResult { decision: String, observation: String, trace_events: usize, independently_verified: bool }

#[tauri::command]
fn harness_status() -> HarnessStatus {
    let client = LlamaCppClient::local("local");
    HarnessStatus { controller_ready: true, llama_endpoint: "http://127.0.0.1:8080/v1/completions".into(), llama_reachable: client.is_reachable(), verification_rule: "A verdict must bind the claim and response.".into(), voice_state: "No STT/TTS adapter installed".into() }
}

fn run_task_blocking(prompt: String, model: String) -> Result<TaskResult, String> {
    if prompt.trim().is_empty() { return Err("Enter a request before starting the harness.".into()); }
    let task = Task { id: format!("desktop-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e| e.to_string())?.as_millis()), claim: prompt, tool_available: true, confidence_milli: 700 };
    let client = LlamaCppClient::local(model);
    let result = TaskLoop::run(&task, Budget { wall_ms: 10_000, tokens: 256, ram_mb: 2_048, tool_calls: 2 }, BudgetUse { wall_ms: 1_000, tokens: 64, ram_mb: 32, tool_calls: 1 }, &client, &UnverifiedVerifier, 2);
    Ok(TaskResult { decision: format!("{:?}", result.decision), observation: result.observation, trace_events: result.trace.0.len(), independently_verified: false })
}

#[tauri::command]
async fn run_task(prompt: String, model: String) -> Result<TaskResult, String> {
    tauri::async_runtime::spawn_blocking(move || run_task_blocking(prompt, model)).await.map_err(|e| format!("harness worker failed: {e}"))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { tauri::Builder::default().plugin(tauri_plugin_opener::init()).invoke_handler(tauri::generate_handler![harness_status, run_task]).run(tauri::generate_context!()).expect("error while running Adaptive Harness desktop"); }
