mod animation;
mod movement;

use bevy::prelude::*;
use animation::{/*AnimationConfig,*/ execute_animations/*, PlayerState*/};
use movement::{character_movement/*, Player*/, FacingDirection};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Misspelled".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, execute_animations))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d::default());

    // Load the character atlas
    let texture = asset_server.load("characters_atlas.png");
    // Create the texture atlas layout (9 columns, 10 rows, 16x32 sprites)
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 32), 9, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // Idle animation uses indices 36-39
    let idle_animation_config = animation::AnimationConfig::new(36, 39, 6);

    // Spawn the player with animation components
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 36,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(5.0)),
        movement::Player,
        animation::PlayerState::Idle,
        FacingDirection { facing_right: true},
        idle_animation_config,
    ));
}