# Architecture

## Production minimale

1. Le **harness** exécute les actions autorisées et maintient un état durable.
2. Le **contrôleur** compare gain attendu, coût, risque et budget restant.
3. Le **governor** sélectionne les expériences et accepte/refuse les variants.
4. La **mémoire** conserve seulement des résultats avec provenance et contexte.
5. Les **runtimes** ne sont sélectionnés que par compatibilité et mesures E2E.

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
