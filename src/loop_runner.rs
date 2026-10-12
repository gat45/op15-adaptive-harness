use crate::{
    budget::{Budget, BudgetUse},
    controller::{Controller, Decision, StopReason, Task},
    evidence::{Evidence, EvidenceSet},
    llama_cpp::{LlamaCppClient, public_answer_prompt, public_text},
    trace::{CausalTrace, TraceEvent},
    verifier::{VerificationRequest, Verifier},
};
use std::time::Instant;
#[derive(Debug)]
pub struct LoopResult {
    pub decision: Decision,
    pub observation: String,
    pub model_error: Option<String>,
    pub trace: CausalTrace,
}
pub struct TaskLoop;
impl TaskLoop {
    pub fn run(
        task: &Task,
        budget: Budget,
        retrieve_cost: BudgetUse,
        client: &LlamaCppClient,
        verifier: &dyn Verifier,
        max_steps: u8,
    ) -> LoopResult {
        let mut evidence = EvidenceSet::default();
        let mut trace = CausalTrace::default();
        let mut remaining = budget;
        let mut observation = String::new();
        let mut model_error = None;
        for step in 0..max_steps {
            let d = Controller::decide(task, &evidence, remaining, retrieve_cost);
            match d {
                Decision::Retrieve => {
                    if let Some(b) = remaining.remaining_after(retrieve_cost) {
                        remaining = b
                    } else {
                        return LoopResult {
                            decision: Decision::Stop(StopReason::BudgetExhausted),
                            observation,
                            model_error,
                            trace,
                        };
                    };
                    let started = Instant::now();
                    // Reasoning-capable GGUF models need room for both a bounded
                    // private trace and a public answer. The server receives a
                    // separate reasoning budget; this remains an output cap.
                    let max_tokens = retrieve_cost.tokens.min(768).max(1) as u16;
                    let result = client.complete(&public_answer_prompt(&task.claim), max_tokens);
                    observation = match result {
                        Ok(c) => {
                            let answer = public_text(&c.text);
                            if answer.is_empty() {
                                model_error=Some("Le modèle n'a produit aucune réponse publique ; augmente le budget de génération ou change de modèle.".into());
                                String::new()
                            } else {
                                answer
                            }
                        }
                        Err(error) => {
                            model_error = Some(error);
                            String::new()
                        }
                    };
                    let request = VerificationRequest {
                        task_id: task.id.clone(),
                        claim: task.claim.clone(),
                        response: observation.clone(),
                        artifacts: Vec::new(),
                        criteria: Vec::new(),
                    };
                    let v = verifier.verify(&request);
                    trace.push(TraceEvent {
                        task_id: task.id.clone(),
                        prediction: format!("retrieve:{step}"),
                        observation: observation.clone(),
                        decision: Decision::Retrieve,
                        verifier_verdict: Some(v.supports(&request)),
                    });
                    if v.supports(&request) {
                        evidence.0.push(Evidence {
                            id: v.artifact,
                            provenance: v.oracle,
                            supports_claim: task.claim.clone(),
                            independently_verified: true,
                            stale: false,
                        });
                    }
                    if started.elapsed().as_millis() > remaining.wall_ms as u128 {
                        return LoopResult {
                            decision: Decision::Stop(StopReason::BudgetExhausted),
                            observation,
                            model_error,
                            trace,
                        };
                    }
                }
                other => {
                    return LoopResult {
                        decision: other,
                        observation,
                        model_error,
                        trace,
                    };
                }
            }
        }
        LoopResult {
            decision: Decision::Stop(StopReason::EvidenceInsufficient),
            observation,
            model_error,
            trace,
        }
    }
}
