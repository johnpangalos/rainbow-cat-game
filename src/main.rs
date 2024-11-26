use crate::game::collision::*;
use crate::game::ground::*;
use crate::game::player::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, render::camera::ScalingMode, sprite::TextureAtlasLayout};
use game::walls::spawn_wall;

mod game;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum MovementSet {
    Input,
    Physics,
    Animation,
    Collision,
}

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
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default()) // This will print FPS to console
        .add_systems(Startup, setup)
        .configure_sets(
            Update,
            (
                MovementSet::Input,
                MovementSet::Physics,
                MovementSet::Animation,
                MovementSet::Collision,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                player_movement.in_set(MovementSet::Input),
                apply_velocity.in_set(MovementSet::Physics),
                (
                    animate_sprite.after(apply_velocity),
                    check_player_wall_collision.after(apply_velocity),
                )
                    .in_set(MovementSet::Animation),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn camera with default settings first
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 800.0,
        min_height: 600.0,
    };

    commands.spawn(camera);

    spawn_ground(&mut commands, &asset_server);
    spawn_player(&mut commands, &asset_server, texture_atlas_layouts);
    spawn_wall(&mut commands);
}
