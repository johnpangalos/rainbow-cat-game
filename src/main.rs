use bevy::prelude::*;

#[derive(Component)]
struct Player {
    x: f32,
    y: f32,
}

fn main() {
    App::new().run();
}
