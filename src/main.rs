use bevy::prelude::*;

mod game;
use game::prelude::*;
use game::system::physics::block_movement;
use game::system::ui::next_block_preview;

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
        .add_plugins(GamePlugin)
        .insert_resource(AutoMoveTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(BlockBag::new())
        .add_systems(Update, block_movement::move_block_manual)
        .add_systems(Update, block_movement::move_block_auto)
        .add_systems(Update, next_block_preview::update_next_block_preview)
        .run();
}