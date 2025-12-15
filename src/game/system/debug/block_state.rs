use bevy::prelude::*;
use crate::game::prelude::*;

pub fn debug_block_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    block_query: Query<(Entity, &BlockState), With<Block>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyB) {
        for (entity, block_state) in block_query.iter() {
            info!("Block Entity: {:?}, State: {:?}", entity, block_state.state);
        }
    }
}