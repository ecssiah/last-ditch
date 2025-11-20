//! Math extensions for the LD coordinate system

use ultraviolet::{Rotor3, Vec3};

pub trait Rotor3Ext {
    fn forward(rotor: Rotor3) -> Vec3;
    fn right(rotor: Rotor3) -> Vec3;
    fn up(rotor: Rotor3) -> Vec3;
}

impl Rotor3Ext for Rotor3 {
    fn forward(rotor: Rotor3) -> Vec3 {
        rotor * Vec3::unit_y()
    }

    fn right(rotor: Rotor3) -> Vec3 {
        rotor * Vec3::unit_x()
    }

    fn up(rotor: Rotor3) -> Vec3 {
        rotor * Vec3::unit_z()
    }
}