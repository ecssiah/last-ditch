use ultraviolet::IVec3;

pub enum State {
    Idle,
    Navigating { grid_position: IVec3, path_request_id: Option<u64> },
    Moving { path_vec: Vec<IVec3> },
}