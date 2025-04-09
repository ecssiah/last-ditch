use crate::simulation::BLOCK_SIZE;

pub struct TextureAtlas {}

impl TextureAtlas {
    pub const TILE_SIZE: u32 = 32;
    pub const ATLAS_WIDTH: u32 = 1024;
    pub const ATLAS_HEIGHT: u32 = 1024;

    pub fn get_uv_coords(tile_x: u32, tile_y: u32) -> [f32; 4] {
        let u_min = (tile_x * Self::TILE_SIZE) as f32 / Self::ATLAS_WIDTH as f32;
        let v_min = (tile_y * Self::TILE_SIZE) as f32 / Self::ATLAS_HEIGHT as f32;
        let u_max =
            ((tile_x + BLOCK_SIZE as u32) * Self::TILE_SIZE) as f32 / Self::ATLAS_WIDTH as f32;
        let v_max =
            ((tile_y + BLOCK_SIZE as u32) * Self::TILE_SIZE) as f32 / Self::ATLAS_HEIGHT as f32;

        [u_min, v_min, u_max, v_max]
    }
}
