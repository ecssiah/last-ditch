use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Info {
    pub solid: bool,
    pub local_position: Vec3,
    pub radius: Vec3,
}
