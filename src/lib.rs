//! Minimal verifiable adaptive controller.
pub mod budget; pub mod controller; pub mod evidence; pub mod hardware; pub mod metacognition; pub mod self_model; pub mod state; pub mod trace; pub mod verifier;
pub use budget::{Budget, BudgetUse}; pub use controller::{Controller, Decision, StopReason, Task}; pub use evidence::{Evidence, EvidenceSet};
