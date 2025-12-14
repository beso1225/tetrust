use bevy::prelude::{Resource, Vec3};

use crate::game::prelude::*;

#[derive(Resource)]
pub struct Grid {
    cells: [[Option<BlockShape>; GRID_WITH]; GRID_HEIGHT],
    width: usize,
    height: usize,
    cell_size: f32,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [[None; GRID_WITH]; GRID_HEIGHT],
            width: GRID_WITH,
            height: GRID_HEIGHT,
            cell_size: CELL_SIZE,
        }
    }

    pub fn idx(&self, x: i32, y: i32) -> Option<&Option<BlockShape>> {
        if (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y) {
            Some(&self.cells[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn grid_to_world(&self, x: i32, y: i32) -> Vec3 {
        Vec3::new(
            (x as f32 - self.width as f32 / 2.0) * self.cell_size,
            (y as f32 - self.height as f32 / 2.0) * self.cell_size,
            0.0,
        )
    }
}