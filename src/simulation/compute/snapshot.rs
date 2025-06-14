pub mod chunk_path_snapshot;

pub use chunk_path_snapshot::ChunkPathSnapshot;

pub trait Snapshot: Send + Sync {}
