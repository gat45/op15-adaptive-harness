# Runtime Audit - 2026-10-11

## Scope

Reviewed all executable Rust modules, Rust tests, and the Tauri desktop source.
This is a host-code audit, not a proof for Android, GPU, HTP, STT, TTS, MCP or
long-duration workload behavior.

## Corrected

| Finding | Correction | Validation |
|---|---|---|
| Unbounded llama.cpp HTTP body could allocate memory until peer close | response is capped at 8 MiB and invalid UTF-8 is rejected | dedicated unit test |
| No write timeout for llama.cpp | 10 second write timeout added | compiled; transport errors are preserved |
| Desktop used a file-existence check as a chat verifier | replaced with `UnverifiedVerifier`; chat output can never be labelled verified | desktop build |
| Blocking inference work could occupy the desktop command runtime | `run_task` runs in `spawn_blocking` | Tauri build |
| Strategy ranking could panic on NaN; counters could overflow | `total_cmp` and saturating counters | Rust tests |
| Hardware latency sum could overflow | saturating additions | Rust tests |

## Confirmed limits, not fixed by simulation

```text
streaming tokens            NOT IMPLEMENTED
request cancellation         NOT IMPLEMENTED
SQLite event persistence     NOT IMPLEMENTED
voice adapters               NOT IMPLEMENTED
MCP execution                NOT IMPLEMENTED
GPU/HTP/XDNA telemetry       NOT IMPLEMENTED
Android JNI/APK              NOT IMPLEMENTED
```

The desktop UI explicitly reports unavailable components. It must not be used
as evidence that these backends work.

## Test record

```text
cargo test                         PASS: 19 tests
apps/desktop: npm run build        PASS
apps/desktop: npm run tauri build -- --debug  PASS
```

The desktop process was launched locally after the build. Its visual window
was not available to the automation inventory, so no UI click-path claim is
made from this audit.
