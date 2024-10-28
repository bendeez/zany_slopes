use crate::configuration::ASSET_SCALE;
use bevy::prelude::*;

pub struct ScenePlugin;

const SCENE: &str = "scene.png";
const SCENE_WIDTH: f32 = 15.0 * ASSET_SCALE;
const SCENE_HEIGHT: f32 = 7.5 * ASSET_SCALE;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_image = asset_server.load(SCENE);
    commands.spawn(SpriteBundle {
        transform: Transform::from_scale(Vec3::new(SCENE_WIDTH, SCENE_HEIGHT, 0.0))
            .with_translation(Vec3::new(0.0, 0.0, -1.0)),
        texture: scene_image.clone(),
        ..default()
    });
}
