# Architecture vocale PC + OnePlus

Le noyau portable `voice_session` gère les états `Idle`, `Listening`,
`Transcribing`, `Thinking`, `Speaking` et `Interrupted`. Une reprise de parole
interrompt uniquement la lecture active ; elle ne donne jamais au flux audio le
droit d’annuler une action confirmée du harnais.

| Couche | PC Windows | OnePlus Android | Statut |
| --- | --- | --- | --- |
| Session/interruption/métriques | Rust partagé | Rust/JNI ou Kotlin partagé | IMPLEMENTED (états) |
| Capture + VAD | navigateur/Silero VAD | AudioRecord + VAD local | NOT_IMPLEMENTED |
| STT | whisper.cpp/Qwen ASR | whisper.cpp Android ou moteur local | NOT_INSTALLED |
| LLM | llama.cpp local | runtime local sélectionné | PC IMPLEMENTED |
| TTS | Piper/Kokoro | Piper Android/équivalent | NOT_INSTALLED |
| Lecture audio | navigateur/Windows | AudioTrack | NOT_IMPLEMENTED |

Métriques obligatoires : début VAD, fin STT, premier token, premier audio TTS,
fin lecture et interruption. Ce projet ne lance aucun moteur vocal tant que les
binaires/modèles correspondants ne sont pas explicitement installés et validés.
