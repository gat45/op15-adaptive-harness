use op15_adaptive_harness::policy_registry::{PolicyDirective, PolicyRegistry, PolicyState};

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
    assert!(matches!(registry.record_shadow_outcome("p1", true, true), Some(PolicyDirective::ContinueShadow { .. })));
    assert!(matches!(registry.record_shadow_outcome("p1", true, true), Some(PolicyDirective::EligibleForReview { .. })));
    assert_eq!(registry.candidates[0].state, PolicyState::EligibleForReview);
    assert!(matches!(registry.review("p1", true), Some(PolicyDirective::Promote { .. })));
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
