use bevy::prelude::*;

mod game;
use game::prelude::*;
use game::system::debug::grid;
use game::system::physics::block_movement;
use game::system::spawn::spawn_block;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetrust".into(),
                resolution: (1200, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        // .add_systems(Startup, (setup_grid, spawn_blocks).chain())
        .add_systems(Startup, (
            setup_grid,
            spawn_block::spawn_l_block,
            spawn_block::spawn_t_block,
        ).chain())
        .add_systems(Update, block_movement::move_block_manual)
        .add_systems(Update, grid::show_grid)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_grid(mut commands: Commands) {
    commands.insert_resource(Grid::new());
}