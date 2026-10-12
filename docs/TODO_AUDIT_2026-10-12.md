# TODO auditée — état réel au 12 octobre 2026

Ce document consolide les échanges, les audits versionnés, le code actuel et
les sources listées dans `SOURCES.md`. Les cases ne sont cochées que lorsqu'un
comportement est implémenté **et** testé au bon niveau.

## État factuel

| Domaine | État | Preuve / limite |
|---|---|---|
| Noyau Rust | Partiel, testé | Budget, permissions, mémoire locale, routeur, persona et vérificateur arithmétique ont des tests unitaires. |
| llama.cpp non-streaming | Fonctionnel localement | Test E2E disponible, mais dépend du serveur local. |
| Streaming UI | Implémenté, à valider E2E | Le code SSE et l'annulation existent ; aucune campagne UI/serveur réelle n'est encore archivée. |
| Mémoire de conversation | Partielle | 20 tours dans `localStorage`, souvenirs longue durée explicites ; pas de journal de session durable ni de consolidation. |
| Jarvix Memory / MCP | Non raccordé au harnais | Le service peut être démarré et indexer depuis l'UI, mais le modèle ne découvre ni n'appelle ses outils. |
| Vérification | Très limitée | Calculateur, tests Rust, concordance textuelle HTTPS et source examinée ; pas de vérification générale d'une réponse. |
| Raisonnement / spécialistes | Orientations seulement | Lentilles et rôles produisent un contexte ; pas de registre d'arguments ni solveur formel exécuté. |
| Benchmarks | Non exécutés | `audit/BENCHMARK_RESULTS.json` vaut `NOT_RUN`. |
| Android | Spécification | Aucun JNI, aucune bibliothèque `arm64-v8a`, aucune APK issue de ce dépôt. |

## P0 — fiabilité avant nouvelles capacités

Références de protocole : [τ-bench](https://arxiv.org/abs/2406.12045) pour
l'état final des actions, [AgentDojo](https://arxiv.org/abs/2406.13352) pour
les injections d'outils, [SWE-rebench](https://arxiv.org/abs/2505.20411) pour
des tâches renouvelées, et [Judging the Judges](https://arxiv.org/abs/2406.07791)
pour évaluer le biais d'un juge LLM. Elles guident les tests ; elles ne
constituent pas une preuve de performance OP15.

- [ ] **Construire puis lancer le dernier exécutable desktop.** L’instance qui
  était ouverte pendant les essais précédents était un ancien binaire. Critère :
  numéro de commit/version visible dans l’UI, GGUF sélectionnable, chat et
  identité testés dans le même exécutable.
- [ ] **Tester le streaming contre le serveur llama.cpp réel.** Cas : premier
  fragment, réponse complète, réponse ne contenant que `reasoning_content`,
  erreur HTTP, fermeture serveur, arrêt utilisateur. Critère : jamais de texte
  caché ni de tour assistant incomplet marqué comme terminé.
- [ ] **Rendre le lecteur SSE robuste au transfert HTTP chunked.** Le lecteur
  actuel lit des lignes TCP et suppose que les événements SSE restent lisibles
  au travers du découpage HTTP. Critère : test avec fragments coupés à chaque
  octet et JSON multi-lignes ; passer par un vrai décodeur HTTP/SSE ou un client
  éprouvé avant exposition hors machine locale.
- [ ] **Conserver un résultat de contrôleur pour un tour streamé.** Aujourd’hui
  le chemin stream n’alimente pas `TaskResult`, donc l’écran Agent ne montre pas
  décision, trace, budget ni verdict du même tour. Critère : même contrat
  d’événements pour flux et réponse bloquante.
- [ ] **Corriger les documents périmés.** `docs/DESKTOP_RELEASE.md` dit encore
  que streaming et annulation sont absents. Le registre d’arguments annoncé dans
  `RESEARCH_INTEGRATION_ANDROID_MEMORY_LOGIC.md` reste une spécification.
  Critère : chaque fonctionnalité possède `implémentée`, `expérimentale` ou
  `prévue`, avec test ou issue associée.
- [ ] **Éviter les collisions d’identifiant mémoire.** `MemoryStore` dérive
  l'identifiant du milliseconde courant. Deux écritures parallèles peuvent se
  heurter. Critère : UUID ou compteur atomique, test concurrent et journal
  append-only récupérable après crash.

## P1 — mémoire, personnalité et continuité utiles

- [ ] **Créer un journal de sessions durable.** Remplacer le seul `localStorage`
  par une base locale versionnée : tour, session, horodatage, modèle, contexte,
  statut de vérification et suppression utilisateur. Critère : reprise après
  redémarrage sans dépendre du navigateur.
- [ ] **Ajouter une admission de mémoire explicite.** Distinguer préférence,
  épisode, décision, hypothèse, leçon vérifiée et source. Critère : aucune sortie
  LLM ne devient souvenir durable sans consentement ou oracle identifié.
- [ ] **Relier Jarvix Memory par un adaptateur MCP en lecture.** Commencer par
  `search`, `get`, `stats`, `recall_cost`; afficher outil, arguments, résultat,
  coût et provenance. Critère : un rappel réel change le contexte d’un tour et
  un serveur indisponible ne provoque pas d’invention.
- [ ] **Ajouter les écritures MCP sous permissions.** Critère : chaque écriture
  a une capacité limitée, une confirmation, un résultat durable et une reprise
  après interruption ; aucune permission « tout écrire ».
- [ ] **Évaluer continuité et stabilité.** Cas : préférence modifiée, rappel
  ancien, sujet interrompu, souvenir contradictoire, faux souvenir injecté,
  ton cohérent. Critère : jeux scellés, provenance de chaque score.

## P1 — vérification, logique et outils

- [ ] **Remplacer la concordance web brute par une preuve de source typée.**
  Le contrôle HTTPS actuel teste seulement une chaîne dans le contenu reçu.
  Critère : URL finale, horodatage, empreinte, extrait borné, statut HTTP,
  redirections sûres et portée de la conclusion conservés ; pas de verdict de
  « vérité générale ».
- [ ] **Introduire `EvidenceResolver` et `ToolOutcomeVerifier`.** Critère : le
  verdict est lié à la demande, la réponse, l’artefact et ses critères ; une
  preuve d’un autre sujet ne peut pas promouvoir une réponse.
- [ ] **Construire le registre d’arguments.** Schéma : question, définitions,
  prémisses, implicites, conclusion, objections, contre-exemples, sources,
  révisions et statut `soutenu/réfuté/contesté/indéterminé`. Critère : deux
  sources contradictoires survivent et déclenchent une question ou un test.
- [ ] **Ajouter un solveur comme oracle optionnel.** Commencer par calcul et
  contraintes simples ; Logic-LM/FOLIO et Lean restent des références. Critère :
  présenter séparément fidélité de la formalisation, validité et solidité des
  prémisses.
- [ ] **Tester résistance aux sophismes, ambiguïtés et paradoxes.** Critère :
  le harnais n’applique pas mécaniquement la logique classique au paradoxe du
  menteur, à la sorite, à Newcomb ou à un dilemme moral.
- [ ] **Finir les actions locales sécurisées.** Validation d’arguments,
  capacités signées, post-conditions, transaction durable et réconciliation
  après crash. Critère : pas de retry aveugle après une action à effet de bord.

## P2 — intelligence adaptative mesurable

- [ ] **Relier le routeur à une fonction d’utilité apprise.** Les lentilles et
  spécialistes sont aujourd’hui des règles ; ils ne comparent pas coût attendu,
  chance de réussite et gain d’information. Critère : comparaison budget égal
  contre stratégie fixe.
- [ ] **Calibrer le modèle de soi.** Mesurer Brier score, ECE, couverture et
  risque sélectif par famille de tâches. Critère : l’abstention est justifiée
  par des données, pas par un seuil arbitraire.
- [ ] **Tracer la causalité de bout en bout.** Contexte → mémoire → décision →
  outil → observation → verdict → changement de stratégie. Critère : une trace
  exportée ne contient ni chaîne de pensée privée ni justification inventée.
- [ ] **Promouvoir les stratégies seulement après expériences.** Shadow mode,
  jeux de régression, rollback et revue. Critère : aucune stratégie ne change
  sur une observation non vérifiée.

## P2 — performances et observabilité

- [ ] **Mesurer réellement TTFT, préfill, decode et annulation.** Le serveur
  peut fournir certaines métriques ; l’UI doit indiquer la source et l’absence
  de donnée. Critère : P50/P95, modèle, paramètres et port archivés par essai.
- [ ] **Attribuer proprement CPU/GPU/VRAM.** Le PID lié au port et `nvidia-smi`
  sont un progrès mais ne couvrent ni plusieurs GPU ni tous les backends.
  Critère : instantané avant/après, PID, GPU, mémoire processus et fallback.
- [ ] **Construire un benchmark reproductible.** Splits `opt/select/regression/final`,
  oracles scellés, baseline `model_only`, stratégies fixes et adaptive. Critère :
  coût par réussite vérifiée et tous les échecs publiés.
- [ ] **Mesurer CPU/GPU/NPU avec preuve backend.** Les chiffres HTP, Vulkan ou
  GPU ne doivent pas être attribués au harnais sans traces du processus et du
  backend réellement actif.

## P3 — produit desktop et expérience utilisateur

- [ ] **Test E2E des écrans.** Modèle GGUF, chat Enter/Shift+Enter, streaming,
  stop, mémoire, source, tests, MCP, Explorateur, erreur serveur. Critère :
  parcours automatisé et capture d’échec.
- [ ] **Réduire les écrans trop denses.** Critère : chat, agent, preuves,
  mémoire et outils restent utilisables sans masquer le composeur ni imposer
  une navigation décorative.
- [ ] **Ajouter voix seulement après le chat fiable.** STT/VAD/TTS et
  interruption doivent rester adaptateurs optionnels. Critère : consentement
  micro, latence mesurée, arrêt de parole testé, aucune capture permanente.

## P4 — Android / OnePlus 15

- [ ] **Créer la bibliothèque Rust `aarch64-linux-android`.** Critère : build
  reproductible, API JSON/DTO versionnée et tests hôte.
- [ ] **Créer le pont JNI et l’écran Compose.** Critère : un événement réel du
  contrôleur est rendu, puis une tâche locale est annulable et reprend après
  recréation d’activité.
- [ ] **Ajouter Room et WorkManager.** Critère : sessions, identité, références
  mémoire et actions en attente survivent ; les travaux différés ne sont jamais
  présentés comme une boucle permanente garantie.
- [ ] **Tester l’inférence locale et les fallbacks.** CPU d’abord, puis Vulkan/
  HTP uniquement avec preuve on-device. Critère : RAM, température, batterie,
  latence et backend observés pour le processus APK.
- [ ] **Construire et installer l’APK.** Critère : APK `arm64-v8a`, tests sur
  CPH2747 rebranché manuellement et aucun claim HTP sans mesure.

## Critères transversaux de clôture

Une case n’est clôturée que si :

1. le code est relié à une interface ou API réellement appelable ;
2. un test couvre succès, erreur et annulation quand ils existent ;
3. la trace identifie données, outils et périmètre de vérification ;
4. un résultat mesuré est séparé d’une estimation ou d’une source externe ;
5. les permissions et données personnelles sont réversibles, exportables et
   supprimables à la demande.
