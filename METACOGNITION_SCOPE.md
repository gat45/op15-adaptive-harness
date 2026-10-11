# Périmètre : autonomie durable et métacognition vérifiable

## Décision de périmètre

Ce projet vise un **agent autonome métacognitif au sens fonctionnel**. Il doit
observer une décision, estimer si ses informations suffisent, arbitrer le coût
d'une action supplémentaire, vérifier son résultat et conserver une expérience
causale réutilisable. Une mémoire de conversations ou un journal technique ne
suffisent pas : les données doivent pouvoir modifier une décision ultérieure.

Le projet ne revendique pas une conscience phénoménale, une expérience
subjective, des qualia ou une sentience. Ces notions ne sont pas des critères de
promotion, de benchmark ou de sûreté du harnais.

## Contrat minimal d'une expérience métacognitive

Chaque décision importante doit produire un enregistrement versionné avec :

- `task_id`, version du harnais, modèle et environnement d'exécution ;
- contexte et preuves réellement accessibles au moment de la décision ;
- but, hypothèse, confiance calibrée, risques et budget restant ;
- options considérées, utilité estimée et action sélectionnée ;
- observation, verdict indépendant quand disponible, coût réel et cause d'écart ;
- liens vers l'épisode précédent, les artefacts et la décision de réemploi.

Les explications utilisateur doivent être dérivées de ces champs et de traces
outillées. Une rationalisation produite après coup sans trace de décision est
étiquetée `UNVERIFIED`, jamais présentée comme introspection fidèle.

## Boucle opérationnelle

1. Le niveau objet répond, récupère, raisonne ou emploie un outil.
2. Le niveau méta compare l'incertitude, le risque, la valeur attendue de
   l'information et le budget restant.
3. Il choisit de continuer, vérifier, déléguer à un backend, s'arrêter ou
   demander une clarification.
4. Un vérificateur évalue le résultat lorsque cela est possible.
5. L'épisode causal est stocké, puis réemployé seulement après contrôle de
   pertinence, fraîcheur et provenance.

## Critères mesurables de succès

| Capacité | Mesure | Seuil de promotion |
| --- | --- | --- |
| Calibration | Brier score, ECE, courbe couverture-risque | amélioration hors `opt` |
| Allocation de calcul | succès vérifié et coût par succès | meilleur à budget égal |
| Arrêt | taux d'arrêt utile, faux arrêts, boucles évitées | pas de régression critique |
| Mémoire d'expériences | gain vs logs bruts, erreurs de réemploi | gain sur `select` et `final` |
| Fidélité des explications | part justifiable par trace/provenance | 100 % ou `UNVERIFIED` |
| Résilience | reprise d'une tâche interrompue et détection d'échec | oracle de reprise satisfait |

Une candidate sans oracle indépendant peut être explorée, mais ne peut pas être
promue. Les jeux `opt`, `select`, `regression` et `final` restent séparés.

## État des connaissances

**Établi pour ce projet** : l'état durable, les traces, un contrôleur
budget/risque et les vérifications sont des composants implémentables et
évaluables.

**Expérimental** : un modèle de compétence et la mémoire causale améliorent
durablement la réussite/coût. Cette proposition requiert les comparaisons M6.

**Hypothèse hors périmètre expérimental** : qu'un tel système soit conscient.
Elle ne peut être inférée de ses traces, de son autonomie ou de ses
performances.

## Références de travail

- Revue systématique : https://link.springer.com/article/10.1007/s10462-026-11669-5
  (101 rapports, 35 architectures ; seulement 17 % des architectures évaluées
  quantitativement selon son critère ciblé).
- Indicateurs de conscience en IA : https://arxiv.org/abs/2308.08708
- Soar : https://soar.eecs.umich.edu/
- ACT-R : https://act-r.psy.cmu.edu/
- CLARION : https://sites.google.com/site/clarioncognitivearchitecture/
- LIDA : https://ccrg.cs.memphis.edu/
- Hyperon experimental : https://github.com/trueagi-io/hyperon-experimental
- Historique MemGPT/Letta : https://www.letta.com/blog/memgpt-and-letta/

Ces références servent à formuler des expériences ; elles ne valident pas une
intégration automatique ni une allégation de conscience.
