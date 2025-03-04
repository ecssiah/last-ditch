#[derive(Debug)]
pub enum Action {
    World(WorldAction),
    Leader(LeaderAction),
}

#[derive(Debug)]
pub enum WorldAction {}

#[derive(Debug)]
pub enum LeaderAction {}
