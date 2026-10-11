# TODO

## M0 — socle sûr

- [ ] Importer les modules validés du harnais vivant : état durable, trajectoire,
  contexte typé, registre d'outils et cache sans effet de bord.
- [ ] Exécuter les tests du contrôleur sur Termux et consigner les versions.
- [ ] Définir les permissions et post-conditions de chaque outil.

## M1 — mémoire et preuves

- [ ] Connecter JARVIX Memory par MCP/HTTP derrière un adaptateur facultatif.
- [ ] Connecter rag-jarvix pour la recherche lexicale/Rust sans doubler le RAG.
- [ ] Mettre en place `EvidenceResolver` et `ToolOutcomeVerifier`.
- [ ] Tester mémoire périmée, contradiction, faux succès et reprise interrompue.

## M2 — évaluation

- [ ] Créer `opt`, `select`, `regression` et `final` avec oracles scellés.
- [ ] Mesurer `model_only`, `tools`, `deep`, `greedy`, `random`, `adaptive`.
- [ ] Publier coût par succès vérifié, échecs et décisions PROMOTE/REJECT.

## M3 — routage

- [ ] Ajouter Self-RAG comme stratégie conditionnelle récupère/critique/vérifie.
- [ ] Entraîner RouteLLM en shadow mode sur décisions-oracle uniquement.
- [ ] Calibrer les seuils du contrôleur sur `opt`, jamais sur `final`.

## M4 — optimisation hors ligne

- [ ] Installer DSPy/GEPA dans `.venv-opt` et optimiser un paramètre à la fois.
- [ ] Comparer Optuna à random/greedy ; n'utiliser BoTorch que si les essais
  matériels multi-objectifs sont réellement coûteux.
- [ ] Versionner candidates, métriques, traces et rollback.

## M5 — runtime et matériel

- [ ] Connecter llama.cpp persistant comme baseline.
- [ ] Bench séparé SGLang puis EAGLE contre llama.cpp : TTFT, decode, RAM,
  exactitude et coût complet.
- [ ] Normaliser les traces HTP/XDNA (prefill, dispatch, transfert, sync,
  fallback) ; sélectionner un backend avant génération.
- [ ] Ne lancer aucune campagne device avant reconnexion manuelle et verrou NPU.
