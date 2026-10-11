use crate::budget::BudgetUse;
#[derive(Debug, Clone, PartialEq, Eq)] pub struct HardwareCost { pub allocation_mb: u64, pub transfer_ms: u64, pub dispatch_ms: u64, pub sync_ms: u64, pub fallback_ms: u64 }
impl HardwareCost { pub fn end_to_end(&self, tokens: u64, tool_calls: u32) -> BudgetUse { BudgetUse { wall_ms: self.transfer_ms + self.dispatch_ms + self.sync_ms + self.fallback_ms, tokens, ram_mb: self.allocation_mb, tool_calls } } }
