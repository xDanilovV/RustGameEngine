use bevy::prelude::*;
use std::time::Duration;
//use crate::movement::Player;

pub const FIRST_RUNNING: usize = 40;
pub const LAST_RUNNING: usize = 43;
pub const FPS_RUNNING: u8 = 12;
pub const FIRST_IDLE: usize = 36;
pub const LAST_IDLE: usize = 39;
pub const FPS_IDLE: u8 = 7;
#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub enum PlayerState {
    Idle,
    Running,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}

#[derive(Component, Clone)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    //pub fps: u8,
    pub frame_timer: Timer,
    pub current_frame: usize,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            //fps,
            frame_timer: Self::timer_from_fps(fps),
            current_frame: first,
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}

pub fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // Tick the animation timer
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index >= config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                }

                // Update the current frame tracker
                config.current_frame = atlas.index;
            }
        }
    }
}