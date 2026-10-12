use crate::controller::Decision;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEvent {
    pub task_id: String,
    pub prediction: String,
    pub observation: String,
    pub decision: Decision,
    pub verifier_verdict: Option<bool>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CausalTrace(pub Vec<TraceEvent>);
impl CausalTrace {
    pub fn push(&mut self, event: TraceEvent) {
        self.0.push(event);
    }
}
