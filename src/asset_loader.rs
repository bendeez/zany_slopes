use bevy::prelude::*;

const YETI_SPRITESHEET: &str = "yeti_spritesheet.png";

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZanySlopeAssets>()
            .add_systems(Startup, load_assets);
    }
}

#[derive(Resource, Debug, Default)]
pub struct ZanySlopeAssets {
    pub yeti: Handle<Image>,
}

pub fn load_assets(asset_server: Res<AssetServer>, mut zany_slope_assets: ResMut<ZanySlopeAssets>) {
    *zany_slope_assets = ZanySlopeAssets {
        yeti: asset_server.load(YETI_SPRITESHEET),
    };
}
