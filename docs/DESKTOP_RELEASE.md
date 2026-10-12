# Desktop Release

## Delivered

The Windows desktop application is in `apps/desktop/`.

```text
React UI -> Tauri command -> shared Rust harness -> llama.cpp localhost
```

It provides a local conversation screen, GGUF/server configuration, controller
inspector, explicit verification state, explicit short- and long-term memory,
and an on-demand profiler. `run_task` calls the shared `TaskLoop`. The response
shown is the actual llama.cpp observation; it is never labelled verified because
the bundled availability check does not verify the claim.

The interface has one scroll surface per page. The chat keeps its composer
visible and shows a compact summary of the most recent decision; the detailed
reasoning lens view stays in **Agent**.

## Build and run

For development (the Vite server stays running):

```text
cd apps/desktop
npm install
npm run tauri dev
```

For normal use, build the standalone application then start the **release**
executable. Do not start `target/debug/adaptive-harness-desktop.exe` directly:
that debug binary intentionally expects the development server on port 1420 and
otherwise displays a browser error saying that localhost refused the connection.

```text
cd apps/desktop
npm run tauri -- build --no-bundle
src-tauri/target/release/adaptive-harness-desktop.exe
```

Build verification completed on Windows 11:

```text
npm run build                         PASS
npm run tauri -- build --no-bundle    PASS
```

Artifacts generated locally:

```text
apps/desktop/src-tauri/target/release/adaptive-harness-desktop.exe
```

## Current limits

- Token streaming, cancellation and SSE are not implemented yet.
- The user-selected model name is passed to the local llama.cpp endpoint; the
  endpoint must be reachable at `127.0.0.1:8080`.
- Voice engines and SQLite event persistence are absent.
- Codebase Memory MCP can be started, inspected and asked to index a project;
  MCP tool discovery and execution by the model are not implemented.
- The file and browser panels are manual, user-visible operations. Writes are
  constrained to an explicitly selected directory, confirmed in the UI, and
  create a `.bak` copy. They are not autonomous model actions.
- Hardware telemetry comes from llama.cpp `/metrics` when the selected server
  exposes it and from `nvidia-smi` when available. No displayed hardware value
  proves attribution to one particular generation.
- The current HTTP client remains a minimal client and must be replaced before
  exposing the application beyond the local machine.
