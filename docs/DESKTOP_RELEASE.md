# Desktop Release

## Delivered

The Windows desktop application is in `apps/desktop/`.

```text
React UI -> Tauri command -> shared Rust harness -> llama.cpp localhost
```

It provides a local conversation screen, model field, controller inspector,
visible trace count, explicit verification state and hardware fields that show
`N/A` until a measured adapter exists. `run_task` calls the shared `TaskLoop`.
The response shown is the actual llama.cpp observation; it is never labelled
verified because the bundled availability check does not verify the claim.

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
- Voice engines, SQLite event persistence, MCP tools and hardware telemetry
  are absent. The UI reports that state rather than simulating it.
- The current HTTP client remains a minimal client and must be replaced before
  exposing the application beyond the local machine.
