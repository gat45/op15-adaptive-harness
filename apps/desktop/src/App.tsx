import { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Status = { controller_ready: boolean; llama_endpoint: string; llama_reachable: boolean; verification_rule: string; voice_state: string };
type TaskResult = { decision: string; observation: string; trace_events: number; independently_verified: boolean };
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

export default function App() {
  const [status, setStatus] = useState<Status | null>(null);
  const [page, setPage] = useState<Page>("Chat");
  const [prompt, setPrompt] = useState("Explique l'etat du harnais en une phrase.");
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
  const chat = <><article><p className="role">UTILISATEUR</p><p>{prompt}</p>{result && <><p className="role">OBSERVATION LLAMA.CPP</p><pre>{result.observation || "Aucune sortie."}</pre></>}{error && <p className="error">{error}</p>}</article><form onSubmit={submit}><textarea aria-label="Demande" value={prompt} onChange={(e) => setPrompt(e.target.value)} /><button type="submit" disabled={running || !status?.llama_reachable}>{running ? "Execution..." : "Envoyer"}</button></form></>;
  const agent = <article><p className="role">DECISION ACTUELLE</p><dl><dt>Decision</dt><dd>{result?.decision ?? "aucune"}</dd><dt>Trace</dt><dd>{result ? `${result.trace_events} evenement(s)` : "aucune"}</dd><dt>Preuve</dt><dd>{result?.independently_verified ? "verifiee" : "non verifiee"}</dd></dl><p className="rule">{status?.verification_rule}</p></article>;
  return <main><aside><h1>ADAPTIVE<br/>HARNESS</h1><nav>{pages.map((item) => <button className={item === page ? "active" : ""} onClick={() => setPage(item)} key={item}>{item}</button>)}</nav></aside><section className="conversation"><header><div><strong>{page === "Chat" ? "Conversation locale" : page}</strong><small>{status?.llama_endpoint ?? "Connexion..."}</small></div>{page === "Chat" && <label>Modele <input value={model} onChange={(e) => setModel(e.target.value)} /></label>}</header>{page === "Chat" ? chat : page === "Agent" ? agent : <Unavailable page={page} />}</section><aside className="inspector"><h2>ETAT</h2><dl><dt>Controleur</dt><dd>{status?.controller_ready ? "pret" : "inconnu"}</dd><dt>llama.cpp</dt><dd>{status ? (status.llama_reachable ? "accessible" : "indisponible") : "verification..."}</dd><dt>Voix</dt><dd>{status?.voice_state ?? "inconnu"}</dd></dl><button onClick={refresh}>Actualiser</button></aside><footer>CPU N/A | GPU N/A | VRAM N/A | RAM N/A | tok/s N/A | TTFT N/A</footer></main>;
}
