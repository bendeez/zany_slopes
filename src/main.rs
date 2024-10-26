mod animation;
mod asset_loader;
mod camera;
mod movement;
mod yeti;
use animation::AnimationPlugin;
use camera::CameraPlugin;
use yeti::YetiPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(CameraPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(YetiPlugin)
        .run();
}
