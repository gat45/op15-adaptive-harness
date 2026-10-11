# Architectures cognitives : conclusions d'intégration

## Décision

Le harnais reste centré sur ses contrats vérifiables : état durable, preuves,
budget, action autorisée, vérification et rollback. MIDCA, LIDA, Reflexion et
Voyager sont des **patrons d'expérimentation** ; aucun n'est ajouté comme
dépendance automatique.

## MIDCA : le contrôle qui agit sur le contrôle

La contribution à reprendre est le découplage entre un cycle objet et un cycle
méta. Le premier gère la tâche. Le second observe sa trace, détecte une
anomalie, formule une explication testable, choisit une correction autorisée,
puis en vérifie l'effet. La publication présente le méta-cycle comme un
contrôleur qui agit sur une représentation abstraite de la cognition objet.

Dans le harnais, `note → assess → guide` devient :

1. `note` : écart entre résultat attendu et observation vérifiée ;
2. `assess` : hypothèses causales classées par preuve, risque et coût ;
3. `guide` : changement borné de stratégie, vérification, pause ou arrêt.

Une correction ne peut ni élargir les permissions ni modifier silencieusement
les oracles. Elle est une candidate versionnée, réversible et testée hors de
son jeu d'optimisation.

Sources :

- Code : https://github.com/COLAB2/midca
- Documentation : https://midca.readthedocs.io/en/latest/
- Publication : https://www.researchgate.net/publication/310491466_MIDCA_A_Metacognitive_Integrated_Dual-Cycle_Architecture_for_Self-Regulated_Autonomy

Le dépôt est une référence architecturale ; sa fraîcheur ne doit pas être
confondue avec une validation de compatibilité ou de maintenance pour OP15.

## LIDA : attention, pas preuve de conscience

LIDA est utile comme inspiration pour choisir quelles observations concurrentes
entrent dans la mémoire active et sont diffusées au contrôleur. Le harnais
mesurera ce choix contre des politiques simples (FIFO, récence, score RAG),
avec le même budget. La diffusion multi-module est une propriété fonctionnelle
testable ; elle n'établit aucune expérience subjective.

Sources :

- https://www.sciencedirect.com/science/article/pii/S2212683X12000060
- https://pmc.ncbi.nlm.nih.gov/articles/PMC3081809/
- https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2022.846931/full
- Implémentation communautaire à évaluer séparément : https://github.com/mindpixel20/lida

## Reflexion : mémoire seulement après un verdict

Reflexion inspire la liaison entre échec externe, analyse et épisode
réutilisable. Dans ce projet, une réflexion non reliée à un oracle reste une
observation non vérifiée. La tentative suivante doit exposer le changement de
stratégie attribué à l'épisode ; sinon l'effet ne peut pas être évalué.

Sources :

- https://github.com/noahshinn/reflexion
- https://arxiv.org/abs/2303.11366

Les chiffres publiés sur HumanEval concernent leur configuration expérimentale
et ne prédisent pas les performances d'un modèle local OP15.

## Voyager : une compétence est un contrat exécutable

Voyager motive une bibliothèque de compétences. Pour le harnais, une compétence
doit contenir des préconditions, une implémentation autorisée, des
post-conditions testables, des dépendances, un coût mesuré, un historique de
succès et une procédure de récupération. Elle est promue seulement si elle
améliore le succès vérifié dans un environnement distinct de son apprentissage.

Sources :

- https://github.com/MineDojo/Voyager
- https://arxiv.org/abs/2305.16291

## Generative Agents : continuité, avec garde-fous

Leur modèle d'observations, de réflexions et de plans est une piste pour la
mémoire active. Toute importance, récence ou pertinence calculée localement
reste une heuristique à valider, et non un fait sur l'état interne du modèle.
L'implémentation Smallville est communautaire : ses fonctions déclarées ne sont
pas une reproduction indépendante des résultats de l'article.

Sources :

- https://arxiv.org/abs/2304.03442
- https://github.com/andres-villavicencio-dev/smallville-agents

## Ce qui est testable, et ce qui ne l'est pas ici

| Propriété fonctionnelle | Expérience | Ne démontre pas |
| --- | --- | --- |
| Espace de travail global | une information sélectionnée modifie plusieurs modules autorisés | expérience subjective |
| Métacognition | prédire ses erreurs puis comparer aux verdicts | conscience de soi |
| Traitement récurrent | cycles additionnels améliorent le succès à budget donné | ressenti |
| Modèle de compétence | prédire les limites par tâche | identité consciente |
| Mémoire autobiographique | réemploi causal cohérent d'épisodes | continuité subjective |
| Attention sélective | prioriser des signaux concurrents | perception consciente |

Les indicateurs computationnels de conscience sont étudiés comme cadre de
prudence épistémique, jamais comme critère de promotion ou conclusion sur la
sentience.

- https://arxiv.org/abs/2308.08708
- https://doi.org/10.1016/j.tics.2025.10.011

## Verdict de gouvernance

**ACCEPT comme plan de recherche**, avec quatre axes séparés : contrôle à deux
niveaux, attention, compétences vérifiées et mémoire corrective. **REJECT en
production par défaut** tant qu'aucun axe n'a démontré un gain hors
optimisation, à budget égal, sans baisse de sûreté ni de reproductibilité.
