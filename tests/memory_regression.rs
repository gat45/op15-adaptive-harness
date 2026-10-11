use op15_adaptive_harness::self_model::SelfModel;
#[test] fn self_model_uses_verdicts() { let mut m = SelfModel::default(); m.record_verdict("build", true, false); m.record_verdict("build", false, true); assert_eq!(m.profiles["build"].empirical_success_rate(), Some(0.5)); assert_eq!(m.profiles["build"].false_confident_answers, 1); }
