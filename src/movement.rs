use bevy::prelude::*;
use crate::animation::{AnimationConfig, PlayerState, FIRST_IDLE, LAST_IDLE, FPS_IDLE, FIRST_RUNNING, LAST_RUNNING, FPS_RUNNING};


#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct FacingDirection {
    pub facing_right: bool,
}

pub fn character_movement(
    mut query: Query<(&mut Transform, &mut PlayerState, &mut AnimationConfig, &mut Sprite, &mut FacingDirection), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player_state, mut config, mut sprite, mut facing) in query.iter_mut() {
        let mut moving = false;
        let mut moving_horizontally = false;
        let mut moving_right = false;

        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 100.0 * time.delta_secs();
            moving = true;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 100.0 * time.delta_secs();
            moving = true;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100.0 * time.delta_secs();
            moving = true;
            moving_horizontally = true;
            moving_right = false;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100.0 * time.delta_secs();
            moving = true;
            moving_horizontally = true;
            moving_right = true;
        }

        // Update sprite flipping based on direction
        if moving_horizontally {
            // Only update if direction changed
            if facing.facing_right != moving_right {
                facing.facing_right = moving_right;
                sprite.flip_x = !moving_right; // Flip when moving left
            }
        }

        // Change state based on movement
        let current_state = *player_state;
        match (current_state, moving) {
            (PlayerState::Idle, true) => {
                // Change to running animation
                *player_state = PlayerState::Running;
                *config = AnimationConfig::new(FIRST_RUNNING, LAST_RUNNING, FPS_RUNNING);
            },
            (PlayerState::Running, false) => {
                // Change to idle animation
                *player_state = PlayerState::Idle;
                *config = AnimationConfig::new(FIRST_IDLE, LAST_IDLE, FPS_IDLE);
            },
            _ => {} // No state change needed
        }
    }
}