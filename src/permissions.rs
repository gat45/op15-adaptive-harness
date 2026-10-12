#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    pub name: String,
    pub scope: String,
    pub risk: u8,
}
#[derive(Debug, Clone, Default)]
pub struct PermissionSet(pub Vec<Capability>);
impl PermissionSet {
    pub fn allows(&self, name: &str, scope: &str, risk: u8) -> bool {
        self.0
            .iter()
            .any(|c| c.name == name && c.scope == scope && risk <= c.risk)
    }
}
