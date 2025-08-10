use crate::simulation;

#[derive(Clone, Copy)]
pub enum Kind {
    Main,
    Empty,
    WorldTest,
    GraphTest,
    Placeholder,
}

impl Kind {
    pub fn config(&self) -> simulation::Config {
        match self {
            Kind::Main => simulation::Config {
                kind: *self,
                world_extent_chunks: 4,
                chunk_extent_blocks: 8,
                seed: 0,
            },
            Kind::Empty => simulation::Config {
                kind: *self,
                world_extent_chunks: 1,
                chunk_extent_blocks: 2,
                seed: 0,
            },
            Kind::WorldTest => simulation::Config {
                kind: *self,
                world_extent_chunks: 3,
                chunk_extent_blocks: 4,
                seed: 0,
            },
            Kind::GraphTest => simulation::Config {
                kind: *self,
                world_extent_chunks: 2,
                chunk_extent_blocks: 4,
                seed: 0,
            },
            Kind::Placeholder => simulation::Config {
                kind: *self,
                world_extent_chunks: 0,
                chunk_extent_blocks: 0,
                seed: 0,
            },
        }
    }
}
