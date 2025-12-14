use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, spawn_block)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

enum BlockShape {
    T,
    I,
    O,
    L,
    J,
    S,
    Z,
}

#[derive(Component)]
struct Block {
    shape: BlockShape,
}

fn spawn_block(mut commands: Commands) {
    commands.spawn((
        Sprite {
            ..default()
        },
        Transform {
        translation: Vec2::ONE.extend(0.0),
        scale: Vec3::new(30.0, 40.0, 1.0),
        ..default()
        },
        Block {
            shape: BlockShape::T,
        },
    ));
}