use crate::simulation::state::population::identity::appearance::skin_tone::SkinTone;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PersonKey {
    pub skin_tone: SkinTone,
}

impl PersonKey {
    pub fn new(skin_tone: &SkinTone) -> Self {
        Self { skin_tone: skin_tone.clone() }
    }
}
