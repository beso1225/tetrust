use bevy::prelude::*;
use crate::game::prelude::*;

pub fn move_block_manual(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut block_query: Query<(Entity, &mut Transform, &mut Position, &BlockState), With<Block>>,
    mut grid: ResMut<Grid>,
) {
    for (entity, mut transform, mut position, block_state) in block_query.iter_mut() {
        match block_state.state {
            BlockStateEnum::Landed => continue,
            BlockStateEnum::Falling => {}
        }
        let mut new_x = position.x;
        let mut new_y = position.y;

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            new_x -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            new_x += 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            new_y -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            new_y += 1;
        }

        if grid.can_move_entity(entity, block_state.shape, position.x, position.y, new_x, new_y) {
            grid.move_entity(entity, block_state.shape, position.x, position.y, new_x, new_y);
            position.x = new_x;
            position.y = new_y;
            transform.translation = grid.grid_to_world(new_x, new_y);
        }
    }
}