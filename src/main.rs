use crate::game::collision::*;
use crate::game::ground::*;
use crate::game::player::*;
use bevy::{prelude::*, render::camera::ScalingMode, sprite::TextureAtlasLayout};
use game::walls::spawn_wall;

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
                check_player_wall_collision,
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
