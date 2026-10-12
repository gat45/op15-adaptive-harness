# Harnais Android OP15 — architecture et TODO exécutable

## Décision d’architecture

L’APK n’embarque pas l’interface Tauri. Il partage des **contrats stables** avec
le noyau Rust desktop et expose une interface Kotlin/Compose native dans
`E:\oneplus\app`. La cible actuelle est `arm64-v8a`, `minSdk 29`,
`compileSdk/targetSdk 36`.

Le socle Android existe déjà : l’application est en Compose, possède Room,
WorkManager et KSP. Le travail est donc de raccorder le harnais, non de
réintroduire une seconde pile de persistance.

```text
Compose UI / ViewModel
        │ intents, état public, annulation
        ▼
AndroidHarnessRepository
        │ DTO versionnés, coroutine / Flow
        ▼
JNI libadaptive_harness.so ──► noyau Rust commun
        │                            │
        │                            ├─ décision, budget, mémoire, traces
        │                            ├─ vérificateurs et permissions
        │                            └─ adaptateur d'inférence
        ▼
Room + WorkManager       llama.cpp CPU → OpenCL → HTP (mesuré)
```

Le flux de production est **CPU d’abord**. OpenCL et HTP ne sont activés que
par configuration explicite, test de compatibilité et trace du backend. Aucun
écran ne doit annoncer « NPU » à partir du seul modèle du téléphone.

## Ce que les sources primaires imposent

- La documentation Android de [llama.cpp](https://github.com/ggml-org/llama.cpp/blob/master/docs/android.md)
  montre une compilation NDK `arm64-v8a`, un modèle fourni par `Uri`/stockage
  privé et une génération Kotlin sous forme de `Flow`. Elle recommande de
  commencer avec un contexte raisonnable pour éviter les pics mémoire.
- La documentation Snapdragon de [llama.cpp](https://github.com/ggml-org/llama.cpp/blob/master/docs/backend/snapdragon/README.md)
  décrit CPU, OpenCL/Adreno et Hexagon comme backends distincts. Le build HTP
  requiert une chaîne dédiée et reste expérimental : il doit être testé sur
  appareil, pas supposé depuis le PC.
- [Room](https://developer.android.com/training/data-storage/room) fournit
  vérification SQL à la compilation et migrations. Il convient aux sessions,
  références mémoire, permissions, traces et index, mais pas aux poids GGUF.
- [WorkManager](https://developer.android.com/develop/background-work/background-tasks/persistent)
  persiste des travaux au redémarrage ; ce n’est pas une permission de faire
  tourner un agent permanent. Utiliser des tâches uniques, contraintes et
  annulation observable pour la consolidation, l’indexation ou l’export.
- [LiteRT-LM](https://developers.google.com/edge/litert-lm) est un comparateur
  d’inférence Android, GPU/NPU et outils. Il ne remplace pas llama.cpp tant
  qu’un même modèle, une même tâche et un même budget n’ont pas été mesurés.

## Contrats communs à créer avant JNI

Le noyau Rust doit produire du JSON/DTO versionné et sans chaîne de pensée
privée :

```text
HarnessRequest v1: session_id, message, model_id, budget, permissions
HarnessEvent v1: started | token | tool_request | evidence | decision | error | finished
HarnessResult v1: public_answer, verification_status, citations, metrics, trace_id
```

`verification_status` ne peut prendre que `verified`, `unverified`, `refuted`
ou `not_applicable`. Une métrique absente est `null` et étiquetée par sa source,
jamais convertie en `0` ou `N/A` positif.

## TODO par étape

### A0 — Inventaire et contrat de sécurité

- [ ] Cartographier les modules Kotlin existants, `binding_jni`, AAR GenieX et
  bibliothèques `.so` effectivement emballées dans chaque flavor.
- [ ] Décider si l’APK OP15 existant est l’hôte officiel du harnais ; sinon
  créer un module Android dédié, sans dupliquer les activités.
- [ ] Écrire le manifeste de permissions : réseau, fichiers via sélecteur
  Android, notifications, microphone éventuel. Interdire `MANAGE_EXTERNAL_STORAGE`
  par défaut et toute commande root depuis une réponse LLM.
- [ ] Écrire la politique données : stockage local, export, suppression, durée
  de conservation et séparation `fait/preference/hypothèse/preuve`.

**Acceptation :** diagramme des composants, liste des capacités, aucune
permission dangereuse ajoutée et revue manuelle documentée.

### A1 — Noyau Rust portable et FFI minimale

- [ ] Extraire de `op15-adaptive-harness` une bibliothèque `cdylib` sans Tauri,
  HTTP desktop ni chemins Windows.
- [ ] Définir les DTO `v1` et les codes d’erreur stables ; sérialisation avec
  taille bornée et UTF-8 validé.
- [ ] Exposer `create_session`, `send`, `cancel`, `get_result`, `destroy`.
- [ ] Garantir qu’aucun callback JNI ne conserve une référence JVM après retour.
- [ ] Compiler `aarch64-linux-android` et tester les DTO côté hôte.

**Acceptation :** une bibliothèque `arm64-v8a` chargeable, test JNI succès,
erreur et annulation ; pas de fuite détectable après 100 sessions.

### A2 — Pont JNI et Flow Kotlin

- [ ] Ajouter un module JNI qui charge la bibliothèque par `System.loadLibrary`.
- [ ] Faire passer les événements Rust dans un `Flow<HarnessEvent>` Kotlin.
- [ ] Utiliser `ViewModel` pour que rotation et reprise n’interrompent pas une
  tâche sans annulation explicite.
- [ ] Ajouter délai, cancellation token et limite de taille de message.

**Acceptation :** écran de test Compose : envoyer, recevoir plusieurs tokens,
annuler, recréer l’activité et visualiser un résultat final cohérent.

### A3 — Inference CPU locale fiable

- [ ] Ajouter un sélecteur Android `ACTION_OPEN_DOCUMENT` pour le GGUF : ne pas
  demander un chemin arbitraire ni scanner le disque entier.
- [ ] Copier/importer le modèle seulement avec consentement, vérifier espace
  libre, empreinte et taille ; stocker une référence de modèle dans Room.
- [ ] Intégrer llama.cpp CPU via bibliothèque conforme aux ABI et headers
  exactement compatibles.
- [ ] Diffuser tokens, TTFT et erreur de chargement ; couper le contexte avant
  l’OOM et expliquer le repli à l’utilisateur.

**Acceptation :** un GGUF choisi par l’utilisateur répond hors réseau ; les
tests couvrent fichier refusé, espace insuffisant, modèle invalide, annulation
et reprise d’application.

### A4 — Sessions, mémoire et identité locale

- [ ] Créer Room : `Session`, `Turn`, `MemoryItem`, `Evidence`, `PermissionGrant`,
  `TraceSummary`, `ModelProfile`.
- [ ] Versionner migrations et index ; chiffrer au minimum les secrets de
  configuration hors de Room.
- [ ] Limiter la mémoire injectée par type, âge, provenance et budget tokens.
- [ ] Ajouter suppression, export et correction d’un souvenir dans l’UI.
- [ ] Mettre la consolidation en `WorkManager` unique, sous contraintes batterie
  et enregistrement, jamais pendant une génération active.

**Acceptation :** redémarrage, migration, conflit de souvenir, suppression et
export sont testés ; aucun contenu LLM n’est mémorisé sans politique d’admission.

### A5 — Raisonnement public, preuves et outils

- [ ] Rendre dans l’UI seulement le plan public : lentilles, hypothèses,
  vérifications, incertitudes et décision — jamais les raisonnements cachés.
- [ ] Ajouter calculateur local et vérificateur de test comme premiers oracles.
- [ ] Définir `EvidenceResolver` : URL/artefact, horodatage, portée, extrait,
  hash, statut et lien avec l’affirmation exacte.
- [ ] Brancher Jarvix/MCP **en lecture** avec timeout, budget et provenance.
- [ ] Placer les écritures de mémoire et les actions fichiers derrière une
  confirmation Android et une permission à portée limitée.

**Acceptation :** une réponse non vérifiée ne devient pas vérifiée sur simple
texte ; outil indisponible, preuve contradictoire et refus utilisateur sont
visibles dans la trace.

### A6 — GPU OpenCL et HTP, uniquement après CPU

- [ ] Construire un paquet expérimental isolé avec la chaîne Snapdragon/NDK
  appropriée ; ne pas remplacer la voie CPU stable.
- [ ] Vérifier chargement des `.so`, backend sélectionné et logs de session.
- [ ] Mesurer sur CPH2747 : RAM, batterie, température, TTFT, préfill, decode,
  succès/échec et backend effectivement employé.
- [ ] Tester le fallback quand HTP est occupé ou échoue ; pas de crash, pas de
  boucle de retry et aucun chiffre HTP attribué à OpenCL/CPU.
- [ ] Comparer CPU, OpenCL et HTP avec modèle, quantisation, contexte et
  température identiques.

**Acceptation :** rapport reproductible contenant commandes, logs, PID/backend,
mesures brutes et échecs. Sans cela, backend conservé sous `experimental`.

### A7 — UX mobile, actions et voix facultative

- [ ] Écrans : conversation, mémoire, preuves, tâches, outils/permissions,
  modèle/performance et paramètres. Garder le composeur visible sans défilement
  ambigu.
- [ ] Ajouter navigation fichiers uniquement via `Storage Access Framework` et
  navigation web via intent/Custom Tab, avec confirmation préalable.
- [ ] Évaluer STT/VAD/TTS après chat fiable ; permission micro, indicateur actif,
  interruption et suppression audio explicites.
- [ ] Ajouter accessibilité, thème, erreurs compréhensibles et mode hors ligne.

**Acceptation :** parcours manuel complet sur téléphone, TalkBack élémentaire,
annulation des actions et aucune capture audio implicite.

### A8 — Qualité, sécurité, publication

- [ ] Tests unitaires Rust/Kotlin, JNI, Room migrations et instrumentation UI.
- [ ] Tests de régression : mémoire contradictoire, injections d’outils,
  déni de permission, OOM, serveur absent, rotation, reprise et annulation.
- [ ] Benchmark scellé `model_only` contre harnais avec coûts énergétiques et
  taux de réussite vérifiée.
- [ ] Générer APK `arm64-v8a`, signer hors dépôt, produire SBOM et documenter
  licences des modèles/bibliothèques.
- [ ] Installer seulement après reconnexion manuelle de l’OP15 ; l’appareil est
  actuellement à considérer indisponible tant que l’ADB n’est pas rétabli.

**Acceptation :** APK installable, tests archivés et matrice de permissions ;
aucune revendication de gain, de mémoire fiable ou de NPU sans mesure associée.

## Priorité de réalisation

`A0 → A1 → A2 → A3 → A4 → A5` forme le premier produit réellement utile et
local. `A6` est une campagne performance, `A7` améliore l’expérience, `A8`
conditionne toute diffusion. LiteRT-LM reste une expérience concurrente après
`A3`, jamais une dépendance ajoutée par défaut.
