use bevy::prelude::*;

mod game;
use game::prelude::*;
use game::plugins::debug::DebugPlugin;
use game::system::physics::block_movement;
use game::system::spawn::spawn_block;
use game::system::ui::next_block_preview;
use game::system::ui::walls;

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
        .insert_resource(BlockBag::new())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, (
            setup_grid,
            walls::spawn_walls,
            spawn_first_block,
            next_block_preview::spawn_next_block_preview,
        ).chain())
        .add_systems(Update, block_movement::move_block_manual)
        .add_systems(Update, block_movement::move_block_auto)
        .add_systems(Update, next_block_preview::update_next_block_preview)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_grid(mut commands: Commands) {
    commands.insert_resource(Grid::new());
}

fn spawn_first_block(commands: Commands, grid: ResMut<Grid>, mut bag: ResMut<BlockBag>) {
    let shape = bag.next();
    spawn_block::spawn_block(shape, commands, grid);
}