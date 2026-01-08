#[derive(Clone, PartialEq)]
pub enum AssetStatus {
    Init,
    LoadingTextures,
    LoadingModels,
    Complete,
}
