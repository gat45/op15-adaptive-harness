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

## M6 — métacognition vérifiable

- [ ] Écrire une trace causale par décision : contexte, hypothèse, incertitude,
  budget, action choisie, résultat, vérification et cause d'échec.
- [ ] Construire un modèle de compétence calibré par famille de tâches et
  mesurer Brier score, ECE, risque sélectif et couverture.
- [ ] Rendre toute explication traçable vers les signaux réellement disponibles ;
  interdire les justifications post-hoc non étayées.
- [ ] Mettre en place la récupération d'épisodes comparables et évaluer sa
  valeur contre une mémoire de logs non structurée.
- [ ] Mesurer la détection d'impasse, l'arrêt utile et la reprise après échec.
- [ ] Prototyper le contrôle MIDCA : `note → assess → guide`, avec action méta
  limitée à des politiques explicitement autorisées et journalisées.
- [ ] Ajouter une attention concurrentielle de la mémoire active ; comparer à
  FIFO/récence simple sur les mêmes tâches.
- [ ] Implémenter des compétences inspirées de Voyager : préconditions,
  exécutable, tests, statistiques, provenance et récupération.
- [ ] Ajouter une correction inspirée de Reflexion seulement après un échec
  vérifié ; mesurer que la tentative suivante change effectivement.

## M7 — recherche cognitive, sans allégation de conscience

- [ ] Étudier Soar, ACT-R, CLARION, LIDA et Hyperon comme sources de concepts,
  sans importer de dépendance de production par défaut.
- [ ] Documenter séparément les indicateurs fonctionnels testables et les
  hypothèses philosophiques non testables dans ce projet.
