use ultraviolet::{Rotor3, Vec3};

pub struct Rotor3Ext {}

impl Rotor3Ext {
    pub fn forward(rotor: Rotor3) -> Vec3 {
        rotor * Vec3::unit_y()
    }

    pub fn right(rotor: Rotor3) -> Vec3 {
        rotor * Vec3::unit_x()
    }

    pub fn up(rotor: Rotor3) -> Vec3 {
        rotor * Vec3::unit_z()
    }
}
