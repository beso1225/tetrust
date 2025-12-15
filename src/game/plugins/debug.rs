use bevy::prelude::*;

use crate::game::system::debug;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug::grid::show_grid)
            .add_systems(Update, debug::block_state::debug_block_state);
    }
}