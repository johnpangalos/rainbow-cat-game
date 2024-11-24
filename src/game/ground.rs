use crate::game::constants::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Ground;

pub fn spawn_ground(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let grass_texture = asset_server.load("grass.png");

    for i in -1..=1 {
        // Spawn multiple ground segments
        commands.spawn((
            SpriteBundle {
                texture: grass_texture.clone(),
                transform: Transform::from_xyz(800.0 * i as f32, GROUND_HEIGHT - 60.0, -1.0),
                ..default()
            },
            Ground,
        ));
    }
}
