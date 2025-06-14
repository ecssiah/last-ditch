pub mod chunk_path_result;

pub use chunk_path_result::ChunkPathResult;

pub trait Result: Send {
    fn as_any(&self) -> &dyn std::any::Any;
}
