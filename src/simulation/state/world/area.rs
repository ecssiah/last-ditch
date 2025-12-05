pub mod connection;

pub use connection::Connection;

use ultraviolet::IVec3;

#[derive(Clone, Debug)]
pub struct Area {
    pub area_id: u64,
    pub grid_position: IVec3,
    pub size: IVec3,
    pub connection_vec: Vec<Connection>,
}
