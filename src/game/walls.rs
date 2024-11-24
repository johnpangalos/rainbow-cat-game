use crate::game::collision::*;
use crate::game::constants::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Wall;

pub fn spawn_wall(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(SCREEN_WIDTH * -0.5, 0.0, 0.0),
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(WALL_SIZE),
                ..default()
            },
            ..default()
        },
        Wall,
        Collider { size: WALL_SIZE },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(SCREEN_WIDTH * 0.5, 0.0, 0.0),
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(WALL_SIZE),
                ..default()
            },
            ..default()
        },
        Wall,
        Collider { size: WALL_SIZE },
    ));
}
