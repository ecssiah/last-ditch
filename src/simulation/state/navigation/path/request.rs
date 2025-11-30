use ultraviolet::IVec3;

pub struct Request {
    pub path_request_id: u64,
    pub start: IVec3,
    pub end: IVec3,
}
