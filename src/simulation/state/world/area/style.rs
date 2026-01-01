use crate::simulation::state::population::nation::nation_kind::NationKind;

#[derive(Clone, Debug)]
pub enum Style {
    None,
    Wireframe,
    GenericRoom,
    Elevator,
    ElevatorCap,
    TradingPlatform,
    Temple { nation_kind: NationKind },
}
