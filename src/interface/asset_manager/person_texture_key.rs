use crate::simulation::state::population::identity::appearance::skin_tone::SkinTone;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PersonTextureKey {
    pub skin_tone: SkinTone,
}

impl PersonTextureKey {
    pub fn from_skin_tone(skin_tone: &SkinTone) -> Self {
        Self { skin_tone: skin_tone.clone() }
    }
}
