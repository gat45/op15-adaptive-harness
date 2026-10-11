# Rapport d’intégration — portefeuille thecodacus

Date d’audit : 2026-10-11  
Périmètre : 90 dépôts publics signalés ; neuf dépôts clonés en lecture seule pour étude.  
Règle : une description de README ou une issue n’est pas une preuve de sécurité ni de performance.

## Décision

`op15-adaptive-harness` conserve son contrôleur Rust comme autorité de décision,
de budget, de permissions et de vérification. Les projets externes sont des
références ou des adaptateurs optionnels ; aucun agent, exécuteur ou modèle de
mémoire externe ne devient une dépendance de production par défaut.

## Sources retenues et état local

| Dépôt | Révision locale | Licence constatée | Ce qui est retenu | Décision |
|---|---:|---|---|---|
| [Pithagoras](https://github.com/thecodacus/pithagoras) | `c2c54e3` | Apache-2.0 | journal d’événements, reprise de session, métriques audio détaillées, séparation interface/exécution | Référence, aucune copie d’exécuteur |
| [Dexter](https://github.com/thecodacus/dexter) | `178ffe8` | Apache-2.0 | découpage Rust microphone → STT → LLM → TTS et réponse par segments | Adapter Windows/Android à écrire ; français obligatoire |
| [Understory](https://github.com/thecodacus/understory) | `f6c3423` | Apache-2.0 | concepts Markdown liés, provenance et interface MCP | Étudier comme mémoire secondaire ; JARVIX reste l’adaptateur prioritaire |
| [LLM Eval Suite](https://github.com/thecodacus/llm-eval-suite) | `84a7345` | aucun fichier de licence constaté | oracles déterministes, serveur de capture HTTP, comparaison à budget égal | Étudier les formats ; ne pas réutiliser le code |
| [AgentBox](https://github.com/thecodacus/agentbox) | `96bb678` | Apache-2.0 | modèle de bac à sable, espaces de travail et journal d’audit | Référence ; pas de socket Docker exposé au LLM |
| [decision-playground](https://github.com/thecodacus/decision-playground) | `843f72e` | aucun fichier de licence constaté | décision structurée et comparaison de candidats | Étude du protocole `/v1/decision` ; pas une dépendance |
| [spec-wins](https://github.com/thecodacus/spec-wins) | `776e799` | MIT | scénarios où spécification, code et tests divergent | Porter des tests dérivés avec oracles propres |
| [qwen3-omni-duplex](https://github.com/thecodacus/qwen3-omni-duplex) | `b4e5926` | aucun fichier de licence constaté | séparation chemin audio rapide / raisonnement | Recherche seulement : incompatible et non mesurée sur OP15/GTX 1080 |
| [Nerve-Protocol](https://github.com/thecodacus/Nerve-Protocol) | `8ffd771` | Apache-2.0 | appairage, permissions par machine, bouton d’arrêt et audit local | Référence pour un futur client distant |

Les clones sont sous `external/thecodacus/` et sont ignorés par Git : ils ne
sont ni redistribués ni incorporés au dépôt public.

## Éléments déjà pris pour le harnais

1. **Machine vocale portable.** `VoiceSession` applique les états
   `Idle → Listening → Thinking → Speaking`, l’interruption contrôlée et les
   mesures VAD/STT/TTFT/TTS. C’est l’abstraction commune PC et OnePlus, inspirée
   des flux de Pithagoras et Dexter, mais écrite dans ce dépôt.
2. **Règle de responsabilité.** Une interruption vocale annule la lecture et
   la génération en cours ; elle ne peut pas annuler une action externe déjà
   confirmée. Cette séparation est nécessaire avant tout outil vocal.
3. **Évaluation orientée preuves.** `spec-wins` devient la première source de
   tâches adversariales : le harnais devra choisir une spécification fiable et
   un oracle indépendant plutôt qu’optimiser un test visible erroné.

## Ce qui ne doit pas être importé tel quel

- Les listes de commandes filtrées de Dexter ne constituent pas un sandbox.
- Les sorties d’outils non fiables de Pithagoras demandent une isolation et une
  provenance plus fortes que des heuristiques d’injection.
- L’exécution Python ou `curl` générée par un modèle dans LLM Eval Suite doit
  être isolée hors de l’hôte.
- Un volume Docker ou le socket Docker ne doit jamais être fourni directement
  à un outil contrôlé par le modèle.
- Les modèles de qwen3-omni-duplex, ses métriques GPU et son duplex ne sont pas
  transposables à l’OP15 ou à la GTX 1080 sans campagne dédiée.

## Plan d’intégration

### P0 — validation du contrôleur

- [ ] Créer trois cas dérivés de `spec-wins`, avec réponses scellées et oracles
  locaux indépendants.
- [ ] Comparer `llama.cpp seul`, politique fixe et politique adaptative sur les
  mêmes tâches ; publier coût par succès vérifié.
- [ ] Ajouter au traceur les motifs de refus : budget, absence de preuve,
  outil indisponible et interruption.

### P1 — voix PC puis OnePlus

- [ ] Installer séparément et valider VAD, STT multilingue français et TTS ;
  enregistrer versions, modèles, RAM, TTFT et première sortie audio.
- [ ] Conserver le modèle STT en mémoire entre les tours, mesurer le gain et
  ajouter un message vocal explicite en cas d’échec.
- [ ] Écrire des adaptateurs `PcAudioAdapter` et `AndroidAudioAdapter` derrière
  `VoiceSession`, sans partager les APIs de capture audio.
- [ ] Tester l’interruption pendant parole, génération et appel d’outil.

### P2 — mémoire et exécution distante

- [ ] Étudier un adaptateur MCP optionnel JARVIX/Understory avec provenance,
  révision et opération réversible.
- [ ] N’autoriser une consolidation de mémoire que si une post-condition
  vérifiable confirme l’écriture.
- [ ] Concevoir un protocole de permissions par appareil inspiré de
  Nerve-Protocol, avec arrêt d’urgence local et journal inviolable.

### P3 — recherche, hors chemin de production

- [ ] Évaluer un endpoint de décision structuré seulement si llama.cpp local le
  fournit réellement et si ses probabilités sont calibrées.
- [ ] Évaluer le duplex audio sur matériel compatible ; ne pas promettre une
  conversation full-duplex avant résultat reproductible.

## Inventaire des 90 dépôts

Les autres dépôts signalés restent des pistes de triage, pas des dépendances :
`llama.cpp`, `OpenWebContainer`, `rendiv`, `Face-Recognition`,
`object-recognition-sift-surf`, `lazy-dev`, `local-ai-configs`,
`codacus-coder`, `workdesk-ai`, `jarvis`, `dash-ai`, `generative_agents`,
`bentodeveloper`, `focus.ai`, `budget-meter-v2`, `turing-monitor`,
`mem0ai-in-a-box`, `clawdbot`, `MinecraftAI`, `stackbridge-client`,
`Nerve-Protocol` et les dépôts web, mobiles, modèles, templates ou applications
métier restants. Ils ne seront examinés qu’en réponse à une hypothèse testable
et une nécessité concrète du harnais.

## Verdict

**ACCEPT — sélection de neuf références, trois apports immédiats et aucune
fusion aveugle.** La prochaine preuve attendue est un benchmark reproductible
sur tâches adversariales et un pipeline vocal français réellement mesuré.
