# Architecture mémoire — Adaptive Harness

## But et limite

La mémoire rend un agent plus cohérent ; elle ne prouve ni conscience, ni
intention subjective. Toute donnée injectée dans une réponse doit indiquer son
origine et son statut de vérification.

## Les cinq niveaux

| Niveau | Durée | Contenu | Support | Injection dans le modèle |
|---|---|---|---|---|
| Mémoire de travail | tour courant | demande, outil, budget, réponse en cours | RAM | entière, bornée |
| Historique récent | session | derniers échanges fidèles | stockage local chiffrable | 12 tours maximum |
| Épisodique | durable | événement daté et son résultat | SQLite append-only | seulement les épisodes pertinents |
| Sémantique | durable | faits, préférences, relations et contraintes | SQLite + index hybride | faits actifs, avec provenance |
| Procédurale | durable | stratégies, échecs et leçons vérifiées | registre versionné | règle concise, jamais comme fait utilisateur |

`Codebase Memory MCP` reste une mémoire de code et de projets : elle ne doit
pas être confondue avec la mémoire personnelle de la conversation.

## Contrat d'un souvenir

Chaque entrée persistante doit posséder au minimum :

```text
id, kind, content, subject, source_turn, source_tool,
observed_at, valid_from, valid_until,
confidence, verification, supersedes, consent_scope, retention_policy
```

- `observed_at` : moment où le système l'a appris ;
- `valid_from` / `valid_until` : période pendant laquelle le fait est censé
  être vrai ;
- `verification` : `user_asserted`, `tool_observed`, `independently_verified`,
  `inferred` ou `retracted` ;
- `supersedes` relie une révision au souvenir ancien, qui reste auditable ;
- `consent_scope` et `retention_policy` permettent l'oubli demandé par
  l'utilisateur et interdisent la conservation implicite des données sensibles.

## Écriture contrôlée

1. Sauvegarder l'épisode brut, sans l'altérer.
2. Extraire des candidats à mémoire avec un type et une provenance.
3. Dédupliquer par sujet, relation et intervalle temporel.
4. Rechercher une contradiction : nouvelle préférence, fait expiré, source plus
   fiable ou simple ambiguïté.
5. Créer une révision ou conserver les deux versions avec leurs périodes de
   validité ; ne jamais écraser silencieusement.
6. Promouvoir en sémantique/procédurale seulement après confirmation explicite,
   outil fiable, ou évaluation reproductible.

Une réflexion libre du modèle est un **candidat**, pas une vérité. Une leçon
procédurale doit être reliée à un verdict de test, suivant le principe de
Reflexion.

## Lecture et contexte

Pour une demande, le contrôleur construit un paquet de contexte borné :

```text
persona stable
 + état de tâche et budget
 + historique récent fidèle
 + souvenirs sémantiques actifs
 + épisodes pertinents
 + leçons procédurales vérifiées
 + conflits et incertitudes explicites
```

Le classement combine pertinence lexicale/vectorielle, récence, importance,
fiabilité de source et validité temporelle. Le résultat cite les identifiants
des souvenirs utilisés. En cas de conflit non résolu, l'agent demande la
précision manquante au lieu de choisir arbitrairement une version.

## Consolidation et oubli

- Une tâche asynchrone compacte les épisodes anciens en résumés **liés aux
  épisodes originaux**, jamais en remplacement destructif.
- Les préférences sont réévaluées à chaque assertion récente ; les faits
  temporaires expirent.
- L'oubli est un tombstone auditables, suivi d'une purge selon la politique de
  rétention.
- Les sauvegardes et exportations sont séparées par utilisateur/profil.

## Mesures obligatoires

1. rappel factuel, temporel, multi-saut et résumé sur LoCoMo ;
2. précision de récupération : un bon texte final ne masque pas une mauvaise
   mémoire récupérée ;
3. taux de conflits détectés, révisions correctes et abstentions utiles ;
4. fidélité : chaque fait cité doit être traçable à un épisode ou une source ;
5. coût : tokens injectés, p95 de récupération, taille disque et latence ;
6. non-régression : même jeux d'essai, budget et modèle pour les variantes.

## État du dépôt et ordre de mise en œuvre

L'UI fournit l'historique récent local (20 tours stockés, 12 réinjectés) et un
premier magasin long-terme explicite. Il est append-only, synchronisé sur disque,
associe chaque souvenir à sa provenance `user_asserted`, récupère les entrées par
termes et date, et oublie par tombstone. Aucun message du chat n'est promu
automatiquement en souvenir. Le fichier est actuellement un TSV auditable dans le
profil de l'application ; il sera migré vers SQLite avant les index et migrations.

1. migration `memory_store.rs` : SQLite, migrations, épisodes append-only et tombstones.
2. `memory_retrieval.rs` : BM25 + temps + provenance, sans dépendance vectorielle
   obligatoire.
3. `memory_policy.rs` : extraction de candidats, consentement, conflits et
   promotion contrôlée.
4. UI : corriger et exporter chaque souvenir ; la liste et l'oubli explicite sont présents.
5. `tests/memory_*` : oubli, conflit, source, reprise après crash et LoCoMo.

## Références primaires

- MemGPT / contextes hiérarchiques : https://arxiv.org/abs/2310.08560
- Generative Agents / importance, récence, réflexion : https://arxiv.org/abs/2304.03442
- LoCoMo / évaluation conversationnelle longue : https://arxiv.org/abs/2402.17753
- Mem0 / extraction et consolidation : https://arxiv.org/abs/2504.19413
- Reflexion / leçons épisodiques vérifiées : https://arxiv.org/abs/2303.11366
- Zep / graphe temporel : https://arxiv.org/abs/2501.13956
- MemConflict / conflits de mémoires : https://arxiv.org/abs/2605.20926
