use bevy::prelude::*;

pub struct SpriteTexture {
    pub texture: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

pub fn load_spritesheet(
    spritesheet: String,
    tile_size: u32,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> SpriteTexture {
    let texture = asset_server.load(spritesheet);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(tile_size), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    SpriteTexture {
        texture: texture.clone(),
        texture_atlas_layout: texture_atlas_layout.clone(),
    }
}
