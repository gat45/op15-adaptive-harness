# Architecture cognitive intégrée

Ce document définit le noyau commun Windows/Android. Il ne revendique ni
conscience ni compréhension parfaite : une interprétation de ton, d'ironie ou
d'implicite est une hypothèse révisable, jamais une lecture de l'intention.

```text
Perception -> contexte de travail -> mémoire/provenance -> hypothèses/plan
     ^                  |                    |                  |
     |                  v                    v                  v
UI <--------- trace causale <---- vérificateur <---- outils sous permissions
                                        |
                              résultats mesurés -> apprentissage contrôlé
```

## Contrat des douze axes

| Axe | Noyau commun | État actuel | Preuve requise avant promotion |
| --- | --- | --- | --- |
| Pragmatique | `interaction.rs`, contrat de réponse | classification prudente + contexte transmis au LLM | jeu de dialogues ambigus annotés |
| Mémoire associative | `memory_store.rs` + provenance | épisodes explicites, récupération lexicale, tombstones | liens, conflits et datation testés |
| Raisonnement | `TaskLoop` | génération locale et trace d'observation | hypothèses comparées sur oracle |
| Métacognition | `metacognition.rs`, `SelfModel` | structures et arrêt sous budget | calibration, ECE/Brier et abstention |
| Planification | objectifs + registre de politique | état d'objectif, pas de plan exécutable général | préconditions, post-conditions et reprise |
| Actions/outils | permissions + transitions | contrat et tests unitaires | capacités signées, arguments validés, transaction durable |
| Apprentissage | registre de candidates | verdict vérifié requis avant promotion | shadow mode, régression et rollback |
| Optimisation | budget/hardware | coût déterministe, profil modèle configurable | benchmark coût par succès et télémétrie réelle |
| Dialogue | contexte court + mémoire explicite | tours récents et suivi de référence | interruptions, correction et changement de sujet |
| Proactivité | propositions bornées | non activée par défaut | expérience approuvée et résultat mesuré |
| Multimodalité | artefacts avec provenance | non intégrée | adaptateurs image/audio/document et tests de source |
| Interface | Tauri/React | chat, mémoire, modèles, MCP, trace et profilage | vues issues des mêmes données noyau |

## Couche de raisonnement transversal

`reasoning_router.rs` est le premier composant de cette couche. Il sélectionne
des lentilles explicites avant une génération : épistémique en permanence, puis
causalité/méthode scientifique/décision pour une régression, quantitatif pour
des mesures, logique pour une contradiction, linguistique pour une ambiguïté et
planification pour une action. L'interface affiche les lentilles de la dernière
exécution dans l'inspecteur Agent.

Les composants suivants partagent l'état de mémoire, les preuves et la trace ;
ils ne créent pas six bases concurrentes :

| Composant | Première responsabilité mesurable |
| --- | --- |
| `KnowledgeOntology` | concepts, relations et source d'une relation |
| `ReasoningRouter` | choisir et exposer les lentilles applicables |
| `LogicVerifier` | signaler une prémisse contradictoire, sans déclarer un texte LLM vrai |
| `MathEngine` | calculs déterministes avec unités et valeurs source |
| `CausalReasoner` | hypothèses concurrentes, prédictions et expérience discriminante |
| `EpistemicTracker` | séparer observé, déclaré, inféré, vérifié et inconnu |

## Comportements non négociables

1. Une réponse familière, ironique ou émotionnelle n'est pas refusée par une
   règle de surface. Le modèle reçoit le contexte et répond de manière calme.
2. Une référence ambiguë n'entraîne pas une action irréversible sans
   clarification ; avec un historique pertinent, le système peut la relier à
   ce contexte et l'indiquer dans la trace.
3. Une mémoire déclarée par l'utilisateur est `user_asserted`, pas un fait
   indépendant. Une relation ou une inférence conserve la même provenance.
4. Toute action à effet externe passe par permission, validation d'arguments,
   précondition et résultat observé.
5. Une stratégie ne peut être promue qu'après des résultats vérifiés à budget
   identique ; les sorties du modèle seules ne constituent pas de l'apprentissage.

## Vues de l'interface cible

Les vues suivantes lisent les mêmes enregistrements noyau ; aucune ne possède
une mémoire cachée propre.

1. **Conversation** : tours, interprétation prudente et références récupérées.
2. **Carte mémoire** : épisodes, relations, dates, provenance, conflits et
   oubli ; à construire lorsque les relations versionnées existent.
3. **Plan** : hypothèses, actions proposées, coût, statut et reprise.
4. **Preuves** : affirmation, source, oracle, date d'observation et verdict.
5. **Approbations** : capacités, arguments, risque, décision humaine et effet.
6. **Performances** : modèle/backend, TTFT, débit, RAM/VRAM/énergie et fallback
   uniquement lorsqu'une sonde fournit une mesure.

## Ordre de construction

1. Ajouter liens versionnés, contradiction et source aux souvenirs ; connecter
   JARVIX/RAG derrière un adaptateur optionnel.
2. Créer un plan borné avec pré/post-conditions et journal persistant.
3. Brancher un `EvidenceResolver` et les actions MCP à permissions transactionnelles.
4. Exposer carte mémoire, preuves, plan et approbations depuis ces données.
5. Évaluer les comportements sur un jeu scellé : contexte, ambiguïté,
   contradiction, coût, arrêt et récupération après interruption.

## Références utilisées comme sources de conception

- ReAct : https://arxiv.org/abs/2210.03629
- Reflexion : https://arxiv.org/abs/2303.11366
- MemGPT : https://arxiv.org/abs/2310.08560
- Generative Agents : https://arxiv.org/abs/2304.03442
- JARVIX Memory : https://github.com/gat45/jarvix-memory
- LangGraph : https://github.com/langchain-ai/langgraph
- DSPy : https://github.com/stanfordnlp/dspy

Ces références inspirent des mécanismes précis ; elles ne valident ni une
intégration commune ni une autonomie générale.
