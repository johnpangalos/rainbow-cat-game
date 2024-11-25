use crate::game::collision::*;
use crate::game::constants::*;
use crate::game::utils::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    pub is_grounded: bool,
    pub is_touching_wall: bool,
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (player, mut velocity, mut transform) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        match keyboard {
            ref k if k.pressed(KeyCode::KeyA) || k.pressed(KeyCode::ArrowLeft) => {
                direction.x -= 1.0;
                transform.scale.x = 1.0_f32.abs();
            }
            ref k if k.pressed(KeyCode::KeyD) || k.pressed(KeyCode::ArrowRight) => {
                direction.x += 1.0;
                transform.scale.x = -1.0_f32.abs();
            }
            _ => {}
        }
        velocity.0.x = direction.x * player.speed;

        match keyboard {
            ref k if k.just_pressed(KeyCode::Space) => {
                // Make sure player can only jump once
                if !player.is_grounded && !player.is_touching_wall {
                    return;
                };
                velocity.0.y = player.jump_force;
            }
            _ => {}
        }
        velocity.0.y -= 980.0 * time.delta_seconds();
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity, &mut Player)>, time: Res<Time>) {
    for (mut transform, velocity, mut player) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();

        let ground_height = GROUND_HEIGHT;

        match transform.translation.y {
            y if y < ground_height => {
                transform.translation.y = ground_height;
                player.is_grounded = true
            }
            _ => {
                player.is_grounded = false;
            }
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &Velocity,
        &Player,
    )>,
) {
    for (indices, mut timer, mut atlas, velocity, player) in query.iter_mut() {
        if velocity.0.x != 0.0 && player.is_grounded {
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        } else if !player.is_grounded {
            atlas.index = indices.first + 1;
        } else {
            atlas.index = indices.first;
        }
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("cat-walking.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(PLAYER_SIZE), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, GROUND_HEIGHT, 1.0),
            ..default()
        },
        Collider {
            size: Vec2::splat(PLAYER_SIZE as f32),
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
            is_touching_wall: false,
        },
        Velocity(Vec2::ZERO),
    ));
}
