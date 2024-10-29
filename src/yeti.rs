use crate::animation::{trigger_animation, AnimationConfig};
use crate::asset_loader::ZanySlopeAssets;
use crate::movement::move_sprite;
use bevy::prelude::*;

const TILE_SIZE: u32 = 16;
const YETI_DIMENSIONS: f32 = 3.0;

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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    zany_slope_assets: Res<ZanySlopeAssets>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::new(0, 2, 20);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(YETI_DIMENSIONS))
                .with_translation(Vec3::new(-50.0, 0.0, 0.0)),
            texture: zany_slope_assets.yeti.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_config.first_sprite_index,
        },
        YetiSprite,
        animation_config,
    ));
}
