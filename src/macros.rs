#[macro_export]
macro_rules! include_config {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/config/", $path))
    };
}

#[macro_export]
macro_rules! include_assets_src {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path))
    };
}
