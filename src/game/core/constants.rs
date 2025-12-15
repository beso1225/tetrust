use bevy::prelude::*;

pub const GRID_WITH: usize = 10;
pub const GRID_HEIGHT: usize = 24;
pub const CELL_SIZE: f32 = 24.0;

// definitions of block offsets
pub const T_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(-1, 0), IVec2::new(1, 0), IVec2::new(0, 1)];
pub const I_OFFSETS: [IVec2; 4] = [IVec2::new(-2, 0), IVec2::new(-1, 0), IVec2::new(0, 0), IVec2::new(1, 0)];
pub const O_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(1, 0), IVec2::new(0, 1), IVec2::new(1, 1)];
pub const L_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(-1, 0), IVec2::new(1, 0), IVec2::new(1, 1)];
pub const J_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(-1, 0), IVec2::new(1, 0), IVec2::new(-1, 1)];
pub const S_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(1, 0), IVec2::new(0, 1), IVec2::new(-1, 1)];
pub const Z_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(-1, 0), IVec2::new(0, 1), IVec2::new(1, 1)];