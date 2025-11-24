pub struct TileArray {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub tiles_per_row: u32,
    pub tiles_per_col: u32,
    pub tile_width: u32,
    pub tile_height: u32,
}

impl TileArray {
    pub fn from_atlas(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        atlas_path: &str,
        tile_width: u32,
        tile_height: u32,
    ) -> Self {
        let img = image::open(atlas_path).expect("Failed to load atlas PNG");
        let img = img.to_rgba8();

        let (atlas_w, atlas_h) = img.dimensions();

        assert!(atlas_w % tile_width == 0);
        assert!(atlas_h % tile_height == 0);

        let tiles_per_row = atlas_w / tile_width;
        let tiles_per_col = atlas_h / tile_height;
        let total_tiles = tiles_per_row * tiles_per_col;

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Block Tile Texture Array"),
            size: wgpu::Extent3d {
                width: tile_width,
                height: tile_height,
                depth_or_array_layers: total_tiles,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Slice atlas into tiles & upload each one
        for tile_index in 0..total_tiles {
            let tx = tile_index % tiles_per_row;
            let ty = tile_index / tiles_per_row;

            let x0 = tx * tile_width;
            let y0 = ty * tile_height;

            // Extract pixels into a Vec<u8>
            let mut tile_pixels = Vec::with_capacity((tile_width * tile_height * 4) as usize);

            for row in 0..tile_height {
                for col in 0..tile_width {
                    let px = img.get_pixel(x0 + col, y0 + row);
                    
                    tile_pixels.extend_from_slice(&px.0);
                }
            }

            // Upload layer
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: tile_index,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &tile_pixels,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * tile_width),
                    rows_per_image: Some(tile_height),
                },
                wgpu::Extent3d {
                    width: tile_width,
                    height: tile_height,
                    depth_or_array_layers: 1,
                },
            );
        }

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Block Tile Array View"),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            tiles_per_row,
            tiles_per_col,
            tile_width,
            tile_height,
        }
    }
}
