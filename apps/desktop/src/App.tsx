import { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Status = { controller_ready: boolean; llama_endpoint: string; verification_rule: string; voice_state: string };
type TaskResult = { decision: string; observation: string; trace_events: number; independently_verified: boolean };

export default function App() {
  const [status, setStatus] = useState<Status | null>(null);
  const [prompt, setPrompt] = useState("Explique l'etat du harnais en une phrase.");
  const [model, setModel] = useState("local");
  const [result, setResult] = useState<TaskResult | null>(null);
  const [error, setError] = useState("");
  const [running, setRunning] = useState(false);

  useEffect(() => { invoke<Status>("harness_status").then(setStatus).catch((e) => setError(String(e))); }, []);
  async function submit(event: FormEvent) {
    event.preventDefault(); setError(""); setRunning(true); setResult(null);
    try { setResult(await invoke<TaskResult>("run_task", { prompt, model })); }
    catch (e) { setError(String(e)); }
    finally { setRunning(false); }
  }

  return <main>
    <aside><h1>ADAPTIVE<br/>HARNESS</h1><nav>{["Chat", "Agent", "Memoire", "Outils MCP", "Modeles", "Profiler", "Benchmarks", "Parametres"].map((x, i) => <button className={i === 0 ? "active" : ""} key={x}>{x}</button>)}</nav></aside>
    <section className="conversation"><header><div><strong>Conversation locale</strong><small>{status?.llama_endpoint ?? "Connexion..."}</small></div><label>Modele <input value={model} onChange={(e) => setModel(e.target.value)} /></label></header>
      <article><p className="role">UTILISATEUR</p><p>{prompt}</p>{result && <><p className="role">OBSERVATION LLAMA.CPP</p><pre>{result.observation || "Aucune sortie."}</pre></>}{error && <p className="error">{error}</p>}</article>
      <form onSubmit={submit}><textarea aria-label="Demande" value={prompt} onChange={(e) => setPrompt(e.target.value)} /><button type="submit" disabled={running}>{running ? "Execution..." : "Envoyer"}</button></form>
    </section>
    <aside className="inspector"><h2>INSPECTEUR</h2><dl><dt>Controleur</dt><dd>{status?.controller_ready ? "pret" : "inconnu"}</dd><dt>Decision</dt><dd>{result?.decision ?? "aucune"}</dd><dt>Trace</dt><dd>{result ? `${result.trace_events} evenement(s)` : "aucune"}</dd><dt>Preuve</dt><dd>{result?.independently_verified ? "verifiee" : "non verifiee"}</dd><dt>Voix</dt><dd>{status?.voice_state ?? "inconnu"}</dd></dl><p className="rule">{status?.verification_rule}</p></aside>
    <footer>CPU N/A | GPU N/A | VRAM N/A | RAM N/A | tok/s N/A | TTFT N/A</footer>
  </main>;
}
