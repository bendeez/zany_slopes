mod animation;
mod asset_loader;
mod camera;
mod movement;
mod scene;
mod window;
mod yeti;
use animation::AnimationPlugin;
use camera::CameraPlugin;
use scene::ScenePlugin;
use window::WindowPlugin;
use yeti::YetiPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WindowPlugin)
        // prevents blurry sprites
        .add_plugins(CameraPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(YetiPlugin)
        .add_plugins(ScenePlugin)
        .run();
}
