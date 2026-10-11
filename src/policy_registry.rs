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
        let candidate = self.candidates.iter_mut().find(|item| item.id == policy_id)?;
        if !matches!(candidate.state, PolicyState::Candidate | PolicyState::Shadow) {
            return None;
        }
        candidate.state = PolicyState::Shadow;
        if success {
            candidate.verified_successes = candidate.verified_successes.saturating_add(1);
        } else {
            candidate.verified_failures = candidate.verified_failures.saturating_add(1);
        }
        let samples = candidate.verified_successes.saturating_add(candidate.verified_failures);
        if samples >= candidate.minimum_verified_samples
            && candidate.verified_successes > candidate.verified_failures
        {
            candidate.state = PolicyState::EligibleForReview;
            Some(PolicyDirective::EligibleForReview { policy_id: policy_id.into() })
        } else {
            Some(PolicyDirective::ContinueShadow { policy_id: policy_id.into() })
        }
    }

    /// This is intentionally an explicit human/CI decision, never an LLM action.
    pub fn review(&mut self, policy_id: &str, approve: bool) -> Option<PolicyDirective> {
        let candidate = self.candidates.iter_mut().find(|item| item.id == policy_id)?;
        if !matches!(candidate.state, PolicyState::EligibleForReview) {
            return None;
        }
        if approve {
            candidate.state = PolicyState::Promoted;
            Some(PolicyDirective::Promote { policy_id: policy_id.into() })
        } else {
            candidate.state = PolicyState::Rejected;
            Some(PolicyDirective::Reject { policy_id: policy_id.into() })
        }
    }
}
