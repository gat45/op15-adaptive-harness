# Connexion llama.cpp

Méthode réellement utilisée : `src/llama_cpp.rs:10-20`, `POST /v1/completions` OpenAI-compatible vers `127.0.0.1:8080`; aucun chat endpoint, exécutable natif ou bibliothèque native.

Mesuré le 2026-10-11 : modèle `F:\tiers\Nanbeige4.2-3B-Q8_0.gguf` GGUF Q8_0 ; complétion 9 tokens entrée, 8 sortie, 17 total, 970 ms côté client, 25.54 tok/s prefill, 9.21 tok/s decode ; endpoint invalide HTTP 404. `llama_cpp_e2e` PASS.

NOT_RUN : streaming, annulation, timeout serveur, outils, contexte réutilisé, reprise réseau, mémoire, version serveur, mémoire RAM et matériel OP15. Ce ne sont pas des benchmarks OP15.
