#[macro_export]
macro_rules! include_shader_src {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/", $path))
    };
}