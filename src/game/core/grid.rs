use bevy::prelude::*;
use crate::game::prelude::*;

pub fn setup_grid(mut commands: Commands) {
    commands.insert_resource(Grid::new());
}