#[derive(Clone, PartialEq)]
pub enum AssetStatus {
    InitTextures,
    LoadingTextures,
    InitModels,
    LoadingModels,
    Complete,
}
