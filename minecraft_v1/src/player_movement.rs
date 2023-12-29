use bevy::prelude::*;

const _PLAYER_SPEED: f32 = 15.0;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, _app: &mut App) {
        
    }
}

fn _move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    for mut transform in query.iter_mut() {
        let mut translation = transform.translation;
        if input.pressed(KeyCode::W) {
            let forward = transform.forward();
            translation += forward * time.delta_seconds() * _PLAYER_SPEED;
        }
        if input.pressed(KeyCode::S) {
            let forward = transform.forward();
            translation -= forward * time.delta_seconds() * _PLAYER_SPEED;
        }
        if input.pressed(KeyCode::A) {
            let left = transform.left();
            translation += left * time.delta_seconds() * _PLAYER_SPEED;
        }
        if input.pressed(KeyCode::D) {
            let right = transform.right();
            translation += right * time.delta_seconds() * _PLAYER_SPEED;
        }
        transform.translation = translation;
    }
}