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

```text
cd apps/desktop
npm install
npm run tauri dev
```

Build verification completed on Windows 11:

```text
npm run build                 PASS
npm run tauri build -- --debug PASS
```

Artifacts generated locally:

```text
apps/desktop/src-tauri/target/debug/adaptive-harness-desktop.exe
apps/desktop/src-tauri/target/debug/bundle/msi/
apps/desktop/src-tauri/target/debug/bundle/nsis/
```

## Current limits

- Token streaming, cancellation and SSE are not implemented yet.
- The user-selected model name is passed to the local llama.cpp endpoint; the
  endpoint must be reachable at `127.0.0.1:8080`.
- Voice engines, SQLite event persistence, MCP tools and hardware telemetry
  are absent. The UI reports that state rather than simulating it.
- The current HTTP client remains a minimal client and must be replaced before
  exposing the application beyond the local machine.
