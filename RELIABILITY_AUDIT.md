# Audit de fiabilité : priorités P0

## Verdict

Le projet ne doit pas additionner des architectures cognitives. Il doit relier
une décision, ses preuves, son autorisation, son effet externe et son verdict
par une transaction causale durable. Une amélioration de politique reste une
candidate testée hors échantillon, jamais une auto-modification libre.

## Invariants P0

1. **Preuve typée.** Un verdict `verified` exige les obligations propres à la
   revendication. Un oracle critique échoué bloque la promotion ; plusieurs
   contrôles faibles ne le compensent pas.
2. **Autorisation exécutable.** L'exécuteur contrôle au moment de l'action une
   capacité explicite, les arguments, le périmètre, le risque et l'autorisation.
   Une heuristique textuelle ou un jugement LLM ne vaut jamais permission.
3. **Transaction et reprise.** Les états `PROPOSED`, `APPROVED`, `STARTED`,
   `EFFECT_APPLIED`, `CONFIRMED` et `FAILED` sont distincts. Après interruption,
   une réconciliation avec le monde externe précède toute reprise : un timeout
   ne signifie pas qu'une action a échoué.
4. **Oracle d'état final.** Une réponse textuelle ne clôt pas une tâche. Le
   système vérifie l'état demandé : fichier, compilation, Git, mesure ou
   post-condition instrumentée.
5. **Budget global.** Temps, tokens, mémoire, appels outils, énergie et coût
   de contrôle sont réservés dans un budget unique, non additionnés localement.

## Rythmes séparés

- La boucle rapide décide, exécute, vérifie et trace la tâche courante.
- La boucle lente transforme uniquement des expériences terminées en candidates
  de politique, les teste en shadow/A-B, puis `PROMOTE`, `REJECT` ou `ROLLBACK`.

Le niveau métacognitif ne s'exécute pas en continu : règles déterministes pour
permissions et états, modèle léger pour routage/calibration, LLM seulement pour
les anomalies ambiguës.

## Cinq expériences bloquantes

| Expérience | Oracle | Résultat obligatoire |
| --- | --- | --- |
| Faux sentiment de vérification | un contrôle critique négatif parmi des contrôles positifs | `REJECT` |
| Mémoire empoisonnée | donnée récupérée contenant une instruction interdite | aucune extension de permission |
| Arrêt prématuré | première preuve plausible mais insuffisante | décision comparée à recherche supplémentaire |
| Apprentissage causal | policy corrigée vs baseline sur jeu indépendant | gain et variance hors entraînement |
| Routage matériel pathologique | backend disponible mais opération incompatible/lente | chemin sain ou refus, pas de fallback en boucle |

## Contrat matériel OP15

Le planificateur matériel reçoit une demande cognitive et retourne seulement
les chemins admissibles avec coût estimé. Chaque estimation est conditionnée
par `(model_hash, quantization, op, shape, backend, driver, runtime,
free_memory, thermal_state, estimated_transfer, estimated_dispatch)`. La
sélection précède la génération ; le coût E2E inclut allocations, copies,
dispatch, synchronisations et fallbacks. Aucun débit théorique seul ne décide.

## Sources et limites

Les audits de JARVIX Memory et les issues externes sont des signaux à reproduire
avant toute généralisation. Ils ne démontrent pas un défaut de tous les projets
ou de toutes les versions.

- Sécurité agent : https://github.com/ethz-spylab/agentdojo
- Agent Security Bench : https://github.com/agiresearch/ASB
- Évaluation par état final : https://github.com/sierra-research/tau2-bench
- Mémoire longue durée : https://arxiv.org/abs/2410.10813
- Évaluation mémoire : https://github.com/ProsusAI/MemEval
- Intervalles et calibration : https://github.com/scikit-learn-contrib/MAPIE
- Référence durable : https://github.com/langchain-ai/langgraph
- Sécurité MCP : https://github.com/modelcontextprotocol/servers/security
- Backend Hexagon : https://github.com/ggml-org/llama.cpp/tree/master/ggml/src/ggml-hexagon
- Variantes Hexagon : https://github.com/kan-linux/ggml-hexagon
