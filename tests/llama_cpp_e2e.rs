use op15_adaptive_harness::LlamaCppClient;

#[test]
#[ignore = "requires a configured local llama.cpp server on 127.0.0.1:8080"]
fn local_llama_cpp_completion() {
    let client = LlamaCppClient::local("F:\\tiers\\Nanbeige4.2-3B-Q8_0.gguf");
    let result = client
        // Reasoning-capable chat templates may spend a few tokens before the
        // public answer.  This is a connectivity/public-output test, not a
        // minimum-token benchmark.
        .complete("Réponds exactement par OK", 768)
        .expect("llama.cpp local doit répondre");
    assert!(!result.text.trim().is_empty());
}
