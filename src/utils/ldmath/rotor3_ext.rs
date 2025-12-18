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

#[inline]
pub fn from_rotation_xy_deg(rotation_xy: f32) -> Rotor3 {
    let rotation_xy = rotation_xy.to_radians();

    Rotor3::from_rotation_xy(rotation_xy)
}

#[inline]
pub fn from_rotation_yz_deg(rotation_yz: f32) -> Rotor3 {
    Rotor3::from_rotation_yz(rotation_yz.to_radians())
}
