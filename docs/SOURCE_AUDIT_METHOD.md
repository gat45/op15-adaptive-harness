# Méthode reproductible pour lire les 198 sources

## Objectif

Lire toutes les sources ne signifie pas installer 198 projets, ni résumer leurs
README avec un modèle. Une entrée est considérée **lue et décidée** seulement si
sa preuve primaire est archivée et si une décision d'architecture est prise.

## Registre obligatoire

Chaque URL de `SOURCES.md` possède une ligne dans un export `JSONL` ou `CSV` :

```text
url, canonical_url, family, source_type, checked_at, http_status,
primary_evidence, licence, activity, platforms, permissions, data_risk,
test_or_reproduction, decision, rationale, reviewer, next_review
```

`decision` vaut uniquement `retain_contract`, `compare`, `reject_dependency`,
`revalidate` ou `reference_only`. Une absence de licence, une redirection ou une
page inaccessible est un motif `revalidate`, jamais une preuve d'absence.

## Pipeline en cinq passes

1. **Inventaire figé.** Extraire les URL uniques de `SOURCES.md`, les canoniser
   sans supprimer les doublons sémantiquement différents (article, dépôt,
   documentation et issue sont des preuves distinctes).
2. **Métadonnées publiques.** Pour chaque dépôt GitHub : existence, état
   archivé, licence, branche, date d'activité, langage et fichier de sécurité.
   Utiliser un jeton GitHub personnel en lecture seule pour éviter la limite
   anonyme ; ne jamais le committer.
3. **Lecture primaire.** Lire README, licence, documentation officielle,
   publication ou spécification. Extraire le rôle déclaré, les prérequis, les
   permissions, les données traitées et le protocole d'évaluation. Les blogs,
   catalogues et profils restent secondaires.
4. **Test minimal isolé.** Seulement pour les candidats `retain_contract` ou
   `compare` : installation dans un environnement jetable, exemple officiel,
   test d'erreur et désinstallation. Aucun accès aux conversations, fichiers ou
   appareils réels sans capacité limitée et consentement explicite.
5. **Décision et révision.** Ajouter la ligne signée au registre, lier le
   résultat à une issue/contrat et fixer la date de revalidation. Une source ne
   devient une dépendance qu'après compatibilité de licence, test et analyse des
   risques.

## Lots et ordre

La lecture est organisée en 8 lots de 25 URL maximum ; chaque lot produit un
commit distinct et un export de preuve. L'ordre minimise les risques :

1. agents, MCP, navigateur, fichiers et projets Android ;
2. mémoire et RAG ;
3. serveur LLM, backends, GPU/NPU et outils de profilage ;
4. logique, solveurs, arguments et benchmarks ;
5. orchestration, routeurs et optimisation ;
6. architectures cognitives et publications ;
7. catalogues, profils et articles secondaires ;
8. liens cassés, doublons et revalidation finale.

## Outil de collecte

[`tools/audit_sources.ps1`](../tools/audit_sources.ps1) prépare un manifeste
machine-lisible. Il ne conclut pas sur la qualité d'un projet et n'installe rien.
Il limite volontairement les lots. Avec `GH_TOKEN` défini localement, il peut
compléter les métadonnées GitHub sans exposer le jeton dans le dépôt.

Exemple, depuis la racine du dépôt :

```powershell
$env:GH_TOKEN = 'jeton_lecture_seule_local'
.\tools\audit_sources.ps1 -BatchSize 25 -OutputPath audit\source-manifest.jsonl
```

Le fichier de sortie est volontairement ignoré par Git : chaque résultat doit
être relu, synthétisé et versionné dans une décision, pas publié comme vérité
brute. Sans `GH_TOKEN`, l'outil conserve l'URL et marque les métadonnées GitHub
comme `not_collected` au lieu d'inférer un faux `404`.

## Définition de fini

L'audit des 198 URL est terminé uniquement lorsque :

- 198 lignes ont un état, une date et une source primaire ou un motif clair ;
- toutes les dépendances potentielles ont licence, risque et test minimal ;
- toutes les références scientifiques distinguent résultat publié et gain OP15
  mesuré ;
- les décisions d'intégration sont couvertes par un test de régression ;
- les URL secondaires ne servent jamais de preuve unique.
