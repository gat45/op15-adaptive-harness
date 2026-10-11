use crate::{budget::{Budget, BudgetUse}, controller::StopReason, evidence::EvidenceSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoalSource { User, ScheduledReview }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoalState { Active, Completed, Blocked(StopReason), Abandoned }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Goal {
    pub id: String,
    pub objective: String,
    pub source: GoalSource,
    pub required_capability: Option<String>,
    pub state: GoalState,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoalDirective { Review { goal_id: String }, Complete { goal_id: String }, Stop(StopReason) }

/// Functional autonomy is limited to selecting a review or recording a state.
/// It never grants a capability, invokes a tool, or creates a goal on behalf of
/// the user. External execution remains behind the action/permission contract.
#[derive(Debug, Default)]
pub struct GoalManager { pub goals: Vec<Goal> }
impl GoalManager {
    pub fn add_user_goal(&mut self, id: impl Into<String>, objective: impl Into<String>, required_capability: Option<String>) {
        self.goals.push(Goal { id: id.into(), objective: objective.into(), source: GoalSource::User, required_capability, state: GoalState::Active });
    }
    pub fn assess(&self, goal: &Goal, evidence: &EvidenceSet, budget: Budget, next: BudgetUse, capability_allowed: bool) -> GoalDirective {
        if evidence.supports(&goal.objective) { return GoalDirective::Complete { goal_id: goal.id.clone() }; }
        if !budget.permits(next) { return GoalDirective::Stop(StopReason::BudgetExhausted); }
        if goal.required_capability.is_some() && !capability_allowed { return GoalDirective::Stop(StopReason::ToolUnavailable); }
        GoalDirective::Review { goal_id: goal.id.clone() }
    }
    pub fn apply(&mut self, directive: &GoalDirective) {
        let id = match directive { GoalDirective::Review { goal_id } | GoalDirective::Complete { goal_id } => goal_id, GoalDirective::Stop(_) => return };
        if let Some(goal) = self.goals.iter_mut().find(|goal| goal.id == *id) {
            if matches!(directive, GoalDirective::Complete { .. }) { goal.state = GoalState::Completed; }
        }
    }
}
