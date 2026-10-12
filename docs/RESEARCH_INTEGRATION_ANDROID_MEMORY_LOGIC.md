# Intégration : mémoire Jarvix, Android et raisonnement vérifiable

_État : recherche vérifiée le 12 octobre 2026. Ce document est un plan de
conception ; aucun dépôt tiers n'est une dépendance automatique._

## Décision d'architecture

```text
Interface PC / Android
        │
OP15 Adaptive Harness ── identité, budget, permissions, planification
        │
Jarvix Memory ── épisodes, décisions, hypothèses, preuves, contradictions
        │
Moteurs bornés ── llama.cpp · calculateur · tests · source HTTPS · MCP autorisé
```

OP15 reste responsable de choisir une action et de demander une confirmation.
Jarvix Memory reste responsable de conserver un souvenir avec provenance et de
garder une hypothèse réfutée. Le LLM ne décide jamais seul qu'une sortie devient
un fait durable.

## Sources étudiées et décision

| Source | Apport confirmé | Décision |
|---|---|---|
| [Jarvix Memory](https://github.com/gat45/jarvix-memory) | Mémoire locale multi-couches, serveur MCP, vérification déterministe, expériences et garde `do_not_repeat`. | **Intégrer par MCP**, sans recopier sa base SQLite ni ses règles de consolidation. |
| [AOHP](https://github.com/aohp-os/aohp) | Harnais au niveau du système Android, suivi de flux et actions inter-applications. Le dépôt le décrit comme prototype précoce. | **Référence sécurité uniquement** ; ne pas intégrer à OP15 ni accorder d'accès système étendu. |
| [Open Jarvis](https://github.com/tokenarc/open-jarvis) | Référence d'agent Android local avec voix et actions. | **Étudier l'UX et les permissions** ; vérifier licence, API et tests avant toute réutilisation. |
| [Room](https://developer.android.com/training/data-storage/room) | Couche SQLite Android avec requêtes vérifiées à la compilation et migrations. | **Choix Android** pour identité, sessions et index local. |
| [WorkManager](https://developer.android.com/topic/libraries/architecture/workmanager) | Travail persistant différé, pas une garantie de boucle permanente. | **Choix Android** pour consolidation, indexation et nettoyage différés. |
| [Logic-LM](https://arxiv.org/abs/2305.12295) | LLM → formulation symbolique → solveur ; l'article évalue cinq jeux de données. | **Référence de protocole** : formalisation visible, solveur séparé, erreur de traduction traçable. |
| [FOLIO](https://arxiv.org/abs/2209.00840) | Corpus NL/FOL de raisonnement logique, avec annotations vérifiables par moteur d'inférence. | **Banc de test**, pas dépendance d'exécution. |
| [Lean 4](https://lean-lang.org/) | Vérification de preuves formelles dans un cadre explicitement formalisé. | **Oracle optionnel PC** pour une proposition mathématique, jamais pour trancher un dilemme moral. |

Les projets Letta, Mem0, Graphiti, LangMem, Cognee, Engram, Cortex Memory,
Brownie, Jenny, PalmClaw, ClosePaw, Chevere AI, ZorvAI, ADK Kotlin et Droid
Harness restent dans la veille comparative. Ils ne seront ajoutés qu'après audit
de licence, protocole, isolation, données envoyées hors appareil et tests.

## Ce que cela ajoute au harnais

### 1. Mémoire réellement reliée à l'agent

1. L'agent reçoit un adaptateur MCP Jarvix déclaré dans sa configuration.
2. Il peut demander `recall`, consulter une preuve ou proposer une expérience.
3. Chaque appel est montré dans l'interface, associé à une permission et journalisé.
4. Les écritures (`add`, consolidation, hypothèse) restent soumises à une règle
   d'admission : consentement explicite ou verdict d'outil identifié.

Le premier adaptateur doit exposer seulement des opérations en lecture
(`search`, `get`, `stats`, `recall_cost`). Les écritures viennent ensuite avec
une confirmation par action, jamais avec une permission globale permanente.

### 2. Registre d'arguments

Un argument mémorisé doit avoir : question, prémisses explicites, hypothèses
implicites, définitions, conclusion, objections, contre-exemples, provenance,
résultat d'oracle et statut.

Statuts : `soutenu`, `réfuté`, `contesté`, `indéterminé`. Une contradiction ne
doit pas être effacée ; elle est conservée avec les conditions de résolution.

### 3. Raisonnement philosophique sans faux oracle

Le routeur OP15 choisit au plus trois lentilles par défaut :

- logique : déduction, prémisses absentes et contradictions ;
- linguistique : sens des termes, ambiguïtés et implicatures ;
- épistémique : sources, hypothèses et incertitude ;
- décision : conséquences, risques et principes en conflit ;
- scientifique : test discriminant et résultat attendu.

Une réponse doit séparer **validité** (la conclusion suit-elle la formulation ?),
**fidélité** (la formulation conserve-t-elle le sens ?), **solidité** (les
prémisses sont-elles étayées ?) et **pertinence** (répond-elle à la question ?).
Un solveur ne valide que la validité de la représentation fournie.

## Plan de réalisation

| Étape | Livrable | Critère de passage |
|---|---|---|
| P0 | Profil `PersonaProfile` et mémoire explicite | Déjà livré côté desktop ; identité et souvenir restent distincts. |
| P1 | Adaptateur MCP Jarvix en lecture | Connexion, liste d'outils, rappel et trace UI testés sur un serveur local réel. |
| P2 | Registre d'arguments et états de contradiction | Tests de conservation d'une contradiction et de révision par preuve. |
| P3 | Oracles logique / calcul / tests | Chaque verdict cite l'outil, l'entrée, la sortie et sa portée. |
| P4 | Port Android | Schéma Room + worker de consolidation + reprise d'une session après arrêt forcé. |
| P5 | Mesures | LoCoMo/FOLIO adaptés + cas français adversariaux ; latence, RAM, stockage, batterie. |

## Contrat Android minimal

Le port Android ne réimplémente pas le harnais : il implémente une couche
`ConversationRepository` autour du noyau partagé.

```text
Room
├── persona                identité et version
├── conversation_turn      tour, session, provenance
├── memory_reference       identifiant Jarvix et statut
├── pending_action         permission et reprise
└── argument_revision      prémisses, conclusion, statut, révision

WorkManager
├── consolidation différée
├── indexation locale
└── purge/export demandés par l'utilisateur
```

Une action téléphone doit passer par une capacité à périmètre précis, une
confirmation explicite et un journal de résultat. Aucune boucle permanente ou
permission universelle n'est déduite de WorkManager ou d'AOHP.

## Banc de tests obligatoire

1. Reprendre un projet après redémarrage sans inventer de souvenir.
2. Modifier une préférence, conserver son historique et récupérer la valeur
   récente dans une nouvelle session.
3. Conserver deux sources contradictoires et demander le test de résolution.
4. Distinguer conclusion invalide, indéterminée et contredite (cas FOLIO).
5. Résister à un sophisme reformulé, tout en acceptant une preuve meilleure.
6. Vérifier la stabilité de ton et de limites de l'identité sur des cas comparables.
7. Mesurer P50/P95 de réponse, RAM, taille de base, batterie et chaleur Android.

## Limites non négociables

- Une personnalité stable est un comportement logiciel, pas une preuve de
  conscience subjective.
- Un souvenir n'est pas un fait ; sa provenance et son statut accompagnent sa
  récupération.
- Une vérification formelle ne prouve pas qu'une traduction naturelle était
  fidèle ou qu'un dilemme possède une réponse unique.
- AOHP, les agents Android tiers et les outils MCP ne reçoivent pas de pouvoirs
  système par défaut.
