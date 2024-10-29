mod animation;
mod asset_loader;
mod camera;
mod movement;
mod scene;
mod yeti;
use animation::AnimationPlugin;
use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use scene::ScenePlugin;
use yeti::YetiPlugin;

use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{CursorGrabMode, PresentMode, WindowLevel, WindowTheme},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Zany Slopes".into(),
                        name: Some("bevy.app".into()),
                        resolution: (1366., 768.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        visible: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(ScenePlugin)
        // prevents blurry sprites
        .add_plugins(CameraPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(YetiPlugin)
        .run();
}
