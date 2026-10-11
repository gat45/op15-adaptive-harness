import { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Status = { controller_ready: boolean; llama_endpoint: string; llama_reachable: boolean; verification_rule: string; voice_state: string };
type TaskResult = { decision: string; observation: string; model_error: string | null; trace_events: number; independently_verified: boolean };
type LauncherStatus = { bundled: boolean; file_exists: boolean; llama_reachable: boolean; message: string };
type MemoryStatus = { reachable: boolean; dashboard_url: string; message: string };
const pages = ["Chat", "Agent", "Memoire", "Outils MCP", "Modeles", "Profiler", "Benchmarks", "Parametres"] as const;
type Page = typeof pages[number];

function Unavailable({ page }: { page: Exclude<Page, "Chat" | "Agent"> }) {
  const notices: Record<typeof page, string> = {
    "Memoire": "Aucune memoire persistante n'est configuree.",
    "Outils MCP": "Aucun serveur MCP n'est configure dans cette application.",
    "Modeles": "Le modele est selectionne par le serveur llama.cpp local.",
    "Profiler": "Aucun adaptateur CPU, GPU ou NPU ne fournit de mesure.",
    "Benchmarks": "Aucun benchmark scelle n'a encore ete execute.",
    "Parametres": "Les moteurs voix et les permissions d'outils ne sont pas encore exposes ici."
  };
  return <article><p className="role">{page.toUpperCase()}</p><p>{notices[page]}</p><p className="rule">Etat affiche: non disponible, aucune valeur simulee.</p></article>;
}

function Launcher({ onStarted }: { onStarted: () => void }) {
  const [status, setStatus] = useState<LauncherStatus | null>(null);
  const [message, setMessage] = useState("");
  const inspect = () => invoke<LauncherStatus>("launcher_status").then(setStatus).catch((error) => setMessage(String(error)));
  const start = async () => {
    setMessage("");
    try { const pid = await invoke<number>("launch_llama_launcher"); setMessage(`Lanceur GGUF demarre (PID ${pid}). Verification de llama.cpp en cours.`); window.setTimeout(() => { inspect(); onStarted(); }, 1200); }
    catch (error) { setMessage(String(error)); }
  };
  useEffect(() => { inspect(); }, []);
  return <article><p className="role">LANCEUR GGUF INTEGRE</p><p>Le lanceur est livré avec l’application. Il ne démarre qu’après ton clic.</p><p><button onClick={inspect}>Verifier le lanceur</button><button onClick={start} disabled={!status?.file_exists || status.llama_reachable}>Demarrer le lanceur GGUF</button></p><p className="rule">{status?.message ?? "Verification du lanceur integre..."}</p>{message && <p className="error">{message}</p>}</article>;
}

function MemoryPanel() {
  const [status, setStatus] = useState<MemoryStatus | null>(null);
  const refresh = () => invoke<MemoryStatus>("memory_status").then(setStatus);
  useEffect(() => { refresh(); }, []);
  return <article><p className="role">CODEBASE MEMORY MCP</p><p>La mémoire de code reste un service local séparé, connecté à l’interface par son endpoint.</p><p className={status?.reachable ? "ok" : "error"}>{status?.reachable ? "Service accessible" : "Service indisponible"}</p><p><button onClick={refresh}>Actualiser l’état</button>{status?.reachable && <a href={status.dashboard_url} target="_blank" rel="noreferrer">Ouvrir le tableau de bord</a>}</p><p className="rule">{status?.message ?? "Vérification du service mémoire..."}</p></article>;
}

export default function App() {
  const [status, setStatus] = useState<Status | null>(null);
  const [page, setPage] = useState<Page>("Chat");
  const [prompt, setPrompt] = useState("Explique l’état du harnais en une phrase, en français, sans balise de raisonnement.");
  const [model, setModel] = useState("local");
  const [result, setResult] = useState<TaskResult | null>(null);
  const [error, setError] = useState("");
  const [running, setRunning] = useState(false);
  const refresh = () => invoke<Status>("harness_status").then(setStatus).catch((e) => setError(String(e)));
  useEffect(() => { refresh(); }, []);
  async function submit(event: FormEvent) {
    event.preventDefault(); setError(""); setRunning(true); setResult(null);
    try { setResult(await invoke<TaskResult>("run_task", { prompt, model })); }
    catch (e) { setError(String(e)); }
    finally { setRunning(false); }
  }
  const chat = <><article><p className="role">UTILISATEUR</p><p>{prompt}</p>{result?.model_error && <p className="error">Erreur llama.cpp : {result.model_error}</p>}{result?.observation && <><p className="role">REPONSE DU MODELE — NON VERIFIEE</p><pre>{result.observation}</pre></>}{result && !result.model_error && !result.observation && <p className="error">Le modèle n’a renvoyé aucune réponse publique.</p>}{error && <p className="error">{error}</p>}</article><form onSubmit={submit}><textarea aria-label="Demande" value={prompt} onChange={(e) => setPrompt(e.target.value)} /><button type="submit" disabled={running || !status?.llama_reachable}>{running ? "Execution..." : "Envoyer"}</button></form></>;
  const agent = <article><p className="role">DECISION ACTUELLE</p><dl><dt>Decision</dt><dd>{result?.decision ?? "aucune"}</dd><dt>Trace</dt><dd>{result ? `${result.trace_events} evenement(s)` : "aucune"}</dd><dt>Preuve</dt><dd>{result?.independently_verified ? "verifiee" : "non verifiee"}</dd></dl><p className="rule">{status?.verification_rule}</p></article>;
  return <main><aside><h1>ADAPTIVE<br/>HARNESS</h1><nav>{pages.map((item) => <button className={item === page ? "active" : ""} onClick={() => setPage(item)} key={item}>{item}</button>)}</nav></aside><section className="conversation"><header><div><strong>{page === "Chat" ? "Conversation locale" : page}</strong><small>{status?.llama_endpoint ?? "Connexion..."}</small></div>{page === "Chat" && <label>Modele <input value={model} onChange={(e) => setModel(e.target.value)} /></label>}</header>{page === "Chat" ? chat : page === "Agent" ? agent : page === "Memoire" ? <MemoryPanel /> : page === "Modeles" ? <Launcher onStarted={refresh} /> : <Unavailable page={page} />}</section><aside className="inspector"><h2>ETAT</h2><dl><dt>Controleur</dt><dd>{status?.controller_ready ? "pret" : "inconnu"}</dd><dt>llama.cpp</dt><dd>{status ? (status.llama_reachable ? "accessible" : "indisponible") : "verification..."}</dd><dt>Voix</dt><dd>{status?.voice_state ?? "inconnu"}</dd></dl><button onClick={refresh}>Actualiser</button></aside><footer>CPU N/A | GPU N/A | VRAM N/A | tok/s N/A | TTFT N/A</footer></main>;
}
