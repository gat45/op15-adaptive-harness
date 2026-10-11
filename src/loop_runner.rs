use std::time::Instant;
use crate::{budget::{Budget,BudgetUse}, controller::{Controller,Decision,StopReason,Task}, evidence::{Evidence,EvidenceSet}, llama_cpp::LlamaCppClient, trace::{CausalTrace,TraceEvent}, verifier::{VerificationRequest,Verifier}};
#[derive(Debug)] pub struct LoopResult { pub decision: Decision, pub observation: String, pub trace: CausalTrace }
pub struct TaskLoop;
impl TaskLoop {
    pub fn run(task:&Task, budget:Budget, retrieve_cost:BudgetUse, client:&LlamaCppClient, verifier:&dyn Verifier, max_steps:u8) -> LoopResult {
        let mut evidence=EvidenceSet::default(); let mut trace=CausalTrace::default(); let mut remaining=budget; let mut observation=String::new();
        for step in 0..max_steps {
            let d=Controller::decide(task,&evidence,remaining,retrieve_cost);
            match d {
                Decision::Retrieve => {
                    if let Some(b)=remaining.remaining_after(retrieve_cost) { remaining=b } else { return LoopResult{decision:Decision::Stop(StopReason::BudgetExhausted),observation,trace} };
                    let started=Instant::now();
                    let result=client.complete(&task.claim,12);
                    observation=match result { Ok(c)=>c.text, Err(error)=>format!("[llama_cpp_error] {error}") };
                    let request=VerificationRequest { task_id:task.id.clone(), claim:task.claim.clone(), response:observation.clone(), artifacts:Vec::new(), criteria:Vec::new() };
                    let v=verifier.verify(&request);
                    trace.push(TraceEvent{task_id:task.id.clone(),prediction:format!("retrieve:{step}"),observation:observation.clone(),decision:Decision::Retrieve,verifier_verdict:Some(v.supports(&request))});
                    if v.supports(&request) { evidence.0.push(Evidence{id:v.artifact,provenance:v.oracle,supports_claim:task.claim.clone(),independently_verified:true,stale:false}); }
                    if started.elapsed().as_millis() > remaining.wall_ms as u128 { return LoopResult{decision:Decision::Stop(StopReason::BudgetExhausted),observation,trace}; }
                },
                other => return LoopResult{decision:other,observation,trace}
            }
        }
        LoopResult{decision:Decision::Stop(StopReason::EvidenceInsufficient),observation,trace}
    }
}
