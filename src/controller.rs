use crate::{budget::{Budget, BudgetUse}, evidence::EvidenceSet};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task { pub id: String, pub claim: String, pub tool_available: bool, pub confidence_milli: u16 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StopReason { BudgetExhausted, ToolUnavailable, Abstain, EvidenceInsufficient }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision { Answer, Retrieve, Verify, Stop(StopReason), ProposeCandidate { policy: String } }
pub struct Controller;
impl Controller {
    /// Hard constraints dominate estimated utility: confidence cannot authorize an answer.
    pub fn decide(task: &Task, evidence: &EvidenceSet, budget: Budget, next: BudgetUse) -> Decision {
        if evidence.supports(&task.claim) { return Decision::Answer; }
        if !budget.permits(next) { return Decision::Stop(StopReason::BudgetExhausted); }
        if !task.tool_available { return Decision::Stop(StopReason::ToolUnavailable); }
        if task.confidence_milli < 500 { return Decision::Stop(StopReason::Abstain); }
        Decision::Retrieve
    }
}
