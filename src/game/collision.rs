use crate::game::player::*;
use crate::game::walls::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

pub fn check_player_wall_collision(
    mut player_query: Query<(&mut Transform, &Collider, &mut Player), With<Player>>,
    wall_query: Query<(&Transform, &Collider), (With<Wall>, Without<Player>)>,
) {
    let (mut player_transform, player_collider, mut player) = player_query.single_mut();

    player.is_touching_wall = false;
    for (wall_transform, wall_collider) in wall_query.iter() {
        let collision = check_collision(
            player_transform.translation.truncate(),
            player_collider.size,
            wall_transform.translation.truncate(),
            wall_collider.size,
        );

        if collision {
            player.is_touching_wall = true;

            let player_center = player_transform.translation.truncate();
            let wall_center = wall_transform.translation.truncate();

            let x_overlap = (player_collider.size.x + wall_collider.size.x) * 0.5
                - (player_center.x - wall_center.x).abs();
            if player_center.x > wall_center.x {
                player_transform.translation.x += x_overlap
            } else {
                player_transform.translation.x -= x_overlap;
            }
        }
        println!("player is touching wall: {:?}", player.is_touching_wall);
    }
}

fn check_collision(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
    let min1 = pos1 - size1 * 0.5;
    let max1 = pos1 + size1 * 0.5;
    let min2 = pos2 - size2 * 0.5;
    let max2 = pos2 + size2 * 0.5;

    max1.x > min2.x && min1.x < max2.x && max1.y > min2.y && min1.y < max2.y
}
