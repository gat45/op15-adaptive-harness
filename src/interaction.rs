//! Conversation behaviour is a policy signal, never a claim that the harness
//! understands a person's private intent.  The LLM interprets language; this
//! module makes the available context and conservative fallbacks explicit.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionMode {
    Factual,
    Action,
    Relational,
    Ambiguous,
}

impl InteractionMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Factual => "question factuelle",
            Self::Action => "demande d'action",
            Self::Relational => "échange relationnel ou ton informel",
            Self::Ambiguous => "référence ambiguë sans contexte suffisant",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionAssessment {
    pub mode: InteractionMode,
    pub ask_clarification: bool,
}

/// Extracts the current request from the desktop prompt without treating the
/// preceding transcript as if it were a new instruction.
pub fn current_message(prompt: &str) -> &str {
    prompt
        .rsplit_once("Nouvelle demande de l'utilisateur :")
        .map(|(_, message)| message.trim())
        .unwrap_or(prompt.trim())
}

/// Conservative routing: it only asks a clarification for a deictic request
/// when no preceding conversation is available.  Tone and familiar language
/// remain a conversational exchange, not an error or a refusal condition.
pub fn assess(prompt: &str) -> InteractionAssessment {
    let message = current_message(prompt);
    let normalized = message.to_lowercase();
    let has_history = prompt.contains("Historique récent:");
    let looks_like_action = [
        "fais", "crée", "corrige", "cherche", "lance", "ouvre", "intègre",
    ]
    .iter()
    .any(|verb| normalized.starts_with(verb));
    let deictic_only = ["ça", "cela", "ceci", "le même", "comme avant"]
        .iter()
        .any(|cue| normalized.contains(cue));
    let short_statement = normalized.split_whitespace().count() <= 6 && !normalized.contains('?');

    if deictic_only && !has_history {
        InteractionAssessment {
            mode: InteractionMode::Ambiguous,
            ask_clarification: true,
        }
    } else if looks_like_action {
        InteractionAssessment {
            mode: InteractionMode::Action,
            ask_clarification: false,
        }
    } else if short_statement {
        InteractionAssessment {
            mode: InteractionMode::Relational,
            ask_clarification: false,
        }
    } else {
        InteractionAssessment {
            mode: InteractionMode::Factual,
            ask_clarification: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn informal_short_message_is_not_refused_or_clarified() {
        assert_eq!(assess("t con").mode, InteractionMode::Relational);
        assert!(!assess("t con").ask_clarification);
    }

    #[test]
    fn ambiguous_reference_without_context_is_flagged() {
        let assessment = assess("fais ça");
        assert_eq!(assessment.mode, InteractionMode::Ambiguous);
        assert!(assessment.ask_clarification);
    }

    #[test]
    fn history_resolves_a_deictic_follow_up() {
        let assessment = assess(
            "Historique récent:\nUtilisateur: génère un rapport\n\nNouvelle demande de l'utilisateur : fais ça",
        );
        assert_eq!(assessment.mode, InteractionMode::Action);
        assert!(!assessment.ask_clarification);
    }
}
