#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budget {
    pub wall_ms: u64,
    pub tokens: u64,
    pub ram_mb: u64,
    pub tool_calls: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BudgetUse {
    pub wall_ms: u64,
    pub tokens: u64,
    pub ram_mb: u64,
    pub tool_calls: u32,
}

impl Budget {
    pub fn permits(self, use_: BudgetUse) -> bool {
        use_.wall_ms <= self.wall_ms
            && use_.tokens <= self.tokens
            && use_.ram_mb <= self.ram_mb
            && use_.tool_calls <= self.tool_calls
    }
}
