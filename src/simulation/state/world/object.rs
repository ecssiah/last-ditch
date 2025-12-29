pub mod block;
pub mod door;
pub mod info;
pub mod ladder;
pub mod stairs;

pub use block::Block;
pub use door::Door;
pub use info::Info;
pub use ladder::Ladder;
pub use stairs::Stairs;

// fn setup_collider(object_kind: self::Kind) -> BoxCollider {
//     match object_kind {
//         Kind::DoorOpen => {
//             let local_position = Vec3::new(0.0, 0.0, CELL_RADIUS_IN_METERS);
//             let radius = Vec3::new(1.00, 0.25, 2.00) * CELL_SIZE_IN_METERS;

//             let mut box_collider = BoxCollider::new(local_position, radius);
//             box_collider.active = false;

//             box_collider
//         }
//         Kind::DoorClosed => {
//             let local_position = Vec3::new(0.0, 0.0, CELL_RADIUS_IN_METERS);
//             let radius = Vec3::new(1.00, 0.25, 2.00) * CELL_RADIUS_IN_METERS;

//             let box_collider = BoxCollider::new(local_position, radius);

//             box_collider
//         }
//         Kind::Stairs => {
//             let local_position = Vec3::new(0.0, 0.0, 0.0);
//             let radius = Vec3::new(1.00, 1.00, 1.00) * CELL_RADIUS_IN_METERS;

//             let box_collider = BoxCollider::new(local_position, radius);

//             box_collider
//         }
//         Kind::Platform => {
//             let local_position = Vec3::new(0.0, 0.0, CELL_RADIUS_IN_METERS - CELL_UNIT_EIGHTH);
//             let radius = Vec3::new(1.00, 1.00, 0.25) * CELL_RADIUS_IN_METERS;

//             let box_collider = BoxCollider::new(local_position, radius);

//             box_collider
//         }
//         Kind::Ladder => {
//             let local_position = Vec3::new(0.0, CELL_RADIUS_IN_METERS - CELL_UNIT_EIGHTH, 0.0);
//             let radius = Vec3::new(1.00, 0.25, 1.00) * CELL_RADIUS_IN_METERS;

//             let box_collider = BoxCollider::new(local_position, radius);

//             box_collider
//         }
//     }
// }
