use crate::simulation::state::population::nation;

#[derive(Clone, Debug)]
pub enum Style {
    None,
    Wireframe,
    GenericRoom,
    Elevator,
    ElevatorCap,
    TradingPlatform,
    Temple { nation_kind: nation::Kind },
}
