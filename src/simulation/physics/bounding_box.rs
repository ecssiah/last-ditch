use glam::IVec3;
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn new(center: Vec3, size: Vec3) -> Self {
        let half = size * 0.5;

        let bounding_box = Self {
            min: center - half,
            max: center + half,
        };

        bounding_box
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
    
    pub fn blocks_overlapping(&self) -> Vec<IVec3> {
        let min = self.min.floor().as_ivec3();
        let max = self.max.floor().as_ivec3();

        let mut blocks = Vec::new();

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    blocks.push(IVec3::new(x, y, z));
                }
            }
        }

        blocks
    }
}
