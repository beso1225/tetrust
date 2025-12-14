use bevy::prelude::*;

use crate::game::prelude::*;

#[derive(Clone, Copy, Debug)]
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

pub enum BlockStateEnum {
    Falling,
    Landed,
}

#[derive(Component)]
pub struct BlockState {
    pub state: BlockStateEnum,
}


#[derive(Component)]
pub struct Block {
    pub shape: BlockShape,
}

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}