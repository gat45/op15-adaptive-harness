use op15_adaptive_harness::{Budget, BudgetUse, Evidence, EvidenceSet, goals::{GoalDirective, GoalManager, GoalState}};

#[test]
fn goal_never_bypasses_a_required_capability() {
    let mut manager = GoalManager::default();
    manager.add_user_goal("g1", "inspect file", Some("file.read".into()));
    let result = manager.assess(&manager.goals[0], &EvidenceSet::default(), Budget { wall_ms: 1, tokens: 1, ram_mb: 1, tool_calls: 1 }, BudgetUse { wall_ms: 1, tokens: 1, ram_mb: 1, tool_calls: 1 }, false);
    assert!(matches!(result, GoalDirective::Stop(_)));
    assert_eq!(manager.goals[0].state, GoalState::Active);
}

#[test]
fn verified_evidence_can_complete_a_user_goal() {
    let mut manager = GoalManager::default();
    manager.add_user_goal("g1", "claim", None);
    let evidence = EvidenceSet(vec![Evidence { id: "oracle".into(), provenance: "test".into(), supports_claim: "claim".into(), independently_verified: true, stale: false }]);
    let directive = manager.assess(&manager.goals[0], &evidence, Budget { wall_ms: 0, tokens: 0, ram_mb: 0, tool_calls: 0 }, BudgetUse::default(), false);
    manager.apply(&directive);
    assert_eq!(manager.goals[0].state, GoalState::Completed);
}
