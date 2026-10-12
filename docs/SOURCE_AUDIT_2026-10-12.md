# Audit des sources — état vérifiable au 12 octobre 2026

## Portée et règle de lecture

`SOURCES.md` contient **198 URL uniques** fournies au fil du projet. Elles sont
toutes conservées, classées par fonction et reliées à cet audit. Cela ne veut
pas dire que 198 projets ont été exécutés, intégrés ou validés sur Windows ou
Android.

L'audit distingue donc quatre états :

| État | Signification |
| --- | --- |
| **Audité** | Dépôt, documentation ou publication primaire lu ; rôle et limite consignés ici. |
| **Référence classée** | URL conservée et classée, mais pas encore étudiée fichier par fichier. |
| **À revalider** | URL ou dépôt à confirmer (accessibilité, licence, activité, compatibilité). |
| **Non retenu comme dépendance** | Peut inspirer une interface ou un protocole, mais ne doit pas être installé automatiquement. |

Une passe automatisée a identifié 105 cibles GitHub directes. Les métadonnées
publiques ont confirmé l'existence et la licence d'une première tranche, puis
l'API GitHub a limité les requêtes anonymes. Une erreur de l'outil de collecte
sur les suffixes de noms a aussi rendu plusieurs faux `404` : ils ne sont donc
pas interprétés comme des dépôts absents. L'audit détaillé devra reprendre par
lots authentifiés et conserver la réponse source, plutôt que convertir une
limite de débit en conclusion.

## Décisions déjà justifiées

| Domaine | Source primaire examinée | Décision pour OP15 Adaptive Harness |
| --- | --- | --- |
| Noyau local LLM | [llama.cpp](https://github.com/ggml-org/llama.cpp) et sa documentation serveur | **Conserver** l'API locale OpenAI-compatible ; mesurer la télémétrie au lieu de l'inventer. |
| Mémoire personnelle | [Jarvix Memory](https://github.com/gat45/jarvix-memory) | **Adapter MCP en lecture d'abord** ; aucune écriture automatique, provenance et consentement requis. |
| Harnais Android | [AOHP](https://github.com/aohp-os/aohp) | **Comparer l'architecture**, ne pas embarquer : projet de recherche, non preuve de sûreté ni de compatibilité OP15. |
| Actions Android | [Open Jarvis](https://github.com/tokenarc/open-jarvis) | **Étudier les permissions et la reprise**, jamais importer des permissions d'application sans modèle de capacités. |
| Logique formelle | [Logic-LM](https://arxiv.org/abs/2305.12295), [FOLIO](https://arxiv.org/abs/2209.00840) | **Oracle optionnel** : séparer traduction en logique, validité du solveur et vérité des prémisses. |
| Persistance Android | [Room](https://developer.android.com/training/data-storage/room), [WorkManager](https://developer.android.com/topic/libraries/architecture/workmanager) | **Cible APK** : sessions et références mémoire via Room ; tâches différées via WorkManager, sans promettre une boucle permanente. |
| Méta-raisonnement | [Russell & Wefald](https://doi.org/10.1016/0004-3702(91)90015-C), [RouteLLM](https://github.com/lm-sys/RouteLLM) | **Principe à mesurer** : comparaison de valeur attendue, coût et risque ; pas une règle fixe présentée comme apprentissage. |
| Évaluation agentique | [AgentDojo](https://github.com/ethz-spylab/agentdojo), [tau2-bench](https://github.com/sierra-research/tau2-bench) | **Références de test** ; le benchmark propre au harnais reste à construire et à sceller. |

## Matrice de sélection par famille

| Famille de `SOURCES.md` | Statut global | Ce qui est retenu maintenant | Ce qui est exclu de l'installation automatique |
| --- | --- | --- | --- |
| Dépôts personnels / OP15 | Audité au niveau de l'architecture connue | `op15-adaptive-harness`, `jarvix-memory`, documentation GénieX comme contexte | Tout clone non présent localement et tout backend NPU sans mesure reproductible. |
| Harnais et agents | Références classées | Interfaces : état, checkpoints, budget, permissions, traces | LangGraph, DSPy, GEPA, OpenHands et autres frameworks comme dépendances du noyau Rust. |
| Mémoire / RAG / MCP | Audité partiellement | Contrat de mémoire, provenance, recherche MCP en lecture | Mem0, Letta, LightRAG, HippoRAG, Qdrant et LlamaIndex comme pile imposée. |
| Raisonnement et cognition | Audité partiellement | Lentilles, registre d'arguments à faire, oracles calcul/contraintes | Affirmation qu'un modèle du monde, Soar, ACT-R, LIDA ou Dreamer rendent le harnais conscient. |
| Optimisation / observation | Références classées | Benchmark à budget égal, traces et télémétrie attribuée | DSPy/GEPA/Optuna/BoTorch/MLflow/Phoenix sans protocole de comparaison. |
| Inférence / accélérateurs | Audité partiellement | llama.cpp sur PC, CPU d'abord sur Android | vLLM, SGLang, FastFlowLM, EAGLE, XRT, MLIR-AIE et HTP comme promesse de performance OP15. |
| Android | Audité partiellement | Architecture officielle Android, Room, WorkManager, LiteRT-LM à comparer | Tous les assistants Android externes tant que licence, permissions et build ne sont pas vérifiés. |
| Logique, philosophie et éthique | Audité partiellement | Contrat : arguments, hypothèses, objections, états de preuve | Une réponse morale/philosophique étiquetée « vérifiée » par un solveur. |
| Pages de catalogues, profils et articles secondaires | Référence classée | Découverte de pistes seulement | Toute conclusion technique ou de licence fondée uniquement sur ces pages. |

## Ce que l'audit révèle comme angles morts

1. **Catalogue ≠ intégration.** Une URL dans `SOURCES.md` n'établit ni la
   compatibilité de licence, ni la qualité, ni un gain mesuré.
2. **La personnalité n'est pas une conscience.** Le profil persistant est un
   contrat de ton, de limites et de préférences ; il n'établit pas une expérience
   subjective.
3. **Une mémoire doit pouvoir être corrigée.** Les souvenirs utilisateur, les
   inférences du modèle et les preuves externes ne peuvent pas partager le même
   statut.
4. **La logique ne valide pas les prémisses.** Même une preuve Lean ou un
   solveur ne garantit que la formule formalisée ; il faut tracer la traduction
   et les sources.
5. **L'autonomie impose des capacités limitées.** Naviguer, lire ou modifier
   des fichiers requiert permission, validation des arguments, post-condition,
   journal et récupération après crash.
6. **Le matériel se mesure.** GPU, VRAM, NPU, tokens/s et économie ne sont pas
   déductibles du simple choix d'un backend ou d'un dépôt cité.

## Plan d'audit exhaustif restant

La prochaine passe ne doit pas « intégrer les 198 ». Elle doit produire, pour
**chaque URL**, une ligne versionnée avec : URL canonique, date, type, source
primaire consultée, licence (si code), activité, plate-forme, dépendances,
risque de permission/données, test reproductible et décision `retenir /
comparer / écarter / revalider`.

Ordre :

1. dépôts qui peuvent exécuter du code ou obtenir des permissions (Android,
   MCP, agents, navigateur, fichiers) ;
2. mémoire et RAG qui auraient accès aux conversations ;
3. outils de vérification, logique et benchmarks ;
4. serveurs et accélérateurs ;
5. catalogues, papiers, profils et articles secondaires.

Le résultat de cette passe est une **décision d'architecture prudente**, pas
une validation scientifique du harnais. Les critères d'implémentation et de
preuve mesurée restent dans [TODO_AUDIT_2026-10-12.md](TODO_AUDIT_2026-10-12.md).
