# Audit produit de l'interface desktop

Date : 2026-10-12. Ce document décrit ce qui fonctionne dans l'interface
Windows et ce qui reste à construire. Il ne remplace pas les essais utilisateur.

## Verdict

La version précédente était utilisable comme prototype, mais pas comme poste de
pilotage : la page **Outils MCP** empilait deux longues cartes, chacune avec son
propre défilement. L'information critique (modèle, mémoire, décision, preuves)
était dispersée entre huit entrées et le pied de page répétait des mesures non
disponibles.

La version actuelle possède une seule zone de défilement par page, garde le
composeur du chat accessible et ramène la décision du dernier tour dans la
conversation. Les détails restent accessibles sans être imposés à chaque tour.

## Contrat écran par écran

| Vue | Fonction réelle | Ne pas lui attribuer |
| --- | --- | --- |
| Chat | conversation locale, 20 tours locaux, 12 réinjectés ; souvenirs explicites récupérés | mémoire durable automatique ou réponse vérifiée |
| Agent | décision, lentilles de raisonnement, trace et état d'oracle du dernier tour | chaîne de pensée privée, plan multi-étapes exécuté |
| Mémoire | ajout, liste et oubli explicite avec provenance utilisateur | graphe causal, résolution de conflit ou RAG complet |
| Outils MCP | état, démarrage et indexation Codebase Memory MCP ; fichiers et navigateur lancés par l'utilisateur | appels MCP ou actions fichiers/navigateur initiés par le LLM |
| Modèles | sélection GGUF, sélection du serveur, sauvegarde et démarrage local | validation de compatibilité/performance du GGUF avant exécution |
| Profiler | durée observée, `/metrics` si présent, GPU global par `nvidia-smi` | TTFT, débit et attribution GPU sans sonde adéquate |
| Benchmarks | rappel du périmètre de validité | benchmark adaptatif exécuté |
| Paramètres | profil GTX 1080 et paramètres de lancement enregistrés | garantie que chaque option est compatible avec chaque build llama.cpp |

## Croisement avec les références

1. LangGraph sépare la mémoire courte par thread et la mémoire longue par store
   durable. L'interface doit donc exposer une session, ses checkpoints et les
   souvenirs distinctement. Le stockage actuel est volontairement plus simple :
   historique local + fichier de souvenirs, sans checkpoint de graphe.
   Source : https://langchain-ai.github.io/langgraph/concepts/durable_execution/

2. Les interruptions humaines doivent afficher la proposition d'action, ses
   arguments et les choix approuver/modifier/refuser. L'écriture de fichier
   actuelle demande bien confirmation, mais elle n'est pas encore une carte
   d'approbation issue d'une proposition du modèle.
   Source : https://langchain-ai.github.io/langgraph/how-tos/human_in_the_loop/review-tool-calls/

3. Phoenix recommande de tracer appel LLM, récupération, outil, latence et
   résultat d'évaluation. Le compteur de trace actuel est insuffisant : une vue
   chronologique d'événements et des évaluations reproductibles restent à
   intégrer avant de revendiquer une observabilité agentique.
   Source : https://arize.com/docs/phoenix/tracing

4. L'allocation adaptative du calcul suppose un coût et une incertitude mesurés.
   Le budget de génération actuel est fixe ; les lentilles affichées sont une
   orientation de raisonnement, pas encore un arbitre expérimental de calcul.
   Source : https://arxiv.org/abs/2604.14853

## Priorités produit

1. Accueil **Pilotage** : modèle actif, mémoire de session, dernier verdict,
   action en attente et trois raccourcis seulement.
2. Journal de run : étapes horodatées `contexte -> récupération -> modèle ->
   vérification -> décision`, avec données réellement reçues.
3. File d'approbation : chaque action proposée porte une capacité, des
   arguments, un risque, un aperçu et les boutons approuver/modifier/refuser.
4. Paramètres liés au serveur : désactiver ou expliquer toute option non
   acceptée par le build llama.cpp sélectionné.
5. Évaluations à budget égal : jeu de tests, oracle, coût, taux de succès et
   régression avant toute promotion d'une politique.
