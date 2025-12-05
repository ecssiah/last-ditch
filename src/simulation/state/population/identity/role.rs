#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Role {
    None,
    Member,
    Enforcer,
    Priest,
    Judge,
}
