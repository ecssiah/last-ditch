pub mod admin_action;
pub mod judge_action;
pub mod test_action;

pub use admin_action::AdminAction;
pub use judge_action::JudgeAction;
pub use judge_action::JumpAction;
pub use judge_action::MovementAction;
pub use test_action::TestAction;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Admin(AdminAction),
    Test(TestAction),
    Judge(JudgeAction),
}
