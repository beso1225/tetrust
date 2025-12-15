use bevy::prelude::*;
use crate::game::prelude::*;

pub fn show_grid(keyboard_input: Res<ButtonInput<KeyCode>>, grid: Res<Grid>) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        // for y in 0..grid.height() as i32 {
        //     for x in 0..grid.width() as i32 {
        //         let cell = grid.idx(x, y);
        //         if let Some(Some(entity)) = cell {
        //             info!("Cell ({}, {}) : {:?}", x, y, entity);
        //         }
        //     }
        // }
        grid.show();
    }
}