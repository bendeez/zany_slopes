use bevy::prelude::*;

pub struct ScenePlugin;

const SCENE: &str = "scene.png";

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands, window: Query<&Window>, asset_server: Res<AssetServer>) {
    let window = window.single();
    let scene_image = asset_server.load(SCENE);
    commands.spawn(SpriteBundle {
        transform: Transform::from_scale(Vec3::new(window.width(), window.height(), 0.0))
            .with_translation(Vec3::new(0.0, 0.0, -1.0)),
        texture: scene_image,
        ..default()
    });
}
