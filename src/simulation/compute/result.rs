pub trait Result: Send {
    fn as_any(&self) -> &dyn std::any::Any;
}
