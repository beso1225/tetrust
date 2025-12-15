use bevy::prelude::*;

mod game;
use game::prelude::*;
use game::plugins::debug::DebugPlugin;
use game::system::physics::block_movement;
use game::system::spawn::spawn_block;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetrust".into(),
                resolution: (600, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DebugPlugin)
        .insert_resource(AutoMoveTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, setup_camera)
        // .add_systems(Startup, (setup_grid, spawn_blocks).chain())
        .add_systems(Startup, (
            setup_grid,
            spawn_block::spawn_l_block,
            spawn_block::spawn_t_block,
        ).chain())
        .add_systems(Update, block_movement::move_block_manual)
        .add_systems(Update, block_movement::move_block_auto)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_grid(mut commands: Commands) {
    commands.insert_resource(Grid::new());
}