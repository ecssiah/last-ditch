#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Role {
    None,
    Citizen,
    Enforcer,
    Priest,
    Judge,
}
