use bevy::prelude::*;

pub mod prelude;

mod core;
pub mod system;
pub mod plugins;

use plugins::{startup, debug, resource};
use system::physics::block_movement;
use system::ui::next_block_preview;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(startup::StartupPlugin)
            .add_plugins(debug::DebugPlugin)
            .add_plugins(resource::ResourcePlugin)
            .add_systems(Update, block_movement::move_block_manual)
            .add_systems(Update, block_movement::move_block_auto)
            .add_systems(Update, next_block_preview::update_next_block_preview);
    }
}