use crate::controller::{Decision, StopReason};
#[derive(Debug, Clone, PartialEq, Eq)] pub struct MetaObservation { pub anomaly: String, pub trace_id: String }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct MetaAssessment { pub cause: String, pub requires_experiment: bool }
#[derive(Debug, Clone, PartialEq, Eq)] pub enum MetaGuide { Verify, Stop(StopReason), ProposeCandidate { policy: String } }
pub fn guide(observation: &MetaObservation, assessment: &MetaAssessment) -> MetaGuide { if observation.anomaly == "budget_exhausted" { MetaGuide::Stop(StopReason::BudgetExhausted) } else if assessment.requires_experiment { MetaGuide::ProposeCandidate { policy: assessment.cause.clone() } } else { MetaGuide::Verify } }
pub fn decision_from_guide(g: MetaGuide) -> Decision { match g { MetaGuide::Verify => Decision::Verify, MetaGuide::Stop(r) => Decision::Stop(r), MetaGuide::ProposeCandidate { policy } => Decision::ProposeCandidate { policy } } }
