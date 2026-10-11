# Architecture

## Production minimale

1. Le **harness** exécute les actions autorisées et maintient un état durable.
2. Le **contrôleur** compare gain attendu, coût, risque et budget restant.
3. Le **governor** sélectionne les expériences et accepte/refuse les variants.
4. La **mémoire** conserve seulement des résultats avec provenance et contexte.
5. Les **runtimes** ne sont sélectionnés que par compatibilité et mesures E2E.

## Extension expérimentale : contrôle à deux niveaux

Le niveau objet exécute la tâche : percevoir les preuves, interpréter, choisir
un but, planifier, agir et observer le résultat. Le niveau méta lit une trace
de ce cycle, détecte les écarts, formule une cause testable, puis modifie une
politique du niveau objet ou décide d'arrêter. C'est l'usage du principe
MIDCA, et non une demande au LLM de commenter sa dernière réponse.

Une couche d'attention sélectionne les événements qui doivent entrer dans la
mémoire active et les décisions qui doivent être diffusées au contrôleur. Les
compétences réutilisables restent des artefacts vérifiables : préconditions,
action exécutable, post-conditions, tests, coûts, taux de succès et procédure
de récupération.

La synthèse MIDCA/LIDA/Reflexion/Voyager est une hypothèse de recherche. Elle
ne devient une architecture de production qu'après comparaisons à budget égal
contre le harnais de base.

## Frontières

- DSPy, GEPA, Optuna et BoTorch sont des optimiseurs hors ligne.
- SGLang, EAGLE et llama.cpp sont des runtimes de benchmark interchangeables.
- RouteLLM et Self-RAG deviennent des politiques évaluées, pas des autorités.
- LangGraph est une référence ; il n'est ajouté que si l'état durable local ne
  suffit plus.

## Trace minimale

```json
{"task_id":"…","strategy":"adaptive","decision":"verify","verified":false,"backend":"…","cost":{"wall_ms":0,"tokens":0,"tool_calls":0},"outcome":"success|failure|timeout|abstain"}
```

Les oracles doivent être externes au LLM : test, compilation, calcul exact,
post-condition fichier, mesure ou citation contrôlée.
