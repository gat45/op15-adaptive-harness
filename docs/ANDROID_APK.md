# Android APK Specification

Status: implementation plan. No Android APK has been built from this repository.

## Goal

```text
Compose UI -> JNI bridge -> shared Rust harness (.so) -> local inference adapters
```

The Android application must use the same decision, evidence, verification,
budget and event formats as `apps/desktop`. It must not duplicate the harness
in Kotlin or call a PC web server.

## ABI and build requirements

```text
ABI: arm64-v8a
minSdk: 29
targetSdk: 36
Rust target: aarch64-linux-android
Native output: libop15_adaptive_harness.so
```

The existing Android application is located at `E:\oneplus\app`. Integration
there is a separate repository change. The JNI layer must follow its existing
runtime `dlopen/dlsym` convention for llama.cpp and must not statically link a
conflicting llama library.

## Narrow JNI contract

```text
create_session(config_json) -> session_id
submit_task(session_id, task_json) -> request_id
poll_events(session_id, after_event_id) -> event_json[]
cancel_request(session_id, request_id) -> status
close_session(session_id) -> status
```

`task_json` and `event_json` are typed DTOs. They cannot contain an arbitrary
shell command or capability grant. A Kotlin screen renders events only after
the Rust runtime emits them.

## Screens

```text
CHAT       request, streamed observation, stop, error state
AGENT      decision, budget, evidence, verifier verdict, trace
MEMORY     verified episodes separated from hypotheses
PERF       CPU/GPU/HTP/RAM/TTFT only when an adapter measures them
SETTINGS   model import, backend selection, permissions, voice adapter
```

## Android lifecycle and storage

```text
Activity/Compose -> bound foreground service -> Rust runtime
                                  |
                                  +-> SQLite sessions and append-only events
```

A task must survive UI recreation. An external effect must record a durable
transition before execution and reconcile after process death. An Android
permission denial and an unavailable HTP backend are typed events, not silent
fallbacks.

## Validation gates

1. Cross-compile the Rust library for arm64-v8a.
2. Build a debug APK with the JNI library packaged.
3. Show a real controller event in Compose without llama.cpp.
4. Run a local GGUF task through the shared controller.
5. Verify a claim with an oracle bound to response and artifacts.
6. Measure CPU/RAM before adding Vulkan or HTP.
7. Treat Vulkan and Hexagon as experimental until device evidence exists.

No claim about HTP access from an APK is valid until the exact APK process has
opened and measured the backend on-device.
