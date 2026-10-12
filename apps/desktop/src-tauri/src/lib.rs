use op15_adaptive_harness::{
    Budget, BudgetUse, LlamaCppClient, Task,
    interaction::assess,
    loop_runner::TaskLoop,
    memory_store::{MemoryKind, MemoryStore, Verification},
    reasoning_router::route,
    specialists::{guidance as specialist_guidance, select as select_specialists},
    verifier::{CalculatorVerifier, Verdict, VerificationRequest, Verifier},
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};
use tauri::Manager;

const MEMORY_HOST: &str = "127.0.0.1";
const MEMORY_PORT: u16 = 9749;
const MAX_TOOL_FILE_BYTES: u64 = 1024 * 1024;

struct UnverifiedVerifier;
impl Verifier for UnverifiedVerifier {
    fn verify(&self, _: &VerificationRequest) -> Verdict {
        Verdict {
            passed: false,
            oracle: "no_oracle_configured".into(),
            artifact: String::new(),
            verified_claim: None,
            summary: "No independent oracle is configured for desktop chat.".into(),
        }
    }
}

#[derive(Serialize)]
struct HarnessStatus {
    controller_ready: bool,
    llama_endpoint: String,
    llama_reachable: bool,
    verification_rule: String,
    voice_state: String,
}
#[derive(Serialize)]
struct TaskResult {
    decision: String,
    observation: String,
    model_error: Option<String>,
    trace_events: usize,
    independently_verified: bool,
    elapsed_ms: u128,
    generation_elapsed_ms: Option<u128>,
    completion_tokens: Option<u64>,
    generation_tokens_per_second: Option<f64>,
    server_prompt_ms: Option<f64>,
    server_predicted_per_second: Option<f64>,
    interaction_mode: String,
    reasoning_lenses: String,
    specialists: String,
}
#[derive(Serialize)]
struct LauncherStatus {
    bundled: bool,
    file_exists: bool,
    llama_reachable: bool,
    message: String,
}
#[derive(Serialize)]
struct MemoryStatus {
    reachable: bool,
    dashboard_url: String,
    message: String,
    rss_mb: Option<String>,
    indexed_projects: Option<String>,
}
#[derive(Serialize)]
struct ProfilerStatus {
    metrics_available: bool,
    metrics_message: String,
    gpu_name: Option<String>,
    gpu_utilization_percent: Option<String>,
    gpu_vram_used_mb: Option<String>,
    gpu_vram_total_mb: Option<String>,
}
#[derive(Serialize)]
struct OracleReport {
    kind: String,
    status: String,
    independently_verified: bool,
    artifact: String,
    summary: String,
}
#[derive(Serialize)]
struct ProjectTestReport {
    passed: bool,
    status: String,
    summary: String,
    output: String,
}
#[derive(Serialize)]
struct FileEntry {
    name: String,
    relative_path: String,
    is_directory: bool,
    size_bytes: Option<u64>,
}
#[derive(Serialize)]
struct LongTermMemoryView {
    id: String,
    kind: String,
    verification: String,
    observed_at_ms: u128,
    content: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct DesktopConfig {
    gguf_path: String,
    server_path: String,
    model_name: String,
    model_folder: String,
    context_size: u32,
    gpu_layers: i32,
    port: u16,
    threads: u16,
    parallel: u8,
    batch_size: u16,
    ubatch_size: u16,
    cache_type: String,
    flash_attention: bool,
    mtp_enabled: bool,
    mtp_n_max: u8,
    server_metrics: bool,
}
impl DesktopConfig {
    fn normalized(mut self) -> Self {
        if self.model_name.trim().is_empty() {
            self.model_name = "local".into();
        }
        if self.context_size == 0 {
            self.context_size = 32_768;
        }
        if self.port == 0 {
            self.port = 8080;
        }
        if self.threads == 0 {
            self.threads = 8;
        }
        if self.parallel == 0 {
            self.parallel = 1;
        }
        if self.batch_size == 0 {
            self.batch_size = 2_048;
        }
        if self.ubatch_size == 0 {
            self.ubatch_size = 512;
        }
        if self.cache_type.trim().is_empty() {
            self.cache_type = "q8_0".into();
        }
        self
    }
}

#[tauri::command]
fn harness_status(port: Option<u16>) -> HarnessStatus {
    let port = port.unwrap_or(8080);
    let mut client = LlamaCppClient::local("local");
    client.port = port;
    HarnessStatus { controller_ready: true, llama_endpoint: format!("http://127.0.0.1:{port}/v1/chat/completions"), llama_reachable: client.is_reachable(), verification_rule: "Une réponse reste non vérifiée tant qu'un oracle indépendant n'a pas validé la même affirmation.".into(), voice_state: "Aucun adaptateur STT/TTS local n'est configuré".into() }
}
fn tcp_reachable(host: &str, port: u16) -> bool {
    (host, port)
        .to_socket_addrs()
        .ok()
        .and_then(|mut a| a.next())
        .is_some_and(|a| TcpStream::connect_timeout(&a, Duration::from_millis(700)).is_ok())
}
fn http_request(
    method: &str,
    host: &str,
    port: u16,
    path: &str,
    body: Option<&str>,
) -> Result<String, String> {
    let mut stream = TcpStream::connect((host, port)).map_err(|e| format!("connexion: {e}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(4)))
        .map_err(|e| e.to_string())?;
    let payload = body.unwrap_or("");
    let req = format!(
        "{method} {path} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
        payload.len()
    );
    stream
        .write_all(req.as_bytes())
        .map_err(|e| format!("écriture: {e}"))?;
    let mut raw = String::new();
    stream
        .read_to_string(&mut raw)
        .map_err(|e| format!("lecture: {e}"))?;
    let (head, response) = raw.split_once("\r\n\r\n").ok_or("réponse HTTP invalide")?;
    if !head.contains(" 200 ") && !head.contains(" 201 ") && !head.contains(" 202 ") {
        return Err(format!("service mémoire: {response}"));
    }
    Ok(response.to_owned())
}
fn json_field(body: &str, key: &str) -> Option<String> {
    let value = body.split_once(&format!("\"{key}\":"))?.1.trim_start();
    if let Some(rest) = value.strip_prefix('"') {
        Some(rest.split('"').next()?.to_owned())
    } else {
        Some(value.split([',', '}', ']']).next()?.trim().to_owned())
    }
}
#[tauri::command]
fn memory_status() -> MemoryStatus {
    let dashboard_url = format!("http://{MEMORY_HOST}:{MEMORY_PORT}/?tab=stats");
    match http_request("GET", MEMORY_HOST, MEMORY_PORT, "/api/processes", None) {
        Ok(body) => MemoryStatus {
            reachable: true,
            dashboard_url,
            message: "Codebase Memory MCP répond sur le port 9749.".into(),
            rss_mb: json_field(&body, "self_rss_mb"),
            indexed_projects: http_request(
                "GET",
                MEMORY_HOST,
                MEMORY_PORT,
                "/api/index-status",
                None,
            )
            .ok()
            .map(|v| {
                if v.trim() == "[]" {
                    "0".into()
                } else {
                    "au moins 1".into()
                }
            }),
        },
        Err(_) => MemoryStatus {
            reachable: false,
            dashboard_url,
            message: "Codebase Memory MCP ne répond pas sur 127.0.0.1:9749.".into(),
            rss_mb: None,
            indexed_projects: None,
        },
    }
}
#[tauri::command]
fn start_memory_mcp(executable_path: String, port: Option<u16>) -> Result<String, String> {
    let executable = PathBuf::from(executable_path.trim());
    if !executable.is_file() {
        return Err(
            "L'exécutable Codebase Memory MCP est introuvable. Choisis son fichier .exe.".into(),
        );
    }
    let port = port.unwrap_or(MEMORY_PORT);
    if tcp_reachable(MEMORY_HOST, port) {
        return Ok(format!(
            "Codebase Memory MCP est déjà accessible sur le port {port}."
        ));
    }
    Command::new(&executable)
        .args(["--ui=true", &format!("--port={port}")])
        .spawn()
        .map_err(|e| format!("démarrage MCP: {e}"))?;
    Ok(format!(
        "Démarrage de Codebase Memory MCP demandé sur le port {port}."
    ))
}
fn nvidia_smi() -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let Ok(output) = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,utilization.gpu,memory.used,memory.total",
            "--format=csv,noheader,nounits",
        ])
        .output()
    else {
        return (None, None, None, None);
    };
    if !output.status.success() {
        return (None, None, None, None);
    }
    let rendered = String::from_utf8_lossy(&output.stdout);
    let line = rendered.lines().next().unwrap_or("");
    let parts = line.split(',').map(str::trim).collect::<Vec<_>>();
    if parts.len() != 4 {
        return (None, None, None, None);
    }
    (
        Some(parts[0].to_owned()),
        Some(parts[1].to_owned()),
        Some(parts[2].to_owned()),
        Some(parts[3].to_owned()),
    )
}
#[tauri::command]
fn profiler_status(port: Option<u16>) -> ProfilerStatus {
    let port = port.unwrap_or(8080);
    let (metrics_available, metrics_message) =
        match http_request("GET", "127.0.0.1", port, "/metrics", None) {
            Ok(_) => (
                true,
                format!("Métriques Prometheus accessibles sur le port {port}."),
            ),
            Err(error) if error.contains("501") || error.contains("Not Implemented") => (
                false,
                format!("Le serveur sur le port {port} ne fournit pas /metrics (HTTP 501)."),
            ),
            Err(_) => (
                false,
                format!(
                    "Aucune métrique compatible n'est exposée par le serveur sur le port {port}."
                ),
            ),
        };
    let (gpu_name, gpu_utilization_percent, gpu_vram_used_mb, gpu_vram_total_mb) = nvidia_smi();
    ProfilerStatus {
        metrics_available,
        metrics_message,
        gpu_name,
        gpu_utilization_percent,
        gpu_vram_used_mb,
        gpu_vram_total_mb,
    }
}
fn safe_relative_path(relative_path: &str) -> Result<PathBuf, String> {
    let candidate = PathBuf::from(relative_path.trim());
    if candidate.is_absolute()
        || candidate.components().any(|part| {
            matches!(
                part,
                std::path::Component::ParentDir
                    | std::path::Component::Prefix(_)
                    | std::path::Component::RootDir
            )
        })
    {
        return Err("Le chemin doit être relatif au dossier explicitement choisi.".into());
    }
    Ok(candidate)
}
fn selected_root(root_path: &str) -> Result<PathBuf, String> {
    let root = PathBuf::from(root_path.trim());
    if !root.is_dir() {
        return Err("Le dossier choisi n'existe pas.".into());
    }
    root.canonicalize()
        .map_err(|e| format!("dossier choisi: {e}"))
}
fn scoped_path(root_path: &str, relative_path: &str) -> Result<(PathBuf, PathBuf), String> {
    let root = selected_root(root_path)?;
    let target = root.join(safe_relative_path(relative_path)?);
    let parent = target.parent().ok_or("chemin invalide")?;
    if parent.exists() {
        let canonical_parent = parent
            .canonicalize()
            .map_err(|e| format!("dossier cible: {e}"))?;
        if !canonical_parent.starts_with(&root) {
            return Err("La cible sort du dossier explicitement choisi.".into());
        }
    }
    if target.exists() {
        let canonical_target = target.canonicalize().map_err(|e| format!("cible: {e}"))?;
        if !canonical_target.starts_with(&root) {
            return Err("La cible résolue sort du dossier explicitement choisi.".into());
        }
    }
    Ok((root, target))
}
#[tauri::command]
fn list_workspace_files(
    root_path: String,
    relative_path: String,
) -> Result<Vec<FileEntry>, String> {
    let (root, target) = scoped_path(&root_path, &relative_path)?;
    if !target.is_dir() {
        return Err("Le chemin demandé n'est pas un dossier.".into());
    }
    let mut entries = fs::read_dir(&target)
        .map_err(|e| format!("lecture dossier: {e}"))?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let path = entry.path();
            let relative_path = path
                .strip_prefix(&root)
                .ok()?
                .to_string_lossy()
                .replace('\\', "/");
            Some(FileEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                relative_path,
                is_directory: metadata.is_dir(),
                size_bytes: metadata.is_file().then_some(metadata.len()),
            })
        })
        .collect::<Vec<_>>();
    entries.sort_by(|a, b| {
        b.is_directory
            .cmp(&a.is_directory)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(entries)
}
#[tauri::command]
fn read_workspace_file(root_path: String, relative_path: String) -> Result<String, String> {
    let (_, target) = scoped_path(&root_path, &relative_path)?;
    let metadata = fs::metadata(&target).map_err(|e| format!("fichier: {e}"))?;
    if !metadata.is_file() {
        return Err("La cible n'est pas un fichier.".into());
    }
    if metadata.len() > MAX_TOOL_FILE_BYTES {
        return Err("Le fichier dépasse la limite de lecture de 1 Mo.".into());
    }
    fs::read_to_string(&target).map_err(|e| format!("lecture fichier: {e}"))
}
#[tauri::command]
fn write_workspace_file(
    root_path: String,
    relative_path: String,
    content: String,
) -> Result<String, String> {
    if content.len() as u64 > MAX_TOOL_FILE_BYTES {
        return Err("Le contenu dépasse la limite d'écriture de 1 Mo.".into());
    }
    let (_, target) = scoped_path(&root_path, &relative_path)?;
    let parent = target.parent().ok_or("chemin invalide")?;
    fs::create_dir_all(parent).map_err(|e| format!("création dossier: {e}"))?;
    if target.exists() {
        let backup = target.with_extension(format!(
            "{}bak",
            target
                .extension()
                .and_then(|v| v.to_str())
                .map(|v| format!("{v}."))
                .unwrap_or_default()
        ));
        fs::copy(&target, &backup).map_err(|e| format!("sauvegarde avant modification: {e}"))?;
    }
    fs::write(&target, content).map_err(|e| format!("écriture fichier: {e}"))?;
    Ok(format!("Fichier enregistré : {}", target.display()))
}
#[tauri::command]
fn open_workspace_folder(root_path: String, relative_path: String) -> Result<(), String> {
    let (_, target) = scoped_path(&root_path, &relative_path)?;
    if !target.is_dir() {
        return Err("La cible n'est pas un dossier.".into());
    }
    Command::new("explorer.exe")
        .arg(target)
        .spawn()
        .map_err(|e| format!("ouverture Explorateur: {e}"))?;
    Ok(())
}
fn url_encode(input: &str) -> String {
    input
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![(byte as char).to_string()]
            }
            b' ' => vec!["+".to_owned()],
            other => vec![format!("%{other:02X}")],
        })
        .collect()
}
#[tauri::command]
fn open_browser_url(url: String) -> Result<(), String> {
    let url = url.trim();
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("Seules les URL http(s) peuvent être ouvertes.".into());
    }
    Command::new("explorer.exe")
        .arg(url)
        .spawn()
        .map_err(|e| format!("ouverture navigateur: {e}"))?;
    Ok(())
}
#[tauri::command]
fn search_web(query: String) -> Result<(), String> {
    let query = query.trim();
    if query.is_empty() {
        return Err("Saisis une recherche web.".into());
    }
    open_browser_url(format!(
        "https://www.google.com/search?q={}",
        url_encode(query)
    ))
}
#[tauri::command]
fn verify_calculation(expression: String, response: String) -> OracleReport {
    let expression = expression.trim();
    if expression.is_empty() {
        return OracleReport {
            kind: "calculator".into(),
            status: "refusé".into(),
            independently_verified: false,
            artifact: String::new(),
            summary: "Saisis une expression arithmétique explicite avant de lancer le calculateur.".into(),
        };
    }
    let request = VerificationRequest {
        task_id: "desktop-calculation".into(),
        claim: format!("résultat numérique de {expression}"),
        response,
        artifacts: Vec::new(),
        criteria: vec!["numeric-answer".into()],
    };
    let verdict = CalculatorVerifier { expression: expression.into() }.verify(&request);
    OracleReport {
        kind: "calculateur indépendant".into(),
        status: if verdict.supports(&request) { "vérifié (portée numérique)" } else { "non confirmé" }.into(),
        independently_verified: verdict.supports(&request),
        artifact: verdict.artifact,
        summary: verdict.summary,
    }
}

#[tauri::command]
fn run_project_tests(root_path: String) -> Result<ProjectTestReport, String> {
    let root = PathBuf::from(root_path.trim());
    if !root.join("Cargo.toml").is_file() {
        return Err("Choisis un projet Rust contenant un Cargo.toml à sa racine.".into());
    }
    let target = std::env::temp_dir().join(format!(
        "adaptive-harness-test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_millis()
    ));
    let mut child = Command::new("cargo")
        .args(["test", "--quiet", "--target-dir"])
        .arg(&target)
        .current_dir(&root)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("démarrage de cargo test: {e}"))?;
    let deadline = Instant::now() + Duration::from_secs(120);
    while child.try_wait().map_err(|e| format!("attente de cargo test: {e}"))?.is_none() {
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(ProjectTestReport {
                passed: false,
                status: "expérience interrompue".into(),
                summary: "cargo test a dépassé la limite de 120 secondes et a été arrêté.".into(),
                output: String::new(),
            });
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let output = child.wait_with_output().map_err(|e| format!("lecture de cargo test: {e}"))?;
    let joined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let max = 6_000;
    let clipped = if joined.len() > max {
        format!("…{}", &joined[joined.len() - max..])
    } else {
        joined
    };
    Ok(ProjectTestReport {
        passed: output.status.success(),
        status: if output.status.success() { "tests passés" } else { "tests en échec" }.into(),
        summary: "Expérience locale exécutée. Elle atteste l’état des tests du projet, pas la vérité générale d’une réponse du modèle.".into(),
        output: clipped,
    })
}
fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
#[tauri::command]
fn memory_index_project(root_path: String, project_name: String) -> Result<String, String> {
    let root = PathBuf::from(root_path.trim());
    if !root.is_dir() {
        return Err("Le dossier à indexer n'existe pas.".into());
    }
    let body = format!(
        r#"{{"root_path":"{}","project_name":"{}"}}"#,
        escape_json(&root.display().to_string()),
        escape_json(project_name.trim())
    );
    http_request("POST", MEMORY_HOST, MEMORY_PORT, "/api/index", Some(&body))
}

fn config_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("dossier de configuration: {e}"))?;
    fs::create_dir_all(&dir).map_err(|e| format!("création de configuration: {e}"))?;
    Ok(dir.join("desktop-config.json"))
}
fn long_term_memory_store(app: &tauri::AppHandle) -> Result<MemoryStore, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("memory directory: {e}"))?;
    MemoryStore::open(dir.join("long-term-memory.tsv"))
}
fn memory_view(record: op15_adaptive_harness::memory_store::MemoryRecord) -> LongTermMemoryView {
    LongTermMemoryView {
        id: record.id,
        kind: format!("{:?}", record.kind),
        verification: format!("{:?}", record.verification),
        observed_at_ms: record.observed_at_ms,
        content: record.content,
    }
}
fn parse_memory_kind(kind: &str) -> Result<MemoryKind, String> {
    match kind {
        "episodic" => Ok(MemoryKind::Episodic),
        "semantic" => Ok(MemoryKind::Semantic),
        "procedural" => Ok(MemoryKind::Procedural),
        _ => Err("Memory kind must be episodic, semantic, or procedural.".into()),
    }
}
#[tauri::command]
fn list_long_term_memory(app: tauri::AppHandle) -> Result<Vec<LongTermMemoryView>, String> {
    Ok(long_term_memory_store(&app)?
        .list_active()?
        .into_iter()
        .map(memory_view)
        .collect())
}
#[tauri::command]
fn add_long_term_memory(
    app: tauri::AppHandle,
    kind: String,
    content: String,
) -> Result<LongTermMemoryView, String> {
    Ok(memory_view(long_term_memory_store(&app)?.add(
        parse_memory_kind(&kind)?,
        Verification::UserAsserted,
        &content,
        None,
    )?))
}
#[tauri::command]
fn record_reviewed_source(
    app: tauri::AppHandle,
    url: String,
    statement: String,
) -> Result<LongTermMemoryView, String> {
    let url = url.trim();
    let statement = statement.trim();
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("La source doit être une URL http(s).".into());
    }
    if statement.is_empty() {
        return Err("Décris précisément quelle affirmation la source étaye ou contredit.".into());
    }
    if statement.len() > 4_000 {
        return Err("La note de source dépasse 4 000 caractères.".into());
    }
    Ok(memory_view(long_term_memory_store(&app)?.add(
        MemoryKind::Semantic,
        Verification::SourceReviewed,
        &format!("Source examinée par l’utilisateur : {url}\nPortée déclarée : {statement}"),
        None,
    )?))
}
#[tauri::command]
fn forget_long_term_memory(app: tauri::AppHandle, id: String) -> Result<(), String> {
    long_term_memory_store(&app)?.retract(&id)
}
#[tauri::command]
fn load_desktop_config(app: tauri::AppHandle) -> DesktopConfig {
    let path = match config_path(&app) {
        Ok(path) => path,
        Err(_) => return DesktopConfig::default().normalized(),
    };
    fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str::<DesktopConfig>(&text).ok())
        .unwrap_or_default()
        .normalized()
}
#[tauri::command]
fn save_desktop_config(
    app: tauri::AppHandle,
    config: DesktopConfig,
) -> Result<DesktopConfig, String> {
    let config = config.normalized();
    let path = config_path(&app)?;
    let temp = path.with_extension("tmp");
    let content = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(&temp, content).map_err(|e| format!("écriture de configuration: {e}"))?;
    fs::rename(temp, path).map_err(|e| format!("finalisation de configuration: {e}"))?;
    Ok(config)
}
#[tauri::command]
fn list_gguf(folder: String) -> Result<Vec<String>, String> {
    let path = PathBuf::from(folder.trim());
    if !path.is_dir() {
        return Err("Le dossier des modèles n'existe pas.".into());
    }
    let mut models = fs::read_dir(&path)
        .map_err(|e| format!("lecture du dossier: {e}"))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("gguf"))
        })
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>();
    models.sort();
    Ok(models)
}

fn run_task_blocking(
    prompt: String,
    model: String,
    port: u16,
    long_term_context: String,
    recent_context: String,
) -> Result<TaskResult, String> {
    if prompt.trim().is_empty() {
        return Err("Saisis une demande avant de lancer le harnais.".into());
    }
    let assignments = select_specialists(&prompt);
    let specialists = assignments
        .iter()
        .map(|assignment| format!("{} : {}", assignment.specialist.label(), assignment.justification))
        .collect::<Vec<_>>()
        .join(" · ");
    let mut context_sections = Vec::new();
    if !recent_context.trim().is_empty() {
        context_sections.push(format!(
            "Historique récent (contexte de dialogue, non vérifié) :\n{recent_context}"
        ));
    }
    if !long_term_context.trim().is_empty() {
        context_sections.push(format!("Mémoire long-terme récupérée (provenance incluse ; ne pas la traiter comme une vérité sans nuance) :\n{long_term_context}"));
    }
    if !assignments.is_empty() {
        context_sections.push(format!(
            "Rôles spécialisés sélectionnés (orientations, pas des oracles) : {}",
            specialist_guidance(&assignments)
        ));
    }
    context_sections.push(format!("Demande actuelle : {prompt}"));
    let claim = context_sections.join("\n\n");
    let interaction = assess(&claim);
    let reasoning_plan = route(&prompt);
    let reasoning_lenses = reasoning_plan
        .selected
        .iter()
        .map(|selection| {
            format!(
                "{} : {}",
                selection.contract.lens.label(),
                selection.justification
            )
        })
        .collect::<Vec<_>>()
        .join(" · ");
    let interaction_mode = format!("{} · {reasoning_lenses}", interaction.mode.label());
    let task = Task {
        id: format!(
            "desktop-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_millis()
        ),
        claim,
        tool_available: true,
        confidence_milli: 700,
    };
    let mut client = LlamaCppClient::local(model);
    client.port = port;
    let started = Instant::now();
    let result = TaskLoop::run(
        &task,
        Budget {
            wall_ms: 30_000,
            tokens: 1_024,
            ram_mb: 2_048,
            tool_calls: 1,
        },
        BudgetUse {
            wall_ms: 1_000,
            tokens: 768,
            ram_mb: 32,
            tool_calls: 1,
        },
        &client,
        &UnverifiedVerifier,
        1,
    );
    let decision = if result.observation.is_empty() {
        format!("{:?}", result.decision)
    } else {
        "Réponse générée — non vérifiée".into()
    };
    Ok(TaskResult {
        decision,
        observation: result.observation,
        model_error: result.model_error,
        trace_events: result.trace.0.len(),
        independently_verified: false,
        elapsed_ms: started.elapsed().as_millis(),
        generation_elapsed_ms: result.generation_elapsed_ms,
        completion_tokens: result.completion_tokens,
        generation_tokens_per_second: result
            .completion_tokens
            .zip(result.generation_elapsed_ms)
            .and_then(|(tokens, elapsed)| {
                (elapsed > 0).then(|| tokens as f64 / (elapsed as f64 / 1000.0))
            }),
        server_prompt_ms: result.server_prompt_ms,
        server_predicted_per_second: result.server_predicted_per_second,
        interaction_mode,
        reasoning_lenses,
        specialists,
    })
}
#[tauri::command]
async fn run_task(
    app: tauri::AppHandle,
    prompt: String,
    recent_context: String,
    model: String,
    port: u16,
) -> Result<TaskResult, String> {
    let long_term_context = long_term_memory_store(&app)?
        .retrieve(&prompt, 5)?
        .into_iter()
        .map(|record| {
            format!(
                "[{} | {:?} | {:?}] {}",
                record.id, record.kind, record.verification, record.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    tauri::async_runtime::spawn_blocking(move || {
        run_task_blocking(prompt, model, port, long_term_context, recent_context)
    })
    .await
    .map_err(|e| format!("worker du harnais: {e}"))?
}

fn bundled_launcher_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .resource_dir()
        .map(|d| d.join("resources").join("llama_launcher.exe"))
        .map_err(|e| format!("résolution du lanceur: {e}"))
}
#[tauri::command]
fn launcher_status(app: tauri::AppHandle) -> LauncherStatus {
    let llama_reachable = LlamaCppClient::local("local").is_reachable();
    match bundled_launcher_path(&app) { Ok(path) if path.is_file() && llama_reachable => LauncherStatus { bundled: true, file_exists: true, llama_reachable, message: "Lanceur GGUF inclus ; llama.cpp répond sur le port 8080.".into() }, Ok(path) if path.is_file() => LauncherStatus { bundled: true, file_exists: true, llama_reachable, message: "Lanceur GGUF inclus. Configure un modèle puis démarre un serveur, ou utilise le lanceur.".into() }, Ok(path) => LauncherStatus { bundled: true, file_exists: false, llama_reachable, message: format!("Lanceur inclus introuvable : {}.", path.display()) }, Err(message) => LauncherStatus { bundled: true, file_exists: false, llama_reachable, message } }
}
#[tauri::command]
fn launch_llama_launcher(app: tauri::AppHandle) -> Result<u32, String> {
    if LlamaCppClient::local("local").is_reachable() {
        return Err("llama.cpp répond déjà sur le port 8080.".into());
    }
    let path = bundled_launcher_path(&app)?;
    if !path.is_file() {
        return Err(format!("Lanceur GGUF introuvable : {}", path.display()));
    }
    let working_directory = path
        .parent()
        .ok_or("Le lanceur n'a pas de dossier parent.")?
        .to_owned();
    Command::new(&path)
        .current_dir(working_directory)
        .spawn()
        .map(|child| child.id())
        .map_err(|e| format!("démarrage du lanceur: {e}"))
}
#[tauri::command]
fn start_configured_server(config: DesktopConfig) -> Result<u32, String> {
    let config = config.normalized();
    if tcp_reachable("127.0.0.1", config.port) {
        return Err(format!(
            "Un serveur répond déjà sur le port {}.",
            config.port
        ));
    }
    let server = PathBuf::from(&config.server_path);
    let model = PathBuf::from(&config.gguf_path);
    if !server.is_file() {
        return Err("Choisis le fichier llama-server.exe dans Paramètres.".into());
    }
    if !model.is_file()
        || !model
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("gguf"))
    {
        return Err("Choisis un fichier .gguf existant dans Modèles.".into());
    }
    let context = config.context_size.to_string();
    let port = config.port.to_string();
    let gpu_layers = config.gpu_layers.to_string();
    let threads = config.threads.to_string();
    let parallel = config.parallel.to_string();
    let batch_size = config.batch_size.to_string();
    let ubatch_size = config.ubatch_size.to_string();
    let mtp_n_max = config.mtp_n_max.to_string();
    let mut command = Command::new(&server);
    command.args([
        "-m",
        model.to_string_lossy().as_ref(),
        "--host",
        "127.0.0.1",
        "--port",
        &port,
        "-c",
        &context,
        "--batch-size",
        &batch_size,
        "--threads",
        &threads,
        "--parallel",
        &parallel,
        "-ub",
        &ubatch_size,
    ]);
    if config.gpu_layers >= 0 {
        command.args(["-ngl", &gpu_layers]);
    }
    if config.flash_attention {
        command.args(["--flash-attn", "on"]);
    }
    if config.cache_type != "off" {
        command.args([
            "--cache-type-k",
            &config.cache_type,
            "--cache-type-v",
            &config.cache_type,
        ]);
    }
    if config.mtp_enabled {
        command.args(["--spec-type", "draft-mtp", "--spec-draft-n-max", &mtp_n_max]);
    }
    if config.server_metrics {
        command.arg("--metrics");
    }
    // Keep model reasoning enabled, but ask llama.cpp to separate it from the
    // public chat content. The UI receives only `content`; hidden reasoning is
    // never mislabelled as a final response or independent evidence.
    command.args(["--reasoning", "auto", "--reasoning-format", "deepseek"]);
    command
        .current_dir(server.parent().unwrap_or(Path::new(".")))
        .spawn()
        .map(|child| child.id())
        .map_err(|e| format!("démarrage de llama-server: {e}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            harness_status,
            memory_status,
            start_memory_mcp,
            profiler_status,
            list_workspace_files,
            read_workspace_file,
            write_workspace_file,
            open_workspace_folder,
            open_browser_url,
            search_web,
            verify_calculation,
            run_project_tests,
            memory_index_project,
            list_long_term_memory,
            add_long_term_memory,
            record_reviewed_source,
            forget_long_term_memory,
            load_desktop_config,
            save_desktop_config,
            list_gguf,
            run_task,
            launcher_status,
            launch_llama_launcher,
            start_configured_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running Adaptive Harness desktop");
}
