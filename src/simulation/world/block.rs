pub mod face;
pub mod id;
pub mod kind;
pub mod light;
pub mod meta;

pub use face::Face;
pub use id::ID;
pub use kind::Kind;
pub use light::Light;
pub use meta::Meta;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub emittance: u8,
    pub solid: bool,
}
