# OP15 Adaptive Harness

Projet de coordination pour un harnais LLM local, vérifiable et économe sur
CPU/GPU/NPU. Il ne remplace pas les projets spécialisés : il relie leurs
contrats, les mesures et les décisions de promotion.

## Objectif

Réduire le coût par tâche réussie vérifiée, sans promotion automatique d'une
stratégie, d'un prompt ou d'un backend. Le contrôleur choisit entre réponse,
récupération, raisonnement, outil, vérification, changement de backend et arrêt.

Le périmètre est celui d'un agent autonome **métacognitif au sens
fonctionnel** : il maintient un modèle auditable de ses capacités, de son
incertitude, de ses actions et de leurs résultats. Le projet ne prétend ni
implémenter ni démontrer une conscience subjective.

## Dépôts de travail

- [Harnais vivant](https://github.com/gat45/geniex_harness) : exécution
  Reason–Act–Observe, governor et RAG unifié.
- [JARVIX Memory](https://github.com/gat45/jarvix-memory) : mémoire vérifiable
  et provenance.
- [rag-jarvix](https://github.com/GaTmaNnes/rag-jarvix) : RAG compact Rust.
- [ggml-hexagon-op15](https://github.com/gat45/ggml-hexagon-op15) : backend HTP.
- [xpu-observer](https://github.com/gat45/xpu-observer) : télémétrie XDNA.

## Démarrage

Ce dépôt commence par les contrats et l'évaluation. Voir :

- [Références complètes](SOURCES.md)
- [Feuille de route](TODO.md)
- [Architecture](ARCHITECTURE.md)
- [Périmètre métacognitif et protocole](METACOGNITION_SCOPE.md)

## Règles de promotion

Une candidate n'est promue que si elle est testée hors de son jeu
d'optimisation, ne dégrade pas une capacité critique et améliore la réussite à
budget égal ou le coût par succès vérifié. Sinon : `REJECT` ou `INCONCLUSIF`.
