# Inventaire local

Audit lecture seule du 2026-10-11. Dépôt: `D:\op15-adaptive-harness`, commit de code `52e2fc7`.

| Composant | Fichier:ligne | Implémenté | Testé | Fonctionnel | Preuve | Lacune |
| --- | --- | --- | --- | --- | --- | --- |
| Budget | `src/budget.rs:2-24` | IMPLEMENTED | IMPLEMENTED | FUNCTIONAL | test budget épuisé | pas énergie/thermique/global |
| Contrôleur | `src/controller.rs:3-18` | PARTIAL | IMPLEMENTED | FUNCTIONAL | 5 assertions décisionnelles | pas utilité calibrée ni REASON/VERIFY réels |
| Preuves | `src/evidence.rs:2-17` | PARTIAL | IMPLEMENTED | FUNCTIONAL | test preuve fraîche | provenance non vérifiée/obligations absentes |
| Client llama.cpp | `src/llama_cpp.rs:3-30` | PARTIAL | IMPLEMENTED | FUNCTIONAL | appel HTTP local | pas stream/cancel/chat/retry JSON robuste |
| État | `src/state.rs:2-3` | PARTIAL | IMPLEMENTED | FUNCTIONAL | round-trip atomique | pas transaction/réconciliation |
| Modèle de soi | `src/self_model.rs:2-7` | PARTIAL | IMPLEMENTED | FUNCTIONAL | test verdicts | pas persistance/calibration |
| Métacognition/trace/vérificateur | `src/metacognition.rs`, `trace.rs`, `verifier.rs` | PARTIAL | NOT_RUN | UNVERIFIED | types présents | non raccordés au main, aucun oracle |
| Hardware | `src/hardware.rs:2-3` | PARTIAL | IMPLEMENTED | FUNCTIONAL | somme testée | aucune télémétrie réelle |
| RAG/MCP/DB/CI/benchmarks | dépôt | NOT_IMPLEMENTED | NOT_APPLICABLE | NOT_APPLICABLE | inventaire | absents |

Rust 2024, aucune dépendance Cargo. 107 URLs distinctes dans `SOURCES.md`. Aucun résultat de benchmark versionné.
