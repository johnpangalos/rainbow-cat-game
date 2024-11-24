use crate::game::constants::*;
use crate::game::ground::*;
use crate::game::player::*;
use crate::game::utils::*;
use bevy::{prelude::*, render::camera::ScalingMode, sprite::TextureAtlasLayout};

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800.0, 600.0).into(),
                title: "Rainbow Cat Game".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                apply_velocity,
                animate_sprite,
                follow_player,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn camera with default settings first
    let mut camera = Camera2dBundle::default();

    // Then modify the projection
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 800.0,
        min_height: 600.0,
    };

    commands.spawn(camera);

    spawn_ground(&mut commands, &asset_server);

    let texture = asset_server.load("cat-walking.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, GROUND_HEIGHT, 1.0),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {
            speed: PLAYER_SPEED,
            jump_force: JUMP_FORCE,
            is_grounded: true,
        },
        Velocity(Vec2::ZERO),
    ));
}
