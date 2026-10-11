# Apprentissage procédural contrôlé

Le harnais peut désormais enregistrer des **politiques candidates** : une hypothèse
de stratégie, son baseline et un nombre minimal d'échantillons vérifiés. Ce n'est
pas une autorisation de réécrire son code, de créer des outils ou de s'accorder des
permissions.

```text
proposer -> exécuter en shadow mode -> verdict indépendant
                                         |
                       non vérifié ------+--> ignoré pour l'apprentissage
                                         |
                       vérifié ----------+--> compte succès/échec
                                                  |
                         meilleur que baseline + seuil atteint
                                                  |
                                      revue humaine / CI
                                      |              |
                                   promouvoir      rejeter
```

## Invariants appliqués par le code

- Une observation non vérifiée ne modifie jamais une politique candidate.
- Une candidate ne devient éligible qu'après le seuil d'échantillons et une
  majorité de succès vérifiés.
- L'éligibilité ne change pas le comportement actif.
- La promotion ou le rejet est une décision explicite de l'opérateur ou de la CI.
- Une candidate refusée ou promue ne reçoit plus de nouveaux résultats shadow.

## Sources étudiées et traduction locale

| Référence | Élément repris | Décision pour OP15 |
|---|---|---|
| Mnesio | politiques versionnées, shadow mode, gate | Registre local minimal, preuves obligatoires, revue explicite |
| Engram | audit append-only et améliorations réversibles | À relier ensuite à `trace.rs` puis à un stockage durable |
| Open-Sable | objectifs/mémoire/outils locaux | Rester sur les objectifs bornés et les permissions existantes |
| AGNT | objectifs et reprise de session | Référence produit seulement, pas de dépendance importée |
| Garudust Agent | runtime Rust à objectifs persistants | À auditer lorsqu'il sera accessible ; aucune affirmation intégrée |

Les dépôts externes sont des références d'architecture. Le registre ne démontre
encore aucun gain : un benchmark à budget égal devra comparer une candidate au
baseline avant toute promotion durable.

## Prochaine étape vérifiable

Persister les événements dans un journal append-only, définir les jeux de test
scellés, puis faire valider les promotions par CI avec oracle indépendant.
