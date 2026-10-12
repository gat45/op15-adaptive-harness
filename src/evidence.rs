#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub id: String,
    pub provenance: String,
    pub supports_claim: String,
    pub independently_verified: bool,
    pub stale: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvidenceSet(pub Vec<Evidence>);

impl EvidenceSet {
    pub fn supports(&self, claim: &str) -> bool {
        self.0
            .iter()
            .any(|e| e.independently_verified && !e.stale && e.supports_claim == claim)
    }
}
