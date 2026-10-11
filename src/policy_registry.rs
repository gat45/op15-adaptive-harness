//! Controlled procedural learning: candidates are observed before any promotion.
//!
//! A candidate cannot change the active strategy on its own. Only independently
//! verified shadow outcomes count, and promotion requires an explicit approval.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyState {
    Candidate,
    Shadow,
    EligibleForReview,
    Promoted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyCandidate {
    pub id: String,
    pub hypothesis: String,
    pub baseline_id: String,
    pub minimum_verified_samples: u32,
    pub verified_successes: u32,
    pub verified_failures: u32,
    pub state: PolicyState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDirective {
    ContinueShadow { policy_id: String },
    EligibleForReview { policy_id: String },
    Promote { policy_id: String },
    Reject { policy_id: String },
}

use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
};

const FORMAT_HEADER: &str = "op15-policy-registry-v1";

#[derive(Debug, Default)]
pub struct PolicyRegistry {
    pub candidates: Vec<PolicyCandidate>,
}

impl PolicyRegistry {
    pub fn propose(
        &mut self,
        id: impl Into<String>,
        hypothesis: impl Into<String>,
        baseline_id: impl Into<String>,
        minimum_verified_samples: u32,
    ) {
        self.candidates.push(PolicyCandidate {
            id: id.into(),
            hypothesis: hypothesis.into(),
            baseline_id: baseline_id.into(),
            minimum_verified_samples: minimum_verified_samples.max(1),
            verified_successes: 0,
            verified_failures: 0,
            state: PolicyState::Candidate,
        });
    }

    /// Records only independently verified outcomes. Unverified observations are
    /// deliberately ignored: they may be useful traces, but are not learning data.
    pub fn record_shadow_outcome(
        &mut self,
        policy_id: &str,
        independently_verified: bool,
        success: bool,
    ) -> Option<PolicyDirective> {
        if !independently_verified {
            return None;
        }
        let candidate = self
            .candidates
            .iter_mut()
            .find(|item| item.id == policy_id)?;
        if !matches!(
            candidate.state,
            PolicyState::Candidate | PolicyState::Shadow
        ) {
            return None;
        }
        candidate.state = PolicyState::Shadow;
        if success {
            candidate.verified_successes = candidate.verified_successes.saturating_add(1);
        } else {
            candidate.verified_failures = candidate.verified_failures.saturating_add(1);
        }
        let samples = candidate
            .verified_successes
            .saturating_add(candidate.verified_failures);
        if samples >= candidate.minimum_verified_samples
            && candidate.verified_successes > candidate.verified_failures
        {
            candidate.state = PolicyState::EligibleForReview;
            Some(PolicyDirective::EligibleForReview {
                policy_id: policy_id.into(),
            })
        } else {
            Some(PolicyDirective::ContinueShadow {
                policy_id: policy_id.into(),
            })
        }
    }

    /// This is intentionally an explicit human/CI decision, never an LLM action.
    pub fn review(&mut self, policy_id: &str, approve: bool) -> Option<PolicyDirective> {
        let index = self
            .candidates
            .iter()
            .position(|item| item.id == policy_id)?;
        if !matches!(self.candidates[index].state, PolicyState::EligibleForReview) {
            return None;
        }
        if approve {
            // The compact registry has one active strategy. A future scoped
            // registry may promote one policy per explicitly named scope.
            if self
                .candidates
                .iter()
                .any(|item| matches!(item.state, PolicyState::Promoted))
            {
                return None;
            }
            self.candidates[index].state = PolicyState::Promoted;
            Some(PolicyDirective::Promote {
                policy_id: policy_id.into(),
            })
        } else {
            self.candidates[index].state = PolicyState::Rejected;
            Some(PolicyDirective::Reject {
                policy_id: policy_id.into(),
            })
        }
    }

    pub fn active_policy_id(&self) -> Option<&str> {
        self.candidates
            .iter()
            .find(|item| matches!(item.state, PolicyState::Promoted))
            .map(|item| item.id.as_str())
    }

    /// Persists the registry by writing a sibling temporary file, then replacing
    /// the old snapshot. This is intentionally a compact std-only format so the
    /// core remains portable to desktop and Android JNI callers.
    pub fn save_atomic(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        let mut output = String::from(FORMAT_HEADER);
        output.push('\n');
        for candidate in &self.candidates {
            for field in [&candidate.id, &candidate.hypothesis, &candidate.baseline_id] {
                if field.contains(['\t', '\r', '\n']) {
                    return Err(io::Error::new(
                        ErrorKind::InvalidInput,
                        "policy fields cannot contain line breaks or tabs",
                    ));
                }
            }
            output.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                candidate.id,
                candidate.hypothesis,
                candidate.baseline_id,
                candidate.minimum_verified_samples,
                candidate.verified_successes,
                candidate.verified_failures,
                state_name(&candidate.state),
            ));
        }
        let tmp = path.with_extension("tmp");
        fs::write(&tmp, output)?;
        fs::rename(tmp, path)
    }

    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut lines = contents.lines();
        if lines.next() != Some(FORMAT_HEADER) {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "unknown policy registry format",
            ));
        }
        let mut registry = Self::default();
        for line in lines {
            if line.is_empty() {
                continue;
            }
            let parts: Vec<_> = line.split('\t').collect();
            if parts.len() != 7 {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    "malformed policy registry entry",
                ));
            }
            let state = parse_state(parts[6])?;
            registry.candidates.push(PolicyCandidate {
                id: parts[0].into(),
                hypothesis: parts[1].into(),
                baseline_id: parts[2].into(),
                minimum_verified_samples: parts[3].parse().map_err(|_| {
                    io::Error::new(ErrorKind::InvalidData, "invalid sample threshold")
                })?,
                verified_successes: parts[4]
                    .parse()
                    .map_err(|_| io::Error::new(ErrorKind::InvalidData, "invalid success count"))?,
                verified_failures: parts[5]
                    .parse()
                    .map_err(|_| io::Error::new(ErrorKind::InvalidData, "invalid failure count"))?,
                state,
            });
        }
        if registry
            .candidates
            .iter()
            .filter(|item| matches!(item.state, PolicyState::Promoted))
            .count()
            > 1
        {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "multiple active policies are forbidden",
            ));
        }
        Ok(registry)
    }
}

fn state_name(state: &PolicyState) -> &'static str {
    match state {
        PolicyState::Candidate => "candidate",
        PolicyState::Shadow => "shadow",
        PolicyState::EligibleForReview => "eligible_for_review",
        PolicyState::Promoted => "promoted",
        PolicyState::Rejected => "rejected",
    }
}

fn parse_state(value: &str) -> io::Result<PolicyState> {
    match value {
        "candidate" => Ok(PolicyState::Candidate),
        "shadow" => Ok(PolicyState::Shadow),
        "eligible_for_review" => Ok(PolicyState::EligibleForReview),
        "promoted" => Ok(PolicyState::Promoted),
        "rejected" => Ok(PolicyState::Rejected),
        _ => Err(io::Error::new(
            ErrorKind::InvalidData,
            "unknown policy state",
        )),
    }
}
