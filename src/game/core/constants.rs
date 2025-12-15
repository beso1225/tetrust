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

pub const INITIAL_SPAWN_GRID_Y: i32 = GRID_HEIGHT as i32 - 3;
pub const INITIAL_SPAWN_GRID_X: i32 = (GRID_WITH / 2) as i32 - 1;

// definitions of block colors
pub const T_COLOR: Color = Color::srgb(0.6, 0.0, 0.6);
pub const I_COLOR: Color = Color::srgb(0.0, 0.6, 0.6);
pub const O_COLOR: Color = Color::srgb(0.6, 0.6, 0.0);
pub const L_COLOR: Color = Color::srgb(0.6, 0.3, 0.2);
pub const J_COLOR: Color = Color::srgb(0.2, 0.3, 0.6);
pub const S_COLOR: Color = Color::srgb(0.2, 0.6, 0.2);
pub const Z_COLOR: Color = Color::srgb(0.6, 0.2, 0.2);