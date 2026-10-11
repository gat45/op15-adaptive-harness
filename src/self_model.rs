use std::collections::BTreeMap;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CapabilityProfile { pub attempts: u32, pub verified_successes: u32, pub false_confident_answers: u32 }
impl CapabilityProfile { pub fn empirical_success_rate(&self) -> Option<f64> { (self.attempts > 0).then(|| self.verified_successes as f64 / self.attempts as f64) } }
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SelfModel { pub profiles: BTreeMap<String, CapabilityProfile>, pub allowed_tools: Vec<String>, pub constraints: Vec<String> }
impl SelfModel { pub fn record_verdict(&mut self, kind: &str, success: bool, false_confident: bool) { let p = self.profiles.entry(kind.into()).or_default(); p.attempts += 1; p.verified_successes += u32::from(success); p.false_confident_answers += u32::from(false_confident); } }
