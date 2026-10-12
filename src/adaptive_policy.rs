use std::collections::BTreeMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strategy {
    Answer,
    Retrieve,
    Verify,
    Abstain,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Estimate {
    pub attempts: u32,
    pub successes: u32,
    pub total_cost: u64,
}
impl Estimate {
    pub fn success_rate(&self) -> f64 {
        (self.successes as f64 + 1.0) / (self.attempts as f64 + 2.0)
    }
    pub fn mean_cost(&self) -> f64 {
        if self.attempts == 0 {
            1.0
        } else {
            self.total_cost as f64 / self.attempts as f64
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct AdaptivePolicy {
    pub estimates: BTreeMap<Strategy, Estimate>,
    pub value_of_success: f64,
}
impl AdaptivePolicy {
    pub fn choose(&self, available: &[Strategy]) -> Strategy {
        available
            .iter()
            .copied()
            .max_by(|a, b| {
                let score = |s: Strategy| {
                    let e = self.estimates.get(&s).cloned().unwrap_or_default();
                    e.success_rate() * self.value_of_success - e.mean_cost()
                };
                score(*a).total_cmp(&score(*b))
            })
            .unwrap_or(Strategy::Abstain)
    }
    pub fn record_verified(&mut self, s: Strategy, success: bool, cost: u64) {
        let e = self.estimates.entry(s).or_default();
        e.attempts = e.attempts.saturating_add(1);
        e.successes = e.successes.saturating_add(u32::from(success));
        e.total_cost = e.total_cost.saturating_add(cost);
    }
}
