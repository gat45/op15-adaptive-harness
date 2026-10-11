use std::{fs, path::Path};
#[derive(Debug, Clone, PartialEq, Eq)] pub struct Verdict { pub passed: bool, pub oracle: String, pub artifact: String }
pub trait Verifier { fn verify(&self) -> Verdict; }
pub struct StaticVerifier(pub Verdict); impl Verifier for StaticVerifier { fn verify(&self) -> Verdict { self.0.clone() } }
pub struct FileExistsVerifier { pub path: String }
impl Verifier for FileExistsVerifier { fn verify(&self) -> Verdict { let passed = Path::new(&self.path).is_file(); Verdict { passed, oracle:"file_exists".into(), artifact:self.path.clone() } } }
pub struct ExactFileVerifier { pub path: String, pub expected: String }
impl Verifier for ExactFileVerifier { fn verify(&self) -> Verdict { let content = fs::read_to_string(&self.path).unwrap_or_default(); Verdict { passed: content == self.expected, oracle:"file_exact".into(), artifact:self.path.clone() } } }
