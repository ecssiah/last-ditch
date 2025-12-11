use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Line {
    pub grid_position1: IVec3,
    pub grid_position2: IVec3,
}