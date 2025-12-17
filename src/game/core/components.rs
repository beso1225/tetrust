use bevy::prelude::*;

use crate::game::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockShape {
    T,
    I,
    O,
    L,
    J,
    S,
    Z,
}

impl BlockShape {
    pub fn offsets(&self) -> &'static [IVec2; 4] {
        match self {
            BlockShape::T => &T_OFFSETS,
            BlockShape::I => &I_OFFSETS,
            BlockShape::O => &O_OFFSETS,
            BlockShape::L => &L_OFFSETS,
            BlockShape::J => &J_OFFSETS,
            BlockShape::S => &S_OFFSETS,
            BlockShape::Z => &Z_OFFSETS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockStateEnum {
    Falling,
    Landed,
}

#[derive(Component, Debug)]
pub struct BlockState {
    pub state: BlockStateEnum,
    pub shape: BlockShape,
}


#[derive(Component)]
pub struct Block;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct NextBlockPreview;

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Wall;

impl Wall {
    pub fn new(position: WallPosition) -> (Wall, Sprite, Transform) {
        let pos = position.position();
        let size = position.size();
        let color = position.color();

        (
            Wall,
            Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
        )
    }
}

pub enum WallPosition {
    Left,
    Right,
    Bottom,
}

impl WallPosition {
    fn position(&self) -> Vec2 {
        let wall_thickness = CELL_SIZE * 0.5;
        let half_width = (GRID_WIDTH as f32 / 2.0) * CELL_SIZE;
        let half_height = (GRID_HEIGHT as f32 / 2.0) * CELL_SIZE;

        match self {
            // Left wall: at the left edge of leftmost column, shifted left by wall thickness/2
            WallPosition::Left => {
                let left_edge = -(half_width + CELL_SIZE / 2.0);
                Vec2::new(left_edge - wall_thickness / 2.0, 0.0)
            }
            // Right wall: at the right edge of rightmost column, shifted right by wall thickness/2
            WallPosition::Right => {
                let right_edge = half_width + CELL_SIZE / 2.0 - CELL_SIZE;
                Vec2::new(right_edge + wall_thickness / 2.0, 0.0)
            }
            // Bottom wall: at the bottom edge of bottom row, shifted down by wall thickness/2
            WallPosition::Bottom => {
                let bottom_edge = -(half_height + CELL_SIZE / 2.0);
                Vec2::new(-wall_thickness, bottom_edge - wall_thickness / 2.0)
            }
        }
    }

    fn size(&self) -> Vec2 {
        let grid_height = GRID_HEIGHT as f32 * CELL_SIZE;
        let grid_width = GRID_WIDTH as f32 * CELL_SIZE;
        let wall_thickness = CELL_SIZE * 0.5; // Make walls thinner

        match self {
            // Left/Right walls: height matches grid plus bottom wall connection
            WallPosition::Left | WallPosition::Right => {
                Vec2::new(wall_thickness, grid_height + CELL_SIZE)
            }
            // Bottom wall: spans from left wall's left edge to right wall's right edge
            WallPosition::Bottom => Vec2::new(grid_width + wall_thickness * 2.0, wall_thickness),
        }
    }

    fn color(&self) -> Color {
        Color::srgb(0.3, 0.3, 0.3)
    }
}