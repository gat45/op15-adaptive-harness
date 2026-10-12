use std::{fs, path::Path};

/// Exact context supplied to an independent oracle. A verdict is only promotable
/// when it explicitly verifies this claim and this response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationRequest {
    pub task_id: String,
    pub claim: String,
    pub response: String,
    pub artifacts: Vec<String>,
    pub criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub passed: bool,
    pub oracle: String,
    pub artifact: String,
    pub verified_claim: Option<String>,
    pub summary: String,
}

impl Verdict {
    pub fn supports(&self, request: &VerificationRequest) -> bool {
        self.passed && self.verified_claim.as_deref() == Some(request.claim.as_str())
    }
}

pub trait Verifier {
    fn verify(&self, request: &VerificationRequest) -> Verdict;
}

pub struct StaticVerifier(pub Verdict);
impl Verifier for StaticVerifier {
    fn verify(&self, _: &VerificationRequest) -> Verdict {
        self.0.clone()
    }
}

/// Availability check only. It can diagnose a missing artifact, but cannot
/// establish that a model response proves the user's claim.
pub struct FileExistsVerifier {
    pub path: String,
}
impl Verifier for FileExistsVerifier {
    fn verify(&self, _: &VerificationRequest) -> Verdict {
        let passed = Path::new(&self.path).is_file();
        Verdict {
            passed,
            oracle: "file_exists".into(),
            artifact: self.path.clone(),
            verified_claim: None,
            summary: "artifact availability only; not claim verification".into(),
        }
    }
}

/// Exact-content oracle for tasks whose success criterion is the full expected
/// response. This verifier deliberately refuses a matching file when the LLM
/// response itself differs.
pub struct ExactFileVerifier {
    pub path: String,
    pub expected: String,
}
impl Verifier for ExactFileVerifier {
    fn verify(&self, request: &VerificationRequest) -> Verdict {
        let content = fs::read_to_string(&self.path).unwrap_or_default();
        let passed = content == self.expected && request.response.trim() == self.expected.trim();
        Verdict {
            passed,
            oracle: "file_exact_response".into(),
            artifact: self.path.clone(),
            verified_claim: passed.then(|| request.claim.clone()),
            summary: if passed {
                "artifact and response match the declared criterion".into()
            } else {
                "artifact or response does not match the declared criterion".into()
            },
        }
    }
}
