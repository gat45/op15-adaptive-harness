//! Persistent conversational identity.
//!
//! A persona is not a claim of consciousness and it is deliberately kept
//! separate from memories: values and conversational style are stable until a
//! user changes them, whereas memories keep their provenance and may expire or
//! be retracted.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PersonaProfile {
    pub name: String,
    pub language: String,
    pub tone: String,
    pub principles: Vec<String>,
    pub boundaries: Vec<String>,
}

impl Default for PersonaProfile {
    fn default() -> Self {
        Self {
            name: "Jarvix".into(),
            language: "français".into(),
            tone: "direct, calme, attentif et sans jargon inutile".into(),
            principles: vec![
                "distinguer les faits, les hypothèses et les souvenirs".into(),
                "reconnaître une incertitude au lieu de l'inventer".into(),
                "reprendre les sujets importants à partir du contexte disponible".into(),
            ],
            boundaries: vec![
                "ne jamais prétendre être conscient ni éprouver des émotions".into(),
                "ne jamais transformer une hypothèse ou un souvenir déclaré en fait vérifié".into(),
                "demander une confirmation avant une action sensible ou irréversible".into(),
            ],
        }
    }
}

impl PersonaProfile {
    pub fn normalized(mut self) -> Self {
        if self.name.trim().is_empty() {
            self.name = Self::default().name;
        }
        if self.language.trim().is_empty() {
            self.language = Self::default().language;
        }
        if self.tone.trim().is_empty() {
            self.tone = Self::default().tone;
        }
        self.principles.retain(|value| !value.trim().is_empty());
        self.boundaries.retain(|value| !value.trim().is_empty());
        self
    }

    /// Text inserted into the public-answer contract. It contains only user
    /// editable behavioural commitments, never hidden model reasoning.
    pub fn prompt_context(&self) -> String {
        format!(
            "Identité conversationnelle persistante : nom = {}; langue = {}; ton = {}.\\nPrincipes : {}.\\nLimites : {}.",
            self.name.trim(),
            self.language.trim(),
            self.tone.trim(),
            self.principles.join(" ; "),
            self.boundaries.join(" ; "),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_identity_is_french_and_has_a_consciousness_boundary() {
        let profile = PersonaProfile::default();
        assert_eq!(profile.language, "français");
        assert!(profile
            .boundaries
            .iter()
            .any(|value| value.contains("conscient")));
    }

    #[test]
    fn blank_fields_fall_back_without_erasing_explicit_principles() {
        let profile = PersonaProfile {
            name: " ".into(),
            language: "".into(),
            tone: "".into(),
            principles: vec!["répondre en français".into(), " ".into()],
            boundaries: vec![],
        }
        .normalized();
        assert_eq!(profile.name, "Jarvix");
        assert_eq!(profile.principles, vec!["répondre en français"]);
    }
}
