use bevy::prelude::*;

use crate::game::prelude::*;

pub fn spawn_walls(
    mut commands: Commands,
) {
    commands.spawn(Wall::new(WallPosition::Left));
    commands.spawn(Wall::new(WallPosition::Right));
    commands.spawn(Wall::new(WallPosition::Bottom));
}