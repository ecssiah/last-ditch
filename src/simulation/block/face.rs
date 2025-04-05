use crate::simulation::BLOCK_RADIUS;
use bitflags::bitflags;
use glam::Vec3;
use serde::Deserialize;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Face: u8 {
        const XP = 1 << 0;
        const XN = 1 << 1;
        const YP = 1 << 2;
        const YN = 1 << 3;
        const ZP = 1 << 4;
        const ZN = 1 << 5;
    }
}

impl Face {
    #[rustfmt::skip]
    pub const ALL: [Face; 6] = [
        Face::XP,
        Face::XN,
        Face::YP,
        Face::YN,
        Face::ZP,
        Face::ZN,
    ];

    pub fn quad(self) -> [Vec3; 4] {
        let center = self.normal() * BLOCK_RADIUS;
        
        let right = self.right() * BLOCK_RADIUS;
        let up = self.up() * BLOCK_RADIUS;
        let left = -right;
        let down = -up;

        [
            center + left + down,
            center + right + down,
            center + right + up,
            center + left + up,
        ]
    }

    #[rustfmt::skip]
    pub fn normal(self) -> Vec3 {
        match self {
            Face::XP => Vec3::new( 1.0,  0.0,  0.0),
            Face::XN => Vec3::new(-1.0,  0.0,  0.0),
            Face::YP => Vec3::new( 0.0,  1.0,  0.0),
            Face::YN => Vec3::new( 0.0, -1.0,  0.0),
            Face::ZP => Vec3::new( 0.0,  0.0,  1.0),
            Face::ZN => Vec3::new( 0.0,  0.0, -1.0),
            _ => panic!("Invalid Face: {:?}", self),
        }
    }

    #[rustfmt::skip]
    pub fn up(self) -> Vec3 {
        match self {
            Face::XP => Vec3::new( 0.0,  1.0,  0.0),
            Face::XN => Vec3::new( 0.0,  1.0,  0.0),
            Face::YP => Vec3::new( 0.0,  0.0,  1.0),
            Face::YN => Vec3::new( 0.0,  0.0,  1.0),
            Face::ZP => Vec3::new( 0.0,  1.0,  0.0),
            Face::ZN => Vec3::new( 0.0,  1.0,  0.0),
            _ => panic!("Invalid Face: {:?}", self),
        }
    }

    #[rustfmt::skip]
    pub fn right(self) -> Vec3 {
        match self {
            Face::XP => Vec3::new( 0.0,  0.0, -1.0),
            Face::XN => Vec3::new( 0.0,  0.0,  1.0),
            Face::YP => Vec3::new(-1.0,  0.0,  0.0),
            Face::YN => Vec3::new( 1.0,  0.0,  0.0),
            Face::ZP => Vec3::new( 1.0,  0.0,  0.0),
            Face::ZN => Vec3::new(-1.0,  0.0,  0.0),
            _ => panic!("Invalid Face: {:?}", self),
        }
    }
}
