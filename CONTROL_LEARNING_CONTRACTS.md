# Les douze mécanismes de contrôle : contrats et tests

## Principe d'intégration

Les composants ne sont pas reliés par des prompts, mais par des contrats de
données, des post-conditions et des oracles. Un module peut seulement influencer
le suivant via des sorties versionnées et auditées. L'assemblage proposé reste
une hypothèse d'ingénierie, non une démonstration de conscience.

| Brique | Contrat dans le harnais | Preuve nécessaire | Référence |
| --- | --- | --- | --- |
| Modèle de soi | capacités, limites, ressources, permissions et incertitude par tâche | calibration hors échantillon | https://doi.org/10.1016/j.neunet.2025.108131 |
| Métaraisonnement | décision continuer, vérifier, déléguer ou arrêter | gain à budget égal, faux arrêts | https://people.eecs.berkeley.edu/~russell/papers/metareasoning.pdf |
| Modèle du monde | prédiction bornée d'une post-condition d'action | erreur prédiction/réalité | https://github.com/danijar/dreamerv3 |
| Attention sélective | score de priorité des observations vers mémoire active | gain vs FIFO/récence | https://pmc.ncbi.nlm.nih.gov/articles/PMC3081809/ |
| Mémoire causale | cause supposée, verdict et effet du réemploi | changement de stratégie vérifiable | https://github.com/noahshinn/reflexion |
| Apprentissage continu | candidate et régression par compétence | absence d'oubli sur `regression` | https://github.com/ContinualAI/avalanche |
| Compétences | préconditions, exécutable, tests, récupération | succès reproductible | https://github.com/MineDojo/Voyager |
| Incertitude calibrée | intervalle/score et politique d'abstention | Brier, ECE, couverture-risque | https://arxiv.org/abs/2107.07511 |
| Vérification indépendante | oracle externe par type d'action | verdict traçable et reproductible | https://arxiv.org/abs/2305.11738 |
| Allocation de ressources | coût total estimé et runtime sélectionné | succès/coût par backend | https://github.com/lm-sys/RouteLLM |
| Objectifs autonomes | but, priorité, expiration et raison d'abandon | conflit résolu et audit | https://soar.eecs.umich.edu/ |
| Contrôle de l'autonomie | permissions, limites, validations et récupération | action refusée/reprise sûre | https://aiagentindex.mit.edu/ |

## 1. `SelfModel` : représentation opérationnelle

Le modèle de soi n'est ni un texte de rôle ni une autobiographie générée. Il
enregistre, par version d'environnement et classe de tâche : capacité estimée,
incertitude, score de calibration, erreurs récurrentes, mémoire disponible,
ressources CPU/GPU/NPU, outils réellement autorisés et contraintes actives.

Une prédiction telle que « risque d'échec élevé » doit être confrontée à un
verdict ultérieur. Sans historique et mesure de calibration, elle est classée
comme intuition non calibrée. MUSE est une référence de recherche pour le
modèle de soi, pas une dépendance introduite ici :
https://doi.org/10.1016/j.neunet.2025.108131.

## 2. Modèle du monde borné

Le harnais ne cherche pas à reproduire DreamerV3 ni à apprendre un monde
général. Avant une action outillée, il prédit seulement des conséquences
observables : artefact attendu, coût, risque, état de permission et
post-condition. Après action, il compare la prédiction à l'oracle et stocke
l'écart causal. DreamerV3 apprend un modèle de monde depuis l'expérience et
l'utilise pour entraîner une politique sur des trajectoires imaginées ; cette
propriété est transposée ici de manière délibérément restreinte.

Référence : https://github.com/danijar/dreamerv3

## 3. Contrôleur de stratégies réversible

Une stratégie qui échoue régulièrement produit une **candidate**, jamais une
auto-modification. La candidate précise le problème, la politique remplacée,
le changement borné, les jeux `opt`/`select`/`regression`/`final`, les métriques
et le rollback. Le governor ne peut promouvoir que si les critères sont
satisfaits hors du jeu d'optimisation. Toute régression critique impose
`ROLLBACK`.

Le cycle est : détecter un écart vérifié, diagnostiquer sous hypothèses,
proposer une candidate, tester, comparer au baseline, promouvoir ou annuler.
Il relie le contrôle de MIDCA, la mémoire corrective de Reflexion et les tests
de compétences de Voyager, sans leur attribuer une capacité commune non
démontrée.

## Ordre de réalisation

1. `SelfModel` calibré et lecture seule ;
2. prédicteur de post-conditions pour un petit ensemble d'outils sûrs ;
3. contrôleur de candidates avec rollback ;
4. attention, mémoire causale, compétences et routage, une brique à la fois ;
5. apprentissage continu seulement si une candidate dépend d'un modèle appris.

Avalanche convient comme source de protocoles de continual learning et de
comparaison de régression, mais il ne doit être installé que dans un
environnement expérimental isolé : https://github.com/ContinualAI/avalanche.

## Limite scientifique

Ces contrats peuvent établir la qualité de l'intégration, la fidélité des
traces, la calibration, la continuité opérationnelle et l'autonomie sous
contraintes. Ils n'établissent ni une expérience subjective ni la conscience.
Le cadre de prudence reste : https://arxiv.org/abs/2308.08708.
