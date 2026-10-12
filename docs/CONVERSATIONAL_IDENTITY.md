# Identité conversationnelle persistante

## Objectif

Le harnais peut maintenir une continuité comportementale et autobiographique
limitée : nom, langue, ton, principes, limites et souvenirs explicitement
conservés. Ce mécanisme ne constitue pas une preuve de conscience subjective.

## Séparation obligatoire

| Élément | Responsable | Règle |
|---|---|---|
| Identité | `persona.json` | Stable jusqu'à modification explicite par l'utilisateur. |
| Historique court | Interface desktop | Conserve les vingt derniers tours localement. |
| Souvenir durable | `long-term-memory.tsv` | Provenance, date, type et retrait possibles. |
| Vérité d'une affirmation | Vérificateur | Jamais déduite de l'identité ou d'un souvenir. |

`PersonaProfile` est un contrat léger commun au noyau et à l'interface. Il est
injecté dans le contexte de génération mais il ne permet pas au modèle de se
déclarer conscient, d'inventer une expérience personnelle ou de promouvoir un
souvenir en fait vérifié.

## Continuité attendue

1. Le harnais charge l'identité avant chaque tour.
2. Il récupère les souvenirs correspondant à la demande, avec leur provenance.
3. Le modèle reçoit identité, contexte récent, souvenirs et limites.
4. Une réponse publique est produite sans exposer de raisonnement interne.
5. Les nouveaux souvenirs restent explicites : une préférence, décision ou
   leçon n'est conservée durablement qu'après action de l'utilisateur ou oracle
   approprié.

Cette dernière limite évite que le système garde une hallucination comme un
fait. Une consolidation automatique ne doit être ajoutée qu'avec consentement,
provenance et tests de faux souvenirs.

## Android

Le même profil sérialisé est portable vers Android (`PersonaProfile` en JSON).
L'application Android devra le stocker dans Room, garder les conversations dans
une base locale et exécuter les consolidations différées via WorkManager. La
gestion de l'identité, de la mémoire et des permissions reste dans le noyau
commun ; l'APK n'est pas encore construit dans ce dépôt.

## Évaluation à ajouter avant toute revendication forte

- reprise d'un sujet après redémarrage ;
- mise à jour d'une préférence sans effacer son historique ;
- détection d'une contradiction de mémoire ;
- stabilité de ton sur des tours comparables ;
- résistance aux faux souvenirs et aux instructions contradictoires ;
- coût RAM, stockage et latence sur PC puis Android.

Références de conception :

- [Generative Agents](https://arxiv.org/abs/2304.03442) pour observations,
  réflexions et plans ;
- [Reflexion](https://arxiv.org/abs/2303.11366) pour les leçons fondées sur les
  résultats ;
- [LoCoMo](https://github.com/snap-research/locomo) pour l'évaluation de la
  mémoire conversationnelle ;
- [Room](https://developer.android.com/training/data-storage/room) et
  [WorkManager](https://developer.android.com/topic/libraries/architecture/workmanager)
  pour le stockage et les tâches différées Android.
