use crate::simulation::{
    consts::*,
    state::world::{block, grid},
};
use glam::{IVec3, Vec3, Vec4};

#[derive(Clone, Debug)]
pub struct Face {
    pub position: IVec3,
    pub direction: grid::Direction,
    pub kind: block::Kind,
    pub light: Vec4,
}

impl Face {
    pub fn new(position: IVec3, direction: grid::Direction, kind: block::Kind) -> Self {
        Self {
            position,
            direction,
            kind,
            light: Vec4::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    pub fn vertices(&self) -> [Vec3; 4] {
        let center = self.position.as_vec3() + BLOCK_RADIUS * self.normal().as_vec3();

        let right = BLOCK_RADIUS * self.right().as_vec3();
        let up = BLOCK_RADIUS * self.up().as_vec3();
        let left = -right;
        let down = -up;

        [
            center + left + down,
            center + right + down,
            center + right + up,
            center + left + up,
        ]
    }

    pub fn normal(&self) -> IVec3 {
        match self.direction {
            grid::Direction::XpYoZo => IVec3::new(1, 0, 0),
            grid::Direction::XnYoZo => IVec3::new(-1, 0, 0),
            grid::Direction::XoYpZo => IVec3::new(0, 1, 0),
            grid::Direction::XoYnZo => IVec3::new(0, -1, 0),
            grid::Direction::XoYoZp => IVec3::new(0, 0, 1),
            grid::Direction::XoYoZn => IVec3::new(0, 0, -1),
            _ => panic!("Invalid Face: {:?}", self),
        }
    }

    pub fn up(&self) -> IVec3 {
        match self.direction {
            grid::Direction::XpYoZo => IVec3::new(0, 1, 0),
            grid::Direction::XnYoZo => IVec3::new(0, 1, 0),
            grid::Direction::XoYpZo => IVec3::new(0, 0, 1),
            grid::Direction::XoYnZo => IVec3::new(0, 0, 1),
            grid::Direction::XoYoZp => IVec3::new(0, 1, 0),
            grid::Direction::XoYoZn => IVec3::new(0, 1, 0),
            _ => panic!("Invalid Face: {:?}", self),
        }
    }

    pub fn right(&self) -> IVec3 {
        match self.direction {
            grid::Direction::XpYoZo => IVec3::new(0, 0, -1),
            grid::Direction::XnYoZo => IVec3::new(0, 0, 1),
            grid::Direction::XoYpZo => IVec3::new(-1, 0, 0),
            grid::Direction::XoYnZo => IVec3::new(1, 0, 0),
            grid::Direction::XoYoZp => IVec3::new(1, 0, 0),
            grid::Direction::XoYoZn => IVec3::new(-1, 0, 0),
            _ => panic!("Invalid Face: {:?}", self),
        }
    }

    pub fn connected(&self, face: &Face) -> bool {
        let same_direction = self.direction == face.direction;

        if same_direction {
            match self.direction {
                grid::Direction::XpYoZo | grid::Direction::XnYoZo => {
                    let same_z = self.position.z == face.position.z;
                    let y_connected = (self.position.y - face.position.y).abs() == 1;
                    let same_y = self.position.y == face.position.y;
                    let z_connected = (self.position.z - face.position.z).abs() == 1;

                    (same_z && y_connected) || (same_y && z_connected)
                }
                grid::Direction::XoYpZo | grid::Direction::XoYnZo => {
                    let same_x = self.position.x == face.position.x;
                    let z_connected = (self.position.z - face.position.z).abs() == 1;
                    let same_z = self.position.z == face.position.z;
                    let x_connected = (self.position.x - face.position.x).abs() == 1;

                    (same_x && z_connected) || (same_z && x_connected)
                }
                grid::Direction::XoYoZp | grid::Direction::XoYoZn => {
                    let same_x = self.position.x == face.position.x;
                    let y_connected = (self.position.y - face.position.y).abs() == 1;
                    let same_y = self.position.y == face.position.y;
                    let x_connected = (self.position.x - face.position.x).abs() == 1;

                    (same_x && y_connected) || (same_y && x_connected)
                }
                _ => panic!("Invalid face: {:?}", self.direction),
            }
        } else {
            false
        }
    }
}
