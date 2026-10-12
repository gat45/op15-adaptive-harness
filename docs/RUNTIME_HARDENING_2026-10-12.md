# Durcissement runtime — 2026-10-12

## Constats corrigés

1. Les requêtes OpenAI compatibles étaient assemblées sous forme de chaînes.
   Une tabulation issue d'un document collé pouvait donc rendre le JSON invalide.
   Le client utilise désormais `serde_json` pour encoder et décoder toutes les
   requêtes/réponses llama.cpp.
2. Le contexte pouvait grossir sans limite. La demande directe est bornée à
   12 000 caractères, l'historique et la mémoire à 4 000 chacun. Le début et
   la fin sont conservés et la troncature est explicitement signalée au modèle.
3. Le routage profond pouvait afficher toutes les lentilles. Il en garde au
   plus cinq ; le routage normal reste limité à trois.
4. Le journal du dernier tour décrit maintenant les étapes réellement faites :
   contexte, mémoire, modèle, oracle et décision. Il ne révèle pas une chaîne
   de pensée privée.
5. Les métriques retournées par llama.cpp sont conservées quand elles existent
   (préfill, débit préfill/génération, tokens et cache), sans les présenter
   comme une mesure GPU attribuée au modèle.

## Vérification : portée et limites

- Le calculateur confirme une expression numérique précise et compare son
  résultat à la réponse produite.
- Le test de projet exécute `cargo test` après confirmation. Il est une
  expérience locale, pas un oracle de vérité générale.
- La recherche ouvre le navigateur. Une source ne devient mémoire durable
  qu'après lecture et validation humaine de sa portée ; ce statut est
  `SourceReviewed`, non une preuve automatique de toute réponse.
- Aucune réponse ordinaire ne peut être affichée comme « vérifiée » en
  l'absence d'un critère et d'un oracle liés à cette réponse.

## Angle mort restant volontaire

Le harnais ne doit pas transformer automatiquement une page web trouvée en
fait fiable. Une vérification factuelle générale demanderait : récupération
de sources, évaluation de leur autorité/date, extraction de passages,
contradiction entre sources et un protocole d'entailment évalué sur un jeu
scellé. C'est un chantier expérimental distinct, non un bouton à simuler.

## Références techniques appliquées

- llama.cpp expose les timings de réponse et `/metrics` seulement lorsque le
  serveur est démarré avec `--metrics` :
  <https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md>
- Phoenix décrit une trace comme la hiérarchie d'appel LLM, récupération et
  outils, avec latence et évaluation séparées :
  <https://arize.com/docs/phoenix/tracing>
- Les travaux de budget adaptatif motivent le plafonnement des lentilles ; ils
  ne démontrent pas encore un gain sur ce projet avant benchmark à budget égal :
  <https://arxiv.org/abs/2604.14853>

## Prochaine évaluation réelle

Constituer un jeu versionné de demandes courtes, ambiguës, causales,
numériques et factuelles. Chaque cas doit donner un oracle, un budget maximal,
un coût mesuré et un verdict. Comparer : LLM seul, harnais sans routage, puis
harnais complet, à modèles et budgets identiques.
