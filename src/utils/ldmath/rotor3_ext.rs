use ultraviolet::{Rotor3, Vec3};

#[inline]
pub fn forward(rotor: Rotor3) -> Vec3 {
    rotor * Vec3::unit_y()
}

#[inline]
pub fn right(rotor: Rotor3) -> Vec3 {
    rotor * Vec3::unit_x()
}

#[inline]
pub fn up(rotor: Rotor3) -> Vec3 {
    rotor * Vec3::unit_z()
}
