# Profil local validé — GTX 1080 / Qwen3.5-9B

Ce profil est proposé dans l'application desktop sous **« Appliquer le profil
GTX 1080 validé »**. C'est un point de départ mesuré, non une promesse de
performance sur une autre machine.

## Configuration retenue

- serveur : `F:\lama-tensorRT 1050-5070\llama-server.exe` ;
- modèle : `D:\qwen9b_IQ4NL_owIQ4NL.gguf` (4,94 GiB) ;
- port : `18181` ; contexte : `32768` ;
- GPU : `-ngl 999`, Flash Attention activée ;
- cache KV K/V : `q8_0` ;
- batch / ubatch / threads / slots : `2048` / `512` / `8` / `1` ;
- MTP intégré : `--spec-type draft-mtp --spec-draft-n-max 2` ;
- métriques serveur : `--metrics`.

## Preuves locales consultées

- `F:\lama-tensorRT 1050-5070\RAPPORT_TESTS_MODELE_RETENU.md` : cinq tests
  fonctionnels réussis, `pp512 = 723,8 ± 1,18 t/s`, `tg128 = 33,16 ± 0,02 t/s`.
- `F:\lama-tensorRT 1050-5070\RAPPORT_V4_KERNEL_PROFILER.md` : GTX 1080
  Pascal, build `b10454-4df29be4f`, goulot `output.weight` et instrumentation
  CUDA optionnelle via `GGML_CUDA_TRACE_CSV`.
- `F:\lama-tensorRT 1050-5070\llama_ui.py` : arguments de lancement et
  résultat du balayage MTP : `n_max=2` est le meilleur compromis mesuré.

Les réglages RTX 5070, MoE 35B, NVFP4 et FreeToken sont volontairement exclus :
ils ne sont pas transférables à la GTX 1080 et au modèle dense 9B.
