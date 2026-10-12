//! A light, auditable router between context analysis and action planning.
//! Lenses orient a local LLM; they are not evidence and never make an answer
//! verified by themselves.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationRequirement {
    None,
    Calculator,
    Symbolic,
    Solver,
    Source,
    Experiment,
}
impl VerificationRequirement {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "aucun contrôle requis",
            Self::Calculator => "calcul indépendant requis",
            Self::Symbolic => "vérification symbolique requise",
            Self::Solver => "solveur requis",
            Self::Source => "source indépendante requise",
            Self::Experiment => "expérience discriminante requise",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningLens {
    Logic,
    Quantitative,
    Epistemic,
    Causal,
    Decision,
    Linguistic,
    Planning,
    Scientific,
}
impl ReasoningLens {
    pub fn label(self) -> &'static str {
        match self {
            Self::Logic => "logique et contradictions",
            Self::Quantitative => "mathématiques et probabilités",
            Self::Epistemic => "épistémologie et incertitude",
            Self::Causal => "causalité",
            Self::Decision => "décision et optimisation",
            Self::Linguistic => "linguistique et pragmatique",
            Self::Planning => "planification",
            Self::Scientific => "méthode scientifique",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensContract {
    pub lens: ReasoningLens,
    pub question: &'static str,
    pub verification: VerificationRequirement,
    pub estimated_cost: u8,
}
pub fn contract(lens: ReasoningLens) -> LensContract {
    match lens {
        ReasoningLens::Logic => LensContract {
            lens,
            question: "Quelles prémisses sont incompatibles ou manquent de preuve ?",
            verification: VerificationRequirement::Symbolic,
            estimated_cost: 2,
        },
        ReasoningLens::Quantitative => LensContract {
            lens,
            question: "Quelles unités, valeurs sources et calculs reproductibles sont nécessaires ?",
            verification: VerificationRequirement::Calculator,
            estimated_cost: 1,
        },
        ReasoningLens::Epistemic => LensContract {
            lens,
            question: "Qu'est-ce qui est observé, déclaré, inféré ou inconnu ?",
            verification: VerificationRequirement::Source,
            estimated_cost: 1,
        },
        ReasoningLens::Causal => LensContract {
            lens,
            question: "Quelles causes concurrentes prédisent des observations différentes ?",
            verification: VerificationRequirement::Experiment,
            estimated_cost: 2,
        },
        ReasoningLens::Decision => LensContract {
            lens,
            question: "Quel choix maximise le bénéfice attendu pour son coût et son risque ?",
            verification: VerificationRequirement::None,
            estimated_cost: 1,
        },
        ReasoningLens::Linguistic => LensContract {
            lens,
            question: "Quelles interprétations pragmatiques restent plausibles dans ce contexte ?",
            verification: VerificationRequirement::None,
            estimated_cost: 1,
        },
        ReasoningLens::Planning => LensContract {
            lens,
            question: "Quelle prochaine étape est autorisée, réversible et vérifiable ?",
            verification: VerificationRequirement::None,
            estimated_cost: 1,
        },
        ReasoningLens::Scientific => LensContract {
            lens,
            question: "Quel test réversible et peu coûteux départage les hypothèses ?",
            verification: VerificationRequirement::Experiment,
            estimated_cost: 2,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedLens {
    pub contract: LensContract,
    pub justification: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningResult {
    pub selected: Vec<SelectedLens>,
    pub hypotheses: Vec<String>,
    pub evidence_ids: Vec<String>,
    pub unresolved_questions: Vec<&'static str>,
    pub required_checks: Vec<VerificationRequirement>,
}
fn add(result: &mut ReasoningResult, lens: ReasoningLens, justification: &'static str, deep: bool) {
    if result
        .selected
        .iter()
        .any(|item| item.contract.lens == lens)
        || (!deep && result.selected.len() >= 3)
    {
        return;
    }
    let contract = contract(lens);
    if contract.verification != VerificationRequirement::None
        && !result.required_checks.contains(&contract.verification)
    {
        result.required_checks.push(contract.verification);
    }
    result.unresolved_questions.push(contract.question);
    result.selected.push(SelectedLens {
        contract,
        justification,
    });
}

/// Normal routing activates zero to three lenses. A request explicitly asking
/// for analysis/comparison can opt into a broader, still explicit set.
pub fn route(text: &str) -> ReasoningResult {
    let text = text.to_lowercase();
    let deep = [
        "analyse",
        "approfondi",
        "complexe",
        "plusieurs angles",
        "compare",
    ]
    .iter()
    .any(|cue| text.contains(cue));
    let mut result = ReasoningResult {
        selected: Vec::new(),
        hypotheses: Vec::new(),
        evidence_ids: Vec::new(),
        unresolved_questions: Vec::new(),
        required_checks: Vec::new(),
    };
    if [
        "pourquoi",
        "cause",
        "hier",
        "fonctionnait",
        "panne",
        "ne marche",
        "régression",
    ]
    .iter()
    .any(|cue| text.contains(cue))
    {
        add(
            &mut result,
            ReasoningLens::Causal,
            "le message décrit un changement ou une cause possible",
            deep,
        );
        add(
            &mut result,
            ReasoningLens::Scientific,
            "des hypothèses concurrentes doivent être départagées",
            deep,
        );
        add(
            &mut result,
            ReasoningLens::Decision,
            "le choix du test doit tenir compte du coût et du risque",
            deep,
        );
    }
    if [
        "%", "ms", "tok/s", "ram", "vram", "mesure", "moyenne", "écart", "calcul", "combien",
    ]
    .iter()
    .any(|cue| text.contains(cue))
    {
        add(
            &mut result,
            ReasoningLens::Quantitative,
            "la réponse dépend de valeurs ou d'unités mesurables",
            deep,
        );
    }
    if [
        "contradiction",
        "incohérent",
        "vrai",
        "faux",
        "preuve",
        "prouve",
    ]
    .iter()
    .any(|cue| text.contains(cue))
    {
        add(
            &mut result,
            ReasoningLens::Logic,
            "des prémisses ou sources peuvent être incompatibles",
            deep,
        );
        add(
            &mut result,
            ReasoningLens::Epistemic,
            "le statut des affirmations doit être séparé",
            deep,
        );
    }
    if [
        "ironie",
        "double sens",
        "sous-entendu",
        "veut dire",
        "ton",
        "blague",
    ]
    .iter()
    .any(|cue| text.contains(cue))
    {
        add(
            &mut result,
            ReasoningLens::Linguistic,
            "le sens dépend du contexte pragmatique",
            deep,
        );
    }
    if [
        "crée", "corrige", "intègre", "lance", "plan", "étape", "todo",
    ]
    .iter()
    .any(|cue| text.contains(cue))
    {
        add(
            &mut result,
            ReasoningLens::Planning,
            "la demande peut entraîner une action à ordonner",
            deep,
        );
    }
    result
}
pub fn guidance(result: &ReasoningResult) -> String {
    if result.selected.is_empty() {
        return "Aucun raisonnement approfondi sélectionné : réponds simplement et attentivement."
            .into();
    }
    let lenses = result
        .selected
        .iter()
        .map(|item| format!("{} ({})", item.contract.lens.label(), item.justification))
        .collect::<Vec<_>>()
        .join(" ; ");
    let checks = result
        .required_checks
        .iter()
        .map(|check| check.label())
        .collect::<Vec<_>>()
        .join(" ; ");
    format!(
        "Lentilles sélectionnées : {lenses}. Questions : {}. Contrôles à effectuer avant une conclusion vérifiée : {}.",
        result.unresolved_questions.join(" "),
        if checks.is_empty() { "aucun" } else { &checks }
    )
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
    hypotheses
        .iter()
        .filter(|hypothesis| hypothesis.test_cost > 0)
        .max_by_key(|hypothesis| hypothesis.information_score * 1_000 / hypothesis.test_cost)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordinary_message_uses_no_expensive_lens() {
        assert!(route("bonjour, ça va ?").selected.is_empty());
    }
    #[test]
    fn npu_regression_is_limited_and_has_checks() {
        let plan = route("Pourquoi mon NPU fonctionnait hier et plus aujourd'hui ?");
        assert_eq!(plan.selected.len(), 3);
        assert!(
            plan.required_checks
                .contains(&VerificationRequirement::Experiment)
        );
    }
    #[test]
    fn complex_request_can_use_more_lenses() {
        assert!(route("Analyse approfondie : pourquoi mon NPU a régressé, compare les mesures et la contradiction").selected.len() > 3);
    }
    #[test]
    fn causal_test_prefers_information_per_cost() {
        let choices = [
            CausalHypothesis {
                id: "expensive".into(),
                expected_observation: "x".into(),
                test_cost: 10,
                information_score: 10,
            },
            CausalHypothesis {
                id: "cheap".into(),
                expected_observation: "y".into(),
                test_cost: 1,
                information_score: 3,
            },
        ];
        assert_eq!(best_discriminating_test(&choices).unwrap().id, "cheap");
    }
    #[test]
    fn math_check_keeps_units_out_of_the_llm() {
        assert_eq!(
            MathCheck {
                baseline: 10.0,
                observed: 15.0
            }
            .relative_change(),
            Some(0.5)
        );
    }
}
