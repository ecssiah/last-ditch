//! Macro definitions

#[macro_export]
macro_rules! include_assets {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path))
    };
}
