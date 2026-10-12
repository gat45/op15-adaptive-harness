//! Bounded specialist roles for one local model.
//!
//! A specialist is a promptable responsibility, not a second autonomous LLM.
//! Selection is deterministic and capped; it cannot execute tools, modify
//! memory, or make a claim independently verified.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Specialist {
    Dialogue,
    Memory,
    Evidence,
    Diagnostic,
    Planner,
    Performance,
}

impl Specialist {
    pub fn label(self) -> &'static str {
        match self {
            Self::Dialogue => "dialogue et pragmatique",
            Self::Memory => "mémoire et contexte",
            Self::Evidence => "preuves et vérification",
            Self::Diagnostic => "diagnostic causal",
            Self::Planner => "planification sûre",
            Self::Performance => "performance et ressources",
        }
    }

    fn mandate(self) -> &'static str {
        match self {
            Self::Dialogue => {
                "conserver les interprétations plausibles, le ton et les références au contexte sans inventer l'intention"
            }
            Self::Memory => {
                "retrouver uniquement les souvenirs fournis, signaler provenance, conflit ou absence, et ne rien mémoriser automatiquement"
            }
            Self::Evidence => {
                "séparer fait observé, affirmation utilisateur, hypothèse et preuve indépendante ; ne pas valider sans oracle"
            }
            Self::Diagnostic => {
                "formuler des causes concurrentes et proposer le test réversible le plus discriminant"
            }
            Self::Planner => {
                "décomposer la prochaine étape autorisée, réversible et vérifiable ; proposer une action plutôt que l'exécuter"
            }
            Self::Performance => {
                "raisonner avec unités, latence, tokens, RAM/VRAM et coût ; ne jamais transformer une mesure globale en attribution certaine"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistAssignment {
    pub specialist: Specialist,
    pub justification: &'static str,
}

fn add(
    assignments: &mut Vec<SpecialistAssignment>,
    specialist: Specialist,
    justification: &'static str,
) {
    if assignments.len() < 3 && !assignments.iter().any(|item| item.specialist == specialist) {
        assignments.push(SpecialistAssignment {
            specialist,
            justification,
        });
    }
}

/// Chooses at most three non-autonomous responsibilities for a single turn.
pub fn select(text: &str) -> Vec<SpecialistAssignment> {
    let text = text.to_lowercase();
    let mut assignments = Vec::new();

    if ["rappelle", "mémoire", "souvenir", "contexte", "oublie"]
        .iter()
        .any(|cue| text.contains(cue))
    {
        add(
            &mut assignments,
            Specialist::Memory,
            "la demande dépend d'un souvenir ou d'une référence antérieure",
        );
    }
    if ["preuve", "prouve", "vrai", "faux", "vérifie", "source"]
        .iter()
        .any(|cue| text.contains(cue))
    {
        add(
            &mut assignments,
            Specialist::Evidence,
            "la conclusion réclame un statut de preuve explicite",
        );
    }
    if [
        "pourquoi",
        "panne",
        "régression",
        "fonctionnait",
        "ne marche",
        "erreur",
    ]
    .iter()
    .any(|cue| text.contains(cue))
    {
        add(
            &mut assignments,
            Specialist::Diagnostic,
            "le message décrit une cause ou une régression à diagnostiquer",
        );
    }
    if [
        "crée", "corrige", "lance", "ouvre", "intègre", "plan", "étape",
    ]
    .iter()
    .any(|cue| text.contains(cue))
    {
        add(
            &mut assignments,
            Specialist::Planner,
            "la demande peut mener à une action qui doit rester contrôlée",
        );
    }
    if [
        "tok/s",
        "ttft",
        "vram",
        "ram",
        "gpu",
        "cpu",
        "npu",
        "ms",
        "latence",
        "performance",
    ]
    .iter()
    .any(|cue| text.contains(cue))
    {
        add(
            &mut assignments,
            Specialist::Performance,
            "la réponse dépend de mesures de performance ou de ressources",
        );
    }
    if assignments.is_empty()
        && ["ironie", "double sens", "sous-entendu", "ton", "blague"]
            .iter()
            .any(|cue| text.contains(cue))
    {
        add(
            &mut assignments,
            Specialist::Dialogue,
            "le sens pragmatique doit rester ouvert à plusieurs interprétations",
        );
    }
    assignments
}

pub fn guidance(assignments: &[SpecialistAssignment]) -> String {
    if assignments.is_empty() {
        return "Aucun spécialiste requis : réponds simplement, sans prétendre à une expertise mobilisée.".into();
    }
    assignments
        .iter()
        .map(|item| {
            format!(
                "{} : {}",
                item.specialist.label(),
                item.specialist.mandate()
            )
        })
        .collect::<Vec<_>>()
        .join(" ; ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_uses_bounded_specialists() {
        let selected =
            select("Pourquoi le NPU fonctionnait hier et plus aujourd'hui ? Mesure le tok/s");
        assert!(
            selected
                .iter()
                .any(|item| item.specialist == Specialist::Diagnostic)
        );
        assert!(
            selected
                .iter()
                .any(|item| item.specialist == Specialist::Performance)
        );
        assert!(selected.len() <= 3);
    }

    #[test]
    fn evidence_and_memory_remain_distinct() {
        let selected = select("Rappelle-moi la source et vérifie si elle est vraie");
        assert!(
            selected
                .iter()
                .any(|item| item.specialist == Specialist::Memory)
        );
        assert!(
            selected
                .iter()
                .any(|item| item.specialist == Specialist::Evidence)
        );
    }

    #[test]
    fn ordinary_greeting_does_not_create_agents() {
        assert!(select("bonjour").is_empty());
    }
}
