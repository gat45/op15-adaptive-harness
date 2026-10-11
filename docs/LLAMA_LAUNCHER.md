# Lanceur llama.cpp externe

Le lanceur GGUF est livré avec l'application desktop. L'écran **Modeles** le
vérifie puis le démarre après un clic explicite ; aucun chemin manuel n'est requis.

## Contrôles appliqués

- ressource Tauri empaquetée : `resources/llama_launcher.exe` ;
- présence vérifiée avant lancement ;
- action explicite sur **Démarrer le lanceur GGUF**.

Le binaire fourni durant l'intégration se trouvait dans le dossier de build et
avait le SHA-256 suivant :

```text
B9EA50704DC564388C5E987D6011361D96230712B0B3571D6C97208F833210BF
```

Il était non signé et sans métadonnée produit. Il est intégré à la demande du
propriétaire du projet ; comparer ce hash avant toute mise à jour du binaire.

Après son démarrage, utiliser **Actualiser** dans l'interface : le statut doit
indiquer que llama.cpp est accessible sur `127.0.0.1:8080`.
