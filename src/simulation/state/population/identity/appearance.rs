pub mod cloth_color;
pub mod hair_color;
pub mod skin_tone;

use crate::simulation::state::population::identity::appearance::{
    cloth_color::ClothColor, hair_color::HairColor, skin_tone::SkinTone,
};

#[derive(Clone)]
pub struct Appearance {
    pub skin_tone: SkinTone,
    pub hair_color: HairColor,
    pub cloth_color: ClothColor,
}

impl Appearance {
    pub fn new(skin_tone: &SkinTone, hair_color: &HairColor, cloth_color: &ClothColor) -> Self {
        Self {
            skin_tone: skin_tone.clone(),
            hair_color: hair_color.clone(),
            cloth_color: cloth_color.clone(),
        }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self::new(
            &SkinTone::Color1,
            &HairColor::Color1,
            &ClothColor::MaleColor1,
        )
    }
}
