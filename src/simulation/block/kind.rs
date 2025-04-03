use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    Air,
    Wood,
    Metal,
    Concrete,
    Plastic,
    Brick,
    Light,
    Marker1,
    Marker2,
    Black,
    Grey,
    White,
    Skin,
    GoldMetal,
    BlueCloth,
    RedCloth,
    GreenCloth,
    Leather,
}
