#[derive(Clone, Debug)]
pub enum JumpStage {
    Ground,
    Launch,
    Rise,
    Fall,
}

#[derive(Clone)]
pub struct JumpState {
    pub timer: u32,
    pub stage: JumpStage,
}