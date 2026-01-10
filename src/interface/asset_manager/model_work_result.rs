use crate::interface::{
    asset_manager::{block_model_key::BlockModelKey, person_model_key::PersonModelKey},
    renderer::{
        block_renderer::block_model::BlockModel, person_renderer::person_model::PersonModel,
    },
};
use std::collections::HashMap;

#[derive(Default)]
pub struct ModelWorkResult {
    pub person_model_map: HashMap<PersonModelKey, PersonModel>,
    pub block_model_map: HashMap<BlockModelKey, BlockModel>,
}
