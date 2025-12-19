use crate::game::core::camera;
use crate::game::core::grid;
use crate::game::system::ui::{next_block_preview, spawn_block, walls};
use bevy::prelude::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::setup_camera).add_systems(
            Startup,
            (
                grid::setup_grid,
                walls::spawn_walls,
                spawn_block::spawn_block,
                next_block_preview::spawn_next_block_preview,
            )
                .chain(),
        );
    }
}
