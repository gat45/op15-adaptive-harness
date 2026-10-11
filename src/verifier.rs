#[derive(Debug, Clone, PartialEq, Eq)] pub struct Verdict { pub passed: bool, pub oracle: String, pub artifact: String }
pub trait Verifier { fn verify(&self) -> Verdict; }
pub struct StaticVerifier(pub Verdict); impl Verifier for StaticVerifier { fn verify(&self) -> Verdict { self.0.clone() } }
