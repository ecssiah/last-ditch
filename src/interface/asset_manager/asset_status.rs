#[derive(Clone, PartialEq)]
pub enum AssetStatus {
    Startup,
    InitTextures,
    LoadingTextures,
    InitModels,
    LoadingModels,
    Complete,
}
