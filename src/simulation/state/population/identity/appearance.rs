pub mod hair_color;
pub mod skin_tone;

use crate::simulation::state::population::identity::appearance::{
    hair_color::HairColor, skin_tone::SkinTone,
};

#[derive(Clone)]
pub struct Appearance {
    pub skin_tone: SkinTone,
    pub hair_color: HairColor,
}

impl Appearance {
    pub fn new(skin_tone: &SkinTone, hair_color: &HairColor) -> Self {
        Self {
            skin_tone: skin_tone.clone(),
            hair_color: hair_color.clone(),
        }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self::new(&SkinTone::Person1, &HairColor::Person1)
    }
}
