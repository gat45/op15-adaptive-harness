# Platform Architecture

Status: approved target architecture. This document describes the target, not
features already proven in the current implementation.

## One Rust core, two native applications

```text
+---------------------------- HARNESS CORE ----------------------------+
| decisions | budgets | evidence | verification | traces | voice state |
+-------------------------------+-------------------------------------+
                                |
                 +--------------+--------------+
                 |                             |
                 v                             v
+-----------------------------+   +-----------------------------+
| Windows desktop             |   | Android APK                 |
| Tauri 2 + React             |   | Kotlin + Jetpack Compose    |
| Rust native                 |   | Rust arm64-v8a .so via JNI  |
+--------------+--------------+   +--------------+--------------+
               |                                 |
     llama.cpp / LM Studio                 embedded llama.cpp
     MCP / restricted tools                Android permissioned APIs
     CPU / CUDA / experimental XDNA2       CPU / Vulkan / experimental HTP
     local STT / TTS                       local STT / TTS
```

The boxes below the core are adapters. They cannot alter a decision, promote
an evidence item, or bypass a permission. Hardware presence is never a claim
that a backend works.

## Current implementation versus target

| Component | Current state | Target state |
|---|---|---|
| Rust controller | implemented and tested on host | shared by both applications |
| claim-bound verifier | implemented and tested | typed oracles and artifact criteria |
| event journal | in-memory causal trace only | durable, queryable event journal |
| desktop UI | not implemented | Tauri/React consumes real events |
| Android JNI | not implemented | arm64-v8a library with narrow JNI API |
| Android UI | not implemented | Compose consumes real events |
| local LLM on Android | not implemented in this repository | llama.cpp service with lifecycle handling |
| voice engines | state contract only | selectable VAD/STT/TTS adapters |
| HTP/XDNA2 | no integrated adapter | experimental, measured per backend |

## Runtime loop

```text
USER REQUEST
     |
     v
LOAD SESSION + RELEVANT VERIFIED EPISODES
     |
     v
DECIDE -----> ANSWER / RETRIEVE / VERIFY / STOP
     |                  |
     |                  v
     |           OBSERVATION OR TYPED ERROR
     |                  |
     +------------------+
                        v
              CLAIM-BOUND INDEPENDENT VERIFIER
                        |
             +----------+----------+
             |                     |
             v                     v
      VERIFIED EPISODE       FAILURE DIAGNOSIS
             |                     |
             +----------+----------+
                        |
                        v
              PERSIST EVENT + UPDATE BUDGET
```

A verifier must receive the claim, response, artifacts, and success criteria.
An availability check can report an artifact but cannot verify a claim.

## Native boundary contracts

```text
Kotlin / React
    |
    | submit task, subscribe events, request cancellation
    v
platform adapter
    |
    | typed DTOs only; no arbitrary shell text
    v
harness runtime
    |
    | decision, evidence, budgets, trace events
    v
inference / tools / verifier adapters
```

The frontend cannot execute a shell command, access a secret, or mark an
operation verified. Sensitive actions require an explicit capability and a
confirmed durable transition.

## Delivery order

1. Durable event format and typed inference outcomes in the Rust core.
2. Windows desktop app for repeatable UI, LLM, benchmark, and voice tests.
3. JNI library and Compose screen showing the same event format.
4. Android llama.cpp service, session lifecycle and local storage.
5. Optional Vulkan, HTP, XDNA2 and voice backends, each gated by measured tests.

## Measurement rule

```text
available adapter -> show measured value
unavailable adapter -> show N/A
failed adapter -> show failure reason
unknown capability -> show unknown
```

No UI may synthesize tokens/s, HTP usage, memory copies, confidence, or
verification success.
