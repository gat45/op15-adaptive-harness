import { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

type Status = {
  controller_ready: boolean;
  llama_endpoint: string;
  llama_reachable: boolean;
  verification_rule: string;
  voice_state: string;
};
type TaskResult = {
  decision: string;
  observation: string;
  model_error: string | null;
  trace_events: number;
  independently_verified: boolean;
  elapsed_ms: number;
  generation_elapsed_ms: number | null;
  completion_tokens: number | null;
  prompt_tokens: number | null;
  cached_tokens: number | null;
  generation_tokens_per_second: number | null;
  server_prompt_ms: number | null;
  server_prompt_per_second: number | null;
  server_predicted_per_second: number | null;
  interaction_mode: string;
  reasoning_lenses: string;
  specialists: string;
  trace_summary: string;
};
type LauncherStatus = {
  file_exists: boolean;
  llama_reachable: boolean;
  message: string;
};
type MemoryStatus = {
  reachable: boolean;
  dashboard_url: string;
  message: string;
  rss_mb: string | null;
  indexed_projects: string | null;
};
type ProfilerStatus = {
  metrics_available: boolean;
  metrics_message: string;
  gpu_name: string | null;
  gpu_utilization_percent: string | null;
  gpu_vram_used_mb: string | null;
  gpu_vram_total_mb: string | null;
  llama_server_pid: number | null;
  llama_server_vram_mb: string | null;
};
type OracleReport = {
  kind: string;
  status: string;
  independently_verified: boolean;
  artifact: string;
  summary: string;
};
type ProjectTestReport = {
  passed: boolean;
  status: string;
  summary: string;
  output: string;
};
type FileEntry = {
  name: string;
  relative_path: string;
  is_directory: boolean;
  size_bytes: number | null;
};
type Config = {
  gguf_path: string;
  server_path: string;
  model_name: string;
  model_folder: string;
  context_size: number;
  gpu_layers: number;
  port: number;
  threads: number;
  parallel: number;
  batch_size: number;
  ubatch_size: number;
  cache_type: string;
  flash_attention: boolean;
  mtp_enabled: boolean;
  mtp_n_max: number;
  server_metrics: boolean;
};
type Turn = { role: string; text: string };
type LongTermMemory = {
  id: string;
  kind: string;
  verification: string;
  observed_at_ms: number;
  content: string;
};
type PersonaProfile = {
  name: string;
  language: string;
  tone: string;
  principles: string[];
  boundaries: string[];
};
const pages = [
  "Chat",
  "Agent",
  "Memoire",
  "Outils MCP",
  "Modeles",
  "Profiler",
  "Benchmarks",
  "Parametres",
] as const;
type Page = (typeof pages)[number];
const blank: Config = {
  gguf_path: "",
  server_path: "",
  model_name: "local",
  model_folder: "",
  context_size: 32768,
  gpu_layers: 999,
  // llama.cpp écoute classiquement sur 8080. Le profil GTX 1080 peut ensuite
  // choisir 18181 pour exposer ses métriques.
  port: 8080,
  threads: 8,
  parallel: 1,
  batch_size: 2048,
  ubatch_size: 512,
  cache_type: "q8_0",
  flash_attention: true,
  mtp_enabled: true,
  mtp_n_max: 2,
  server_metrics: true,
};
const memoryKey = "adaptive-harness-chat-v1";
function Notice({
  children,
  kind = "info",
}: {
  children: React.ReactNode;
  kind?: "info" | "ok" | "error";
}) {
  return <p className={`notice ${kind}`}>{children}</p>;
}

function Models({
  config,
  setConfig,
  refresh,
}: {
  config: Config;
  setConfig: (x: Config) => void;
  refresh: () => void;
}) {
  const [s, setS] = useState<LauncherStatus | null>(null),
    [models, setModels] = useState<string[]>([]),
    [msg, setMsg] = useState("");
  const inspect = () =>
    invoke<LauncherStatus>("launcher_status")
      .then(setS)
      .catch((e) => setMsg(String(e)));
  const save = async () => {
    const x = await invoke<Config>("save_desktop_config", { config });
    setConfig(x);
    return x;
  };
  const chooseFolder = async () => {
    const x = await open({
      directory: true,
      multiple: false,
      title: "Choisir le dossier GGUF",
    });
    if (typeof x === "string") setConfig({ ...config, model_folder: x });
  };
  const chooseModel = async () => {
    const x = await open({
      multiple: false,
      title: "Choisir le modèle GGUF",
      filters: [{ name: "GGUF", extensions: ["gguf"] }],
    });
    if (typeof x === "string") setConfig({ ...config, gguf_path: x });
  };
  const scan = async () => {
    try {
      const x = await invoke<string[]>("list_gguf", {
        folder: config.model_folder,
      });
      setModels(x);
      setMsg(
        x.length ? `${x.length} modèle(s) trouvé(s).` : "Aucun GGUF trouvé.",
      );
    } catch (e) {
      setMsg(String(e));
    }
  };
  const start = async () => {
    if (!config.server_path.trim() || !config.gguf_path.trim()) {
      setMsg("Choisis d’abord llama-server.exe et un modèle GGUF.");
      return;
    }
    try {
      const x = await save(),
        pid = await invoke<number>("start_configured_server", { config: x });
      setMsg(`Serveur démarré (PID ${pid}).`);
      setTimeout(() => {
        inspect();
        refresh();
      }, 1400);
    } catch (e) {
      setMsg(String(e));
    }
  };
  useEffect(() => {
    inspect();
  }, []);
  return (
    <article>
      <p className="role">MODELES LOCAUX</p>
      <h2>Serveur et modèle GGUF</h2>
      <label>
        Dossier GGUF
        <input
          value={config.model_folder}
          onChange={(e) =>
            setConfig({ ...config, model_folder: e.target.value })
          }
        />
      </label>
      <div className="actions">
        <button onClick={chooseFolder}>Parcourir le dossier</button>
        <button onClick={scan}>Rechercher</button>
      </div>
      {models.length > 0 && (
        <label>
          Modèles détectés
          <select
            value={config.gguf_path}
            onChange={(e) =>
              setConfig({ ...config, gguf_path: e.target.value })
            }
          >
            <option value="">Choisir</option>
            {models.map((x) => (
              <option key={x} value={x}>
                {x}
              </option>
            ))}
          </select>
        </label>
      )}
      <label>
        GGUF sélectionné
        <input
          value={config.gguf_path}
          onChange={(e) => setConfig({ ...config, gguf_path: e.target.value })}
        />
      </label>
      <div className="actions">
        <button onClick={chooseModel}>Parcourir un GGUF</button>
        <button
          onClick={() =>
            save().then(() => setMsg("Configuration enregistrée."))
          }
        >
          Enregistrer
        </button>
        <button
          className="primary"
          onClick={start}
          disabled={s?.llama_reachable || !config.server_path || !config.gguf_path}
        >
          Démarrer le serveur
        </button>
      </div>
      {!config.server_path || !config.gguf_path ? (
        <Notice>
          Configuration incomplète : sélectionne le moteur llama.cpp et le GGUF
          avant de démarrer.
        </Notice>
      ) : null}
      <Notice kind={s?.llama_reachable ? "ok" : "info"}>
        {msg || s?.message || "Vérification…"}
      </Notice>
    </article>
  );
}

function Memory({ tools }: { tools?: boolean }) {
  const [s, setS] = useState<MemoryStatus | null>(null),
    [root, setRoot] = useState("D:\\op15-adaptive-harness"),
    [name, setName] = useState("op15-adaptive-harness"),
    [mcpPath, setMcpPath] = useState(
      "F:\\soos\\plugins\\codebase-memory-mcp.exe",
    ),
    [msg, setMsg] = useState(""),
    [records, setRecords] = useState<LongTermMemory[]>([]),
    [content, setContent] = useState(""),
    [kind, setKind] = useState("semantic");
  const refresh = () =>
    invoke<MemoryStatus>("memory_status")
      .then(setS)
      .catch((e) => setMsg(String(e)));
  const refreshLongTerm = () =>
    invoke<LongTermMemory[]>("list_long_term_memory")
      .then(setRecords)
      .catch((e) => setMsg(String(e)));
  const index = async () => {
    try {
      setMsg(
        await invoke<string>("memory_index_project", {
          rootPath: root,
          projectName: name,
        }),
      );
      refresh();
    } catch (e) {
      setMsg(String(e));
    }
  };
  const startMcp = async () => {
    try {
      setMsg(
        await invoke<string>("start_memory_mcp", {
          executablePath: mcpPath,
          port: 9749,
        }),
      );
      setTimeout(refresh, 1600);
    } catch (e) {
      setMsg(String(e));
    }
  };
  const chooseMcp = async () => {
    const value = await open({
      multiple: false,
      title: "Choisir Codebase Memory MCP",
      filters: [{ name: "Exécutable", extensions: ["exe"] }],
    });
    if (typeof value === "string") setMcpPath(value);
  };
  const add = async () => {
    try {
      await invoke("add_long_term_memory", { kind, content });
      setContent("");
      setMsg("Souvenir ajouté avec provenance « déclaré par l’utilisateur ».");
      refreshLongTerm();
    } catch (e) {
      setMsg(String(e));
    }
  };
  const forget = async (id: string) => {
    try {
      await invoke("forget_long_term_memory", { id });
      setMsg(
        "Souvenir retiré : le journal conserve le tombstone pour l’audit.",
      );
      refreshLongTerm();
    } catch (e) {
      setMsg(String(e));
    }
  };
  useEffect(() => {
    refresh();
    refreshLongTerm();
  }, []);
  if (tools)
    return (
      <article>
        <p className="role">OUTIL MCP CONNECTE</p>
        <h2>Codebase Memory MCP</h2>
        <dl className="metrics">
          <dt>Connexion</dt>
          <dd className={s?.reachable ? "ok-text" : "error-text"}>
            {s?.reachable ? "accessible" : "indisponible"}
          </dd>
          <dt>RAM service</dt>
          <dd>{s?.rss_mb ? `${s.rss_mb} Mo` : "non remontée"}</dd>
          <dt>Projets indexés</dt>
          <dd>{s?.indexed_projects ?? "non remonté"}</dd>
        </dl>
        <div className="actions">
          <button onClick={refresh}>Actualiser</button>
          {s?.reachable && (
            <a href={s.dashboard_url} target="_blank" rel="noreferrer">
              Tableau de bord
            </a>
          )}
        </div>
        {!s?.reachable && (
          <fieldset>
            <legend>Démarrer le service local</legend>
            <label>
              Codebase Memory MCP
              <input
                value={mcpPath}
                onChange={(e) => setMcpPath(e.target.value)}
              />
            </label>
            <div className="actions">
              <button onClick={chooseMcp}>Parcourir</button>
              <button className="primary" onClick={startMcp}>
                Démarrer MCP
              </button>
            </div>
          </fieldset>
        )}
        <fieldset disabled={!s?.reachable}>
          <legend>Indexer un projet</legend>
          <label>
            Dossier
            <input value={root} onChange={(e) => setRoot(e.target.value)} />
          </label>
          <label>
            Nom
            <input value={name} onChange={(e) => setName(e.target.value)} />
          </label>
          <button onClick={index}>Indexer</button>
        </fieldset>
        <Notice>{msg || s?.message || "Vérification…"}</Notice>
      </article>
    );
  return (
    <article>
      <p className="role">MEMOIRE LONGUE DUREE</p>
      <h2>Souvenirs explicites</h2>
      <Notice>
        Seuls les souvenirs ajoutés ici persistent et peuvent être récupérés
        dans le chat. Aucun message n’est mémorisé automatiquement.
      </Notice>
      <label>
        Type
        <select value={kind} onChange={(e) => setKind(e.target.value)}>
          <option value="semantic">Préférence ou fait déclaré</option>
          <option value="episodic">Épisode</option>
          <option value="procedural">Procédure</option>
        </select>
      </label>
      <label>
        Souvenir
        <textarea
          value={content}
          placeholder="Ex. L’utilisateur préfère des réponses en français."
          onChange={(e) => setContent(e.target.value)}
        />
      </label>
      <button className="primary" onClick={add} disabled={!content.trim()}>
        Mémoriser explicitement
      </button>
      <div className="memory-list">
        {records.length === 0 ? (
          <p className="rule">Aucun souvenir persistant.</p>
        ) : (
          records.map((record) => (
            <div key={record.id} className="turn assistant">
              <p className="role">
                {record.kind} · {record.verification}
              </p>
              <pre>{record.content}</pre>
              <button onClick={() => forget(record.id)}>Oublier</button>
            </div>
          ))
        )}
      </div>
      {msg && <Notice>{msg}</Notice>}
    </article>
  );
}

function Settings({
  config,
  setConfig,
}: {
  config: Config;
  setConfig: (x: Config) => void;
}) {
  const [msg, setMsg] = useState("");
  const browse = async () => {
    const x = await open({
      multiple: false,
      title: "Choisir llama-server.exe",
      filters: [{ name: "Exécutable", extensions: ["exe"] }],
    });
    if (typeof x === "string") setConfig({ ...config, server_path: x });
  };
  const preset = () =>
    setConfig({
      ...config,
      gguf_path: "D:\\qwen9b_IQ4NL_owIQ4NL.gguf",
      server_path: "F:\\lama-tensorRT 1050-5070\\llama-server.exe",
      model_folder: "D:\\",
      model_name: "qwen9b_IQ4NL_owIQ4NL.gguf",
      port: 18181,
      context_size: 32768,
      gpu_layers: 999,
      threads: 8,
      parallel: 1,
      batch_size: 2048,
      ubatch_size: 512,
      cache_type: "q8_0",
      flash_attention: true,
      mtp_enabled: true,
      mtp_n_max: 2,
      server_metrics: true,
    });
  const save = async () => {
    try {
      setConfig(await invoke<Config>("save_desktop_config", { config }));
      setMsg("Configuration enregistrée.");
    } catch (e) {
      setMsg(String(e));
    }
  };
  return (
    <article>
      <p className="role">PARAMETRES D’EXECUTION</p>
      <button onClick={preset}>Appliquer le profil GTX 1080 validé</button>
      <label>
        llama-server.exe
        <input
          value={config.server_path}
          onChange={(e) =>
            setConfig({ ...config, server_path: e.target.value })
          }
        />
        <button onClick={browse}>Parcourir llama-server.exe</button>
      </label>
      <div className="settings-grid">
        <label>
          Port
          <input
            type="number"
            value={config.port}
            onChange={(e) =>
              setConfig({ ...config, port: Number(e.target.value) })
            }
          />
        </label>
        <label>
          Contexte
          <input
            type="number"
            value={config.context_size}
            onChange={(e) =>
              setConfig({ ...config, context_size: Number(e.target.value) })
            }
          />
        </label>
        <label>
          Couches GPU
          <input
            type="number"
            value={config.gpu_layers}
            onChange={(e) =>
              setConfig({ ...config, gpu_layers: Number(e.target.value) })
            }
          />
        </label>
      </div>
      <p className="rule">
        KV {config.cache_type}, Flash Attention{" "}
        {config.flash_attention ? "on" : "off"}, MTP{" "}
        {config.mtp_enabled ? `on n_max=${config.mtp_n_max}` : "off"}.
      </p>
      <button className="primary" onClick={save}>
        Enregistrer
      </button>
      {msg && <Notice kind="ok">{msg}</Notice>}
    </article>
  );
}

function PersonaSettings({
  persona,
  setPersona,
}: {
  persona: PersonaProfile | null;
  setPersona: (value: PersonaProfile) => void;
}) {
  const [message, setMessage] = useState("");
  if (!persona) return <Notice>Chargement de l’identité conversationnelle…</Notice>;
  const update = (key: keyof PersonaProfile, value: string) => {
    if (key === "principles" || key === "boundaries") {
      setPersona({ ...persona, [key]: value.split("\n").map((item) => item.trim()).filter(Boolean) });
    } else {
      setPersona({ ...persona, [key]: value });
    }
  };
  const save = async () => {
    try {
      const saved = await invoke<PersonaProfile>("save_persona", { persona });
      setPersona(saved);
      setMessage("Identité enregistrée. Elle guidera les prochains tours sans remplacer les souvenirs ni les preuves.");
    } catch (error) {
      setMessage(String(error));
    }
  };
  return (
    <article>
      <p className="role">IDENTITÉ CONVERSATIONNELLE</p>
      <Notice>
        Cette identité fixe le style et les limites de Jarvix. Elle ne prouve ni conscience ni mémoire de faits : les souvenirs et leurs sources restent séparés dans l’onglet Mémoire.
      </Notice>
      <div className="settings-grid">
        <label>Nom<input value={persona.name} onChange={(event) => update("name", event.target.value)} /></label>
        <label>Langue<input value={persona.language} onChange={(event) => update("language", event.target.value)} /></label>
      </div>
      <label>Ton<input value={persona.tone} onChange={(event) => update("tone", event.target.value)} /></label>
      <label>Principes — un par ligne<textarea value={persona.principles.join("\n")} onChange={(event) => update("principles", event.target.value)} /></label>
      <label>Limites — une par ligne<textarea value={persona.boundaries.join("\n")} onChange={(event) => update("boundaries", event.target.value)} /></label>
      <button className="primary" onClick={save}>Enregistrer l’identité</button>
      {message && <Notice kind="ok">{message}</Notice>}
    </article>
  );
}

function Profiler({
  port,
  elapsedMs,
}: {
  port: number;
  elapsedMs: number | null;
}) {
  const [status, setStatus] = useState<ProfilerStatus | null>(null);
  const refresh = () =>
    invoke<ProfilerStatus>("profiler_status", { port })
      .then(setStatus)
      .catch(() => setStatus(null));
  useEffect(() => {
    refresh();
  }, [port]);
  const gpu =
    status?.gpu_utilization_percent &&
    status.gpu_vram_used_mb &&
    status.gpu_vram_total_mb
      ? `${status.gpu_utilization_percent}% · ${status.gpu_vram_used_mb} / ${status.gpu_vram_total_mb} Mo`
      : "non remontée";
  return (
    <article>
      <p className="role">PROFILER</p>
      <dl className="metrics">
        <dt>Dernière exécution</dt>
        <dd>{elapsedMs ? `${elapsedMs} ms` : "aucune"}</dd>
        <dt>TTFT / tok/s</dt>
        <dd>
          {status?.metrics_available
            ? "Métriques disponibles ; export détaillé à raccorder."
            : (status?.metrics_message ?? "Vérification…")}
        </dd>
        <dt>GPU détecté</dt>
        <dd>{status?.gpu_name ?? "nvidia-smi indisponible"}</dd>
        <dt>GPU / VRAM globale</dt>
        <dd>{gpu}</dd>
        <dt>llama.cpp sur ce port</dt>
        <dd>
          {status?.llama_server_pid != null
            ? `PID ${status.llama_server_pid} · ${status.llama_server_vram_mb != null ? `${status.llama_server_vram_mb} Mo VRAM de calcul` : "VRAM non remontée par nvidia-smi"}`
            : "processus non identifié sur ce port"}
        </dd>
      </dl>
      <div className="actions">
        <button onClick={refresh}>Actualiser</button>
      </div>
      <p className="rule">
        La ligne globale concerne toute la machine. La ligne llama.cpp relie le port sélectionné à son PID puis interroge nvidia-smi ; elle reste vide si le pilote ne publie pas l’usage par processus. Le profil GTX 1080 lance llama.cpp avec{" "}
        <code>--metrics</code> sur le port 18181.
      </p>
    </article>
  );
}

function VerificationLab({ answer, suggestedQuery }: { answer: string; suggestedQuery: string }) {
  const [expression, setExpression] = useState("");
  const [testRoot, setTestRoot] = useState("D:\\op15-adaptive-harness");
  const [query, setQuery] = useState(suggestedQuery);
  const [sourceUrl, setSourceUrl] = useState("https://");
  const [webClaim, setWebClaim] = useState("");
  const [webOracle, setWebOracle] = useState<OracleReport | null>(null);
  const [sourceStatement, setSourceStatement] = useState("");
  const [oracle, setOracle] = useState<OracleReport | null>(null);
  const [test, setTest] = useState<ProjectTestReport | null>(null);
  const [message, setMessage] = useState("");
  const calculate = async () => {
    try {
      setOracle(await invoke<OracleReport>("verify_calculation", { expression, response: answer }));
    } catch (error) {
      setMessage(String(error));
    }
  };
  const runTests = async () => {
    if (!window.confirm("Exécuter cargo test dans ce projet ? Les tests peuvent compiler du code local, mais aucun fichier source ne sera modifié.")) return;
    try {
      setTest(await invoke<ProjectTestReport>("run_project_tests", { rootPath: testRoot }));
    } catch (error) {
      setMessage(String(error));
    }
  };
  const search = async () => {
    try {
      await invoke("search_web", { query });
      setMessage("Recherche ouverte dans le navigateur. Ajoute ensuite l’URL de la source consultée : elle restera une source à examiner, pas une preuve automatique.");
    } catch (error) {
      setMessage(String(error));
    }
  };
  const openSource = async () => {
    try {
      await invoke("open_browser_url", { url: sourceUrl });
      setMessage("Source ouverte. Vérifie l’auteur, la date, le passage pertinent et son lien réel avec la réponse avant de la retenir.");
    } catch (error) {
      setMessage(String(error));
    }
  };
  const recordSource = async () => {
    if (!window.confirm("Confirmer que tu as personnellement examiné cette source et que cette note décrit uniquement sa portée ?")) return;
    try {
      await invoke("record_reviewed_source", { url: sourceUrl, statement: sourceStatement });
      setMessage("Source examinée enregistrée avec sa provenance. Elle sera proposée comme contexte aux prochains tours, sans devenir automatiquement une vérité générale.");
      setSourceStatement("");
    } catch (error) {
      setMessage(String(error));
    }
  };
  const verifyWeb = async () => {
    try {
      setWebOracle(await invoke<OracleReport>("verify_web_claim", { url: sourceUrl, claim: webClaim }));
    } catch (error) {
      setMessage(String(error));
    }
  };
  return (
    <article>
      <p className="role">LABORATOIRE DE VÉRIFICATION</p>
      <Notice>
        Une vérification a toujours une portée limitée. Un calcul peut confirmer un nombre ; un test peut confirmer un comportement reproductible ; une recherche apporte des sources à examiner. Aucun de ces contrôles ne rend automatiquement toute la réponse vraie.
      </Notice>
      <div className="grid">
        <section>
          <h2>Calcul indépendant</h2>
          <label>
            Expression à contrôler
            <input value={expression} placeholder="(2 + 3) * 4" onChange={(event) => setExpression(event.target.value)} />
          </label>
          <button disabled={!answer || !expression.trim()} onClick={calculate}>Calculer et comparer à la réponse</button>
          {oracle && <div className={oracle.independently_verified ? "notice ok" : "notice"}><strong>{oracle.status}</strong><br />{oracle.summary}<br /><small>{oracle.artifact}</small></div>}
        </section>
        <section>
          <h2>Expérience locale</h2>
          <label>
            Projet Rust à tester
            <input value={testRoot} onChange={(event) => setTestRoot(event.target.value)} />
          </label>
          <button onClick={runTests}>Exécuter les tests du projet</button>
          {test && <div className={test.passed ? "notice ok" : "notice error"}><strong>{test.status}</strong><br />{test.summary}{test.output && <pre>{test.output}</pre>}</div>}
        </section>
      </div>
      <section>
        <h2>Recherche et sources</h2>
        <label>
          Recherche à ouvrir
          <input value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Question précise à documenter" />
        </label>
        <div className="actions"><button disabled={!query.trim()} onClick={search}>Rechercher sur le web</button></div>
        <label>
          URL d’une source consultée
          <input value={sourceUrl} onChange={(event) => setSourceUrl(event.target.value)} />
        </label>
        <div className="actions"><button disabled={sourceUrl === "https://"} onClick={openSource}>Ouvrir et examiner la source</button></div>
        <label>
          Affirmation courte à contrôler dans cette source
          <textarea value={webClaim} placeholder="Ex. La documentation active les métriques avec l’option --metrics." onChange={(event) => setWebClaim(event.target.value)} />
        </label>
        <button disabled={sourceUrl === "https://" || webClaim.trim().length < 8} onClick={verifyWeb}>Contrôler la concordance HTTPS</button>
        {webOracle && <div className={webOracle.independently_verified ? "notice ok" : "notice"}><strong>{webOracle.status}</strong><br />{webOracle.summary}<br /><small>{webOracle.artifact}</small></div>}
        <label>
          Portée réellement vérifiée dans cette source
          <textarea value={sourceStatement} placeholder="Ex. La documentation indique X, section Y ; elle ne démontre pas Z." onChange={(event) => setSourceStatement(event.target.value)} />
        </label>
        <button disabled={sourceUrl === "https://" || !sourceStatement.trim()} onClick={recordSource}>Conserver la source examinée</button>
      </section>
      {message && <Notice>{message}</Notice>}
    </article>
  );
}

function WorkspaceTools() {
  const [root, setRoot] = useState("D:\\op15-adaptive-harness");
  const [relativePath, setRelativePath] = useState("");
  const [entries, setEntries] = useState<FileEntry[]>([]);
  const [filePath, setFilePath] = useState("");
  const [content, setContent] = useState("");
  const [webQuery, setWebQuery] = useState("");
  const [url, setUrl] = useState("https://");
  const [message, setMessage] = useState("");
  const list = async (path = relativePath) => {
    try {
      setEntries(
        await invoke<FileEntry[]>("list_workspace_files", {
          rootPath: root,
          relativePath: path,
        }),
      );
      setRelativePath(path);
      setMessage(`Dossier ouvert : ${path || "."}`);
    } catch (error) {
      setMessage(String(error));
    }
  };
  const chooseRoot = async () => {
    const value = await open({
      directory: true,
      multiple: false,
      title: "Choisir le dossier que le harnais peut gérer",
    });
    if (typeof value === "string") {
      setRoot(value);
      setRelativePath("");
      setEntries([]);
    }
  };
  const read = async (path: string) => {
    try {
      setContent(
        await invoke<string>("read_workspace_file", {
          rootPath: root,
          relativePath: path,
        }),
      );
      setFilePath(path);
      setMessage(`Fichier chargé : ${path}`);
    } catch (error) {
      setMessage(String(error));
    }
  };
  const write = async () => {
    if (!filePath.trim()) {
      setMessage("Indique un chemin de fichier relatif au dossier choisi.");
      return;
    }
    if (
      !window.confirm(
        `Enregistrer « ${filePath} » dans le dossier autorisé ? Une copie .bak sera créée si le fichier existe.`,
      )
    )
      return;
    try {
      setMessage(
        await invoke<string>("write_workspace_file", {
          rootPath: root,
          relativePath: filePath,
          content,
        }),
      );
      list(relativePath);
    } catch (error) {
      setMessage(String(error));
    }
  };
  const search = async () => {
    try {
      await invoke("search_web", { query: webQuery });
      setMessage("Recherche ouverte dans le navigateur par défaut.");
    } catch (error) {
      setMessage(String(error));
    }
  };
  const browse = async () => {
    try {
      await invoke("open_browser_url", { url });
      setMessage("URL ouverte dans le navigateur par défaut.");
    } catch (error) {
      setMessage(String(error));
    }
  };
  return (
    <article>
      <p className="role">ACTIONS LOCALES APPROUVEES</p>
      <h2>Fichiers et navigateur</h2>
      <Notice>
        Le dossier choisi délimite l’action. Lire, créer et modifier sont
        visibles ici ; chaque écriture demande une confirmation et crée une
        sauvegarde lorsque le fichier existe.
      </Notice>
      <label>
        Dossier autorisé
        <input value={root} onChange={(event) => setRoot(event.target.value)} />
      </label>
      <div className="actions">
        <button onClick={chooseRoot}>Choisir un dossier</button>
        <button onClick={() => list()}>Parcourir</button>
        <button
          onClick={() =>
            invoke("open_workspace_folder", {
              rootPath: root,
              relativePath,
            }).catch((error) => setMessage(String(error)))
          }
        >
          Ouvrir dans l’Explorateur
        </button>
      </div>
      <label>
        Chemin dans le dossier
        <input
          value={relativePath}
          placeholder="Sous-dossier facultatif"
          onChange={(event) => setRelativePath(event.target.value)}
        />
      </label>
      <div className="memory-list">
        {entries.map((entry) => (
          <div key={entry.relative_path} className="turn assistant">
            <p className="role">
              {entry.is_directory
                ? "DOSSIER"
                : `${entry.size_bytes ?? 0} octets`}
            </p>
            <pre>{entry.relative_path}</pre>
            <button
              onClick={() =>
                entry.is_directory
                  ? list(entry.relative_path)
                  : read(entry.relative_path)
              }
            >
              {entry.is_directory ? "Ouvrir" : "Lire"}
            </button>
          </div>
        ))}
      </div>
      <fieldset>
        <legend>Créer ou modifier un fichier</legend>
        <label>
          Fichier relatif
          <input
            value={filePath}
            placeholder="notes/idee.md"
            onChange={(event) => setFilePath(event.target.value)}
          />
        </label>
        <label>
          Contenu
          <textarea
            value={content}
            onChange={(event) => setContent(event.target.value)}
            placeholder="Contenu à enregistrer"
          />
        </label>
        <button className="primary" onClick={write}>
          Enregistrer après confirmation
        </button>
      </fieldset>
      <fieldset>
        <legend>Navigateur internet</legend>
        <label>
          Recherche web
          <input
            value={webQuery}
            onChange={(event) => setWebQuery(event.target.value)}
          />
        </label>
        <button onClick={search} disabled={!webQuery.trim()}>
          Rechercher dans le navigateur
        </button>
        <label>
          URL http(s)
          <input value={url} onChange={(event) => setUrl(event.target.value)} />
        </label>
        <button onClick={browse}>Ouvrir l’URL</button>
      </fieldset>
      {message && <Notice>{message}</Notice>}
    </article>
  );
}

export default function App() {
  const [status, setStatus] = useState<Status | null>(null),
    [config, setConfig] = useState<Config>(blank),
    [persona, setPersona] = useState<PersonaProfile | null>(null),
    [page, setPage] = useState<Page>("Chat"),
    [prompt, setPrompt] = useState(""),
    [turns, setTurns] = useState<Turn[]>([]),
    [result, setResult] = useState<TaskResult | null>(null),
    [footerProfiler, setFooterProfiler] = useState<ProfilerStatus | null>(null),
    [error, setError] = useState(""),
    [running, setRunning] = useState(false);
  const refresh = () =>
    invoke<Status>("harness_status", { port: config.port || 8080 })
      .then(setStatus)
      .catch((e) => setError(String(e)));
  useEffect(() => {
    invoke<Config>("load_desktop_config")
      .then(setConfig)
      .catch(() => setConfig(blank));
    invoke<PersonaProfile>("load_persona")
      .then(setPersona)
      .catch(() => setPersona(null));
    try {
      const saved = localStorage.getItem(memoryKey);
      if (saved) setTurns(JSON.parse(saved));
    } catch {}
  }, []);
  useEffect(() => {
    refresh();
  }, [config.port]);
  useEffect(() => {
    invoke<ProfilerStatus>("profiler_status", { port: config.port || 8080 })
      .then(setFooterProfiler)
      .catch(() => setFooterProfiler(null));
  }, [config.port, result?.elapsed_ms]);
  useEffect(() => {
    localStorage.setItem(memoryKey, JSON.stringify(turns.slice(-20)));
  }, [turns]);
  const clear = () => {
    setTurns([]);
    localStorage.removeItem(memoryKey);
  };
  const submit = async (e: FormEvent) => {
    e.preventDefault();
    const text = prompt.trim();
    if (!text) return;
    setRunning(true);
    setError("");
    const context = turns
      .slice(-12)
      .map(
        (t) => `${t.role === "user" ? "Utilisateur" : "Assistant"}: ${t.text}`,
      )
      .join("\n");
    try {
      const response = await invoke<TaskResult>("run_task", {
        prompt: text,
        recentContext: context,
        model: config.model_name,
        port: config.port || 8080,
      });
      setResult(response);
      if (response.observation)
        setTurns((old) =>
          [
            ...old,
            { role: "user", text },
            { role: "assistant", text: response.observation },
          ].slice(-20),
        );
    } catch (x) {
      setError(String(x));
    } finally {
      setPrompt("");
      setRunning(false);
      refresh();
    }
  };
  const chat = (
    <>
      <article>
        {turns.length === 0 && (
          <Notice>
            La mémoire conversationnelle est vide. Les prochains tours seront
            conservés localement dans cette interface.
          </Notice>
        )}
        {turns.map((t, i) => (
          <div key={i} className={`turn ${t.role}`}>
            <p className="role">
              {t.role === "user"
                ? "UTILISATEUR"
                : "ADAPTIVE HARNESS — NON VERIFIE"}
            </p>
            <pre>{t.text}</pre>
          </div>
        ))}
        {result?.model_error && (
          <Notice kind="error">Erreur llama.cpp : {result.model_error}</Notice>
        )}
        {error && <Notice kind="error">{error}</Notice>}
        {result && (
          <div className="run-summary">
            <div>
              <p className="role">DERNIER TOUR</p>
              <strong>{result.decision}</strong>
              <small>{result.interaction_mode}</small>
            </div>
            <button onClick={() => setPage("Agent")}>Voir l’analyse</button>
          </div>
        )}
      </article>
      <form onSubmit={submit}>
        <textarea
          aria-label="Demande"
          value={prompt}
          placeholder="Écris normalement : le contexte récent est conservé."
          onChange={(e) => setPrompt(e.target.value)}
          onKeyDown={(event) => {
            if (event.key === "Enter" && !event.shiftKey) {
              event.preventDefault();
              if (!running && status?.llama_reachable && prompt.trim()) {
                void submit(event as unknown as FormEvent);
              }
            }
          }}
        />
        <button type="button" onClick={clear}>
          Effacer mémoire
        </button>
        <button
          className="primary"
          disabled={running || !status?.llama_reachable}
        >
          {running ? "Réponse…" : "Envoyer"}
        </button>
      </form>
    </>
  );
  const lastUserMessage = [...turns].reverse().find((turn) => turn.role === "user")?.text ?? "";
  const agent = (
    <>
    <article>
      <p className="role">CONTROLEUR</p>
      <dl className="metrics">
        <dt>Décision</dt>
        <dd>{result?.decision ?? "aucune"}</dd>
        <dt>Compréhension du tour</dt>
        <dd>{result?.interaction_mode ?? "aucune"}</dd>
        <dt>Analyse du raisonnement</dt>
        <dd>
          {result?.reasoning_lenses ||
            "Aucune lentille approfondie requise pour ce tour."}
        </dd>
        <dt>Spécialistes mobilisés</dt>
        <dd>
          {result?.specialists ||
            "Aucun rôle spécialisé requis pour ce tour."}
        </dd>
        <dt>Trace</dt>
        <dd>{result?.trace_summary ?? "aucune"}</dd>
        <dt>Contexte serveur</dt>
        <dd>{result?.prompt_tokens != null ? `${result.prompt_tokens} tokens traités${result.cached_tokens != null ? ` · ${result.cached_tokens} réutilisés depuis le cache` : ""}` : "non fourni par le serveur"}</dd>
        <dt>Oracle</dt>
        <dd>{result?.independently_verified ? "validé" : "non configuré"}</dd>
      </dl>
      <p className="rule">
        {result?.observation
          ? "Une réponse a été générée. Elle reste non vérifiée sans oracle indépendant."
          : status?.verification_rule}
      </p>
    </article>
    <VerificationLab answer={result?.observation ?? ""} suggestedQuery={lastUserMessage} />
    </>
  );
  const profiler = (
    <Profiler
      port={config.port || 8080}
      elapsedMs={result?.elapsed_ms ?? null}
    />
  );
  const bench = (
    <article>
      <p className="role">BENCHMARKS</p>
      <p>
        Le modèle actif sur 8080 est distinct du Qwen 9B validé. Les résultats
        GTX 1080 ne lui sont pas attribués.
      </p>
      <button onClick={() => setPage("Chat")}>Tester le chat</button>
    </article>
  );
  let content: React.ReactNode = chat;
  if (page === "Agent") content = agent;
  if (page === "Memoire") content = <Memory />;
  if (page === "Outils MCP")
    content = (
      <>
        <Memory tools />
        <WorkspaceTools />
      </>
    );
  if (page === "Modeles")
    content = (
      <Models config={config} setConfig={setConfig} refresh={refresh} />
    );
  if (page === "Profiler") content = profiler;
  if (page === "Benchmarks") content = bench;
  if (page === "Parametres")
    content = <><Settings config={config} setConfig={setConfig} /><PersonaSettings persona={persona} setPersona={setPersona} /></>;
  return (
    <main>
      <aside>
        <h1>
          ADAPTIVE
          <br />
          HARNESS
        </h1>
        <nav>
          {pages.map((x) => (
            <button
              key={x}
              className={x === page ? "active" : ""}
              onClick={() => setPage(x)}
            >
              {x}
            </button>
          ))}
        </nav>
      </aside>
      <section className="conversation">
        <header>
          <div>
            <strong>{page === "Chat" ? "Conversation locale" : page}</strong>
            <small>{status?.llama_endpoint ?? "Connexion…"}</small>
          </div>
          <span className={status?.llama_reachable ? "badge online" : "badge"}>
            {status?.llama_reachable ? "llama.cpp connecté" : "modèle arrêté"}
          </span>
        </header>
        <div className="page-content">{content}</div>
      </section>
      <aside className="inspector">
        <h2>ETAT</h2>
        <dl>
          <dt>Contrôleur</dt>
          <dd>{status?.controller_ready ? "prêt" : "inconnu"}</dd>
          <dt>llama.cpp</dt>
          <dd>{status?.llama_reachable ? "accessible" : "indisponible"}</dd>
          <dt>Mémoire chat</dt>
          <dd>{turns.length} tour(s)</dd>
          <dt>Identité</dt>
          <dd>{persona?.name ?? "chargement"}</dd>
        </dl>
        <button onClick={refresh}>Actualiser</button>
      </aside>
      <footer>
        Exécution locale · durée : {result ? `${result.elapsed_ms} ms` : "aucune"}
        {result?.server_predicted_per_second != null
          ? ` · génération : ${result.server_predicted_per_second.toFixed(1)} tok/s (${result.completion_tokens} tokens)`
          : result?.generation_tokens_per_second != null
            ? ` · génération : ${result.generation_tokens_per_second.toFixed(1)} tok/s (${result.completion_tokens} tokens)`
          : " · tok/s : non fourni par le serveur"}
        {result?.server_prompt_ms != null
          ? ` · préfill : ${result.server_prompt_ms.toFixed(0)} ms${result.server_prompt_per_second != null ? ` (${result.server_prompt_per_second.toFixed(1)} tok/s)` : ""}`
          : " · TTFT : non mesurable sans streaming"}
        {footerProfiler?.gpu_name
          ? ` · ${footerProfiler.gpu_name} : ${footerProfiler.gpu_utilization_percent ?? "?"}% · VRAM ${footerProfiler.gpu_vram_used_mb ?? "?"}/${footerProfiler.gpu_vram_total_mb ?? "?"} Mo (machine)`
          : " · GPU/VRAM : nvidia-smi indisponible"}
        {footerProfiler?.llama_server_pid != null
          ? ` · llama PID ${footerProfiler.llama_server_pid}${footerProfiler.llama_server_vram_mb != null ? ` : ${footerProfiler.llama_server_vram_mb} Mo` : ""}`
          : ""}
      </footer>
    </main>
  );
}
