//! Selects explicit reasoning lenses before asking a local LLM for a final
//! answer. The selection is transparent and cheap; it is not a proof that a
//! model applied a discipline correctly.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningLens {
    Epistemic,
    Logic,
    Quantitative,
    Causal,
    Decision,
    Linguistic,
    Planning,
    Scientific,
}

impl ReasoningLens {
    pub fn label(self) -> &'static str {
        match self {
            Self::Epistemic => "faits, hypothèses et inconnues",
            Self::Logic => "cohérence et contradictions",
            Self::Quantitative => "mesures et calcul reproductible",
            Self::Causal => "causes possibles et contrefactuels",
            Self::Decision => "coût, risque et valeur d'information",
            Self::Linguistic => "sens, ton, implicite et ambiguïté",
            Self::Planning => "objectif, étapes et préconditions",
            Self::Scientific => "hypothèses réfutables et expérience discriminante",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningPlan {
    pub lenses: Vec<ReasoningLens>,
    pub questions: Vec<&'static str>,
}

fn add(plan: &mut ReasoningPlan, lens: ReasoningLens, question: &'static str) {
    if !plan.lenses.contains(&lens) {
        plan.lenses.push(lens);
    }
    if !plan.questions.contains(&question) {
        plan.questions.push(question);
    }
}

/// Routes a request toward lenses that can be checked. Epistemic tracking is
/// always present: the harness must distinguish a measured fact from a model
/// hypothesis even for ordinary conversation.
pub fn route(text: &str) -> ReasoningPlan {
    let text = text.to_lowercase();
    let mut plan = ReasoningPlan { lenses: vec![ReasoningLens::Epistemic], questions: vec!["Qu'est-ce qui est observé, supposé ou encore inconnu ?"] };

    if ["pourquoi", "cause", "hier", "fonctionnait", "panne", "ne marche", "régression"]
        .iter().any(|cue| text.contains(cue)) {
        add(&mut plan, ReasoningLens::Causal, "Quelles causes concurrentes prédisent des observations différentes ?");
        add(&mut plan, ReasoningLens::Scientific, "Quel test réversible et peu coûteux départage ces hypothèses ?");
        add(&mut plan, ReasoningLens::Decision, "Quel test maximise l'information utile pour son coût et son risque ?");
    }
    if ["%", "ms", "tok/s", "ram", "vram", "mesure", "moyenne", "écart", "calcul", "combien"]
        .iter().any(|cue| text.contains(cue)) {
        add(&mut plan, ReasoningLens::Quantitative, "Quelles unités, valeurs de référence et calculs reproductibles sont nécessaires ?");
    }
    if ["contradiction", "incohérent", "vrai", "faux", "preuve", "prouve"]
        .iter().any(|cue| text.contains(cue)) {
        add(&mut plan, ReasoningLens::Logic, "Quelles prémisses sont incompatibles ou manquent de preuve ?");
    }
    if ["ironie", "double sens", "sous-entendu", "veut dire", "ton", "blague"]
        .iter().any(|cue| text.contains(cue)) {
        add(&mut plan, ReasoningLens::Linguistic, "Quelles interprétations pragmatiques sont plausibles dans ce contexte ?");
    }
    if ["crée", "corrige", "intègre", "lance", "plan", "étape", "todo"]
        .iter().any(|cue| text.contains(cue)) {
        add(&mut plan, ReasoningLens::Planning, "Quelle prochaine étape est réversible, autorisée et vérifiable ?");
    }
    plan
}

pub fn guidance(plan: &ReasoningPlan) -> String {
    let lenses = plan.lenses.iter().map(|lens| lens.label()).collect::<Vec<_>>().join(" ; ");
    let questions = plan.questions.join(" ");
    format!("Angles de raisonnement sélectionnés : {lenses}. Utilise-les silencieusement : {questions}")
}

#[derive(Debug, Clone, PartialEq)]
pub struct MathCheck {
    pub baseline: f64,
    pub observed: f64,
}
impl MathCheck {
    pub fn relative_change(&self) -> Option<f64> {
        (self.baseline != 0.0).then(|| (self.observed - self.baseline) / self.baseline)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalHypothesis {
    pub id: String,
    pub expected_observation: String,
    pub test_cost: u32,
    pub information_score: u32,
}
pub fn best_discriminating_test(hypotheses: &[CausalHypothesis]) -> Option<&CausalHypothesis> {
    hypotheses.iter().filter(|hypothesis| hypothesis.test_cost > 0).max_by_key(|hypothesis| hypothesis.information_score * 1_000 / hypothesis.test_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npu_regression_uses_causal_and_measured_lenses() {
        let plan = route("Pourquoi mon NPU fonctionnait hier et plus aujourd'hui ? Compare les mesures.");
        assert!(plan.lenses.contains(&ReasoningLens::Causal));
        assert!(plan.lenses.contains(&ReasoningLens::Scientific));
        assert!(plan.lenses.contains(&ReasoningLens::Quantitative));
    }
    #[test]
    fn causal_test_prefers_information_per_cost() {
        let choices = [
            CausalHypothesis { id: "expensive".into(), expected_observation: "x".into(), test_cost: 10, information_score: 10 },
            CausalHypothesis { id: "cheap".into(), expected_observation: "y".into(), test_cost: 1, information_score: 3 },
        ];
        assert_eq!(best_discriminating_test(&choices).unwrap().id, "cheap");
    }
    #[test]
    fn math_check_keeps_units_out_of_the_llm() {
        assert_eq!(MathCheck { baseline: 10.0, observed: 15.0 }.relative_change(), Some(0.5));
    }
}
