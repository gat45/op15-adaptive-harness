use op15_adaptive_harness::policy_registry::{PolicyDirective, PolicyRegistry, PolicyState};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn unverified_outcomes_never_change_a_candidate() {
    let mut registry = PolicyRegistry::default();
    registry.propose("p1", "retrieve first", "baseline", 2);
    assert_eq!(registry.record_shadow_outcome("p1", false, true), None);
    assert_eq!(registry.candidates[0].state, PolicyState::Candidate);
    assert_eq!(registry.candidates[0].verified_successes, 0);
}

#[test]
fn verified_shadow_results_require_review_before_promotion() {
    let mut registry = PolicyRegistry::default();
    registry.propose("p1", "retrieve first", "baseline", 2);
    assert!(matches!(
        registry.record_shadow_outcome("p1", true, true),
        Some(PolicyDirective::ContinueShadow { .. })
    ));
    assert!(matches!(
        registry.record_shadow_outcome("p1", true, true),
        Some(PolicyDirective::EligibleForReview { .. })
    ));
    assert_eq!(registry.candidates[0].state, PolicyState::EligibleForReview);
    assert!(matches!(
        registry.review("p1", true),
        Some(PolicyDirective::Promote { .. })
    ));
    assert_eq!(registry.candidates[0].state, PolicyState::Promoted);
}

#[test]
fn failed_shadow_policy_is_not_eligible() {
    let mut registry = PolicyRegistry::default();
    registry.propose("p1", "retrieve first", "baseline", 2);
    registry.record_shadow_outcome("p1", true, false);
    registry.record_shadow_outcome("p1", true, false);
    assert_eq!(registry.candidates[0].state, PolicyState::Shadow);
    assert_eq!(registry.review("p1", true), None);
}

#[test]
fn persisted_registry_preserves_the_single_promoted_policy() {
    let mut registry = PolicyRegistry::default();
    registry.propose("p1", "retrieve first", "baseline", 1);
    registry.record_shadow_outcome("p1", true, true);
    registry.review("p1", true);
    let path = std::env::temp_dir().join(format!(
        "op15-policy-{}.state",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    registry.save_atomic(&path).unwrap();
    let restored = PolicyRegistry::load(&path).unwrap();
    fs::remove_file(path).unwrap();
    assert_eq!(restored.active_policy_id(), Some("p1"));
    assert_eq!(restored.candidates[0].verified_successes, 1);
}

#[test]
fn only_one_policy_can_be_promoted() {
    let mut registry = PolicyRegistry::default();
    for id in ["p1", "p2"] {
        registry.propose(id, "retrieve first", "baseline", 1);
        registry.record_shadow_outcome(id, true, true);
    }
    assert!(matches!(
        registry.review("p1", true),
        Some(PolicyDirective::Promote { .. })
    ));
    assert_eq!(registry.review("p2", true), None);
    assert_eq!(registry.active_policy_id(), Some("p1"));
}
