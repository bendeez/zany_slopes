use bevy::prelude::*;

pub fn move_sprite<S: Component>(
    mut query: Query<&mut Transform, With<S>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // we expect the Component of type S to be used as a marker Component by only a single entity
    let mut sprite_transformation = query.single_mut();
    // we create a new timer when the animation is triggered
    let velocity = 100.0 * time.delta_seconds();
    if keys.pressed(KeyCode::KeyD) {
        sprite_transformation.translation.x += velocity;
    }
    if keys.pressed(KeyCode::KeyA) {
        sprite_transformation.translation.x -= velocity;
    }
    if keys.pressed(KeyCode::KeyW) {
        sprite_transformation.translation.y += velocity;
    }
    if keys.pressed(KeyCode::KeyS) {
        sprite_transformation.translation.y -= velocity;
    }
}
