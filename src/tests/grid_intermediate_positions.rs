use crate::simulation::world::grid;
use glam::IVec3;

struct IntermediatePositionsCase {
    description: String,
    source: IVec3,
    target: IVec3,
    expected_intermediate_positions: Vec<IVec3>,
}

impl IntermediatePositionsCase {
    pub fn check(&self) {
        let intermediate_positions = grid::Grid::intermediate_positions(self.source, self.target);

        assert_eq!(
            intermediate_positions, self.expected_intermediate_positions,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn intermediate_positions() {
    let test_cases = vec![
        IntermediatePositionsCase {
            description: "(0, 0, 0) to (1, 1, 1)".to_string(),
            source: IVec3::new(0, 0, 0),
            target: IVec3::new(1, 1, 1),
            expected_intermediate_positions: Vec::from([IVec3::new(1, 1, 0), IVec3::new(0, 1, 1)]),
        },
        IntermediatePositionsCase {
            description: "(0, 0, 0) to (1, 0, 1)".to_string(),
            source: IVec3::new(0, 0, 0),
            target: IVec3::new(1, 0, 1),
            expected_intermediate_positions: Vec::from([IVec3::new(1, 0, 0), IVec3::new(0, 0, 1)]),
        },
        IntermediatePositionsCase {
            description: "(0, 0, 0) to (1, 0, 0)".to_string(),
            source: IVec3::new(0, 0, 0),
            target: IVec3::new(1, 0, 0),
            expected_intermediate_positions: Vec::from([]),
        },
        IntermediatePositionsCase {
            description: "(0, 0, 0) to (1, 0, 1)".to_string(),
            source: IVec3::new(0, 0, 0),
            target: IVec3::new(2, 2, 2),
            expected_intermediate_positions: Vec::from([]),
        },
    ];

    for case in test_cases {
        case.check();
    }
}
