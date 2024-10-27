use crate::animation::{trigger_animation, AnimationConfig};
use crate::asset_loader::load_spritesheet;
use crate::movement::move_sprite;
use bevy::prelude::*;

const YETI_SPRITESHEET: &str = "yeti_spritesheet.png";

pub struct YetiPlugin;

impl Plugin for YetiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_yeti).add_systems(
            Update,
            (
                // press the right arrow key to animate the right sprite
                trigger_animation::<YetiSprite>.run_if(|input: Res<ButtonInput<KeyCode>>| {
                    input.pressed(KeyCode::KeyA)
                        || input.pressed(KeyCode::KeyD)
                        || input.pressed(KeyCode::KeyW)
                        || input.pressed(KeyCode::KeyS)
                }),
                move_sprite::<YetiSprite>, // press the left arrow key to animate the left sprite
            ),
        );
    }
}

#[derive(Component)]
struct YetiSprite;

pub fn setup_yeti(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let sprite_texture = load_spritesheet(
        YETI_SPRITESHEET.to_string(),
        asset_server,
        texture_atlas_layouts,
    );
    let animation_config = AnimationConfig::new(0, 2, 20);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.0))
                .with_translation(Vec3::new(-50.0, 0.0, 0.0)),
            texture: sprite_texture.texture,
            ..default()
        },
        TextureAtlas {
            layout: sprite_texture.texture_atlas_layout,
            index: animation_config.first_sprite_index,
        },
        YetiSprite,
        animation_config,
    ));
}
