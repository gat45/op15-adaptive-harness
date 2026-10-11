use crate::{controller::{Controller, Decision, Task}, budget::{Budget, BudgetUse}, evidence::EvidenceSet, trace::{CausalTrace, TraceEvent}, verifier::{VerificationRequest, Verifier}};
pub struct TaskRunner;
impl TaskRunner {
    pub fn run(task: &Task, evidence: &EvidenceSet, budget: Budget, next: BudgetUse, verifier: &dyn Verifier, trace: &mut CausalTrace) -> Decision {
        let decision = Controller::decide(task, evidence, budget, next);
        let request = VerificationRequest { task_id: task.id.clone(), claim: task.claim.clone(), response: String::new(), artifacts: Vec::new(), criteria: Vec::new() };
        let verdict = matches!(decision, Decision::Verify).then(|| { let result = verifier.verify(&request); result.supports(&request) });
        trace.push(TraceEvent { task_id: task.id.clone(), prediction: format!("{decision:?}"), observation: "controller decision".into(), decision: decision.clone(), verifier_verdict: verdict });
        decision
    }
}
