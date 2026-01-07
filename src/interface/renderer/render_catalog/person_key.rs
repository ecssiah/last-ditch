use crate::simulation::state::population::identity::appearance::skin_tone::SkinTone;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PersonKey<'a> {
    pub skin_tone: &'a SkinTone,
}

impl<'a> PersonKey<'a> {
    pub fn new(skin_tone: &'a SkinTone) -> Self {
        Self { skin_tone }
    }
}
