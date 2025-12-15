use bevy::prelude::*;
use crate::game::{prelude::*, system::spawn::spawn_block};

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

pub fn move_block_auto(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<AutoMoveTimer>,
    mut block_query: Query<(Entity, &mut Transform, &mut Position, &mut BlockState), With<Block>>,
    mut grid: ResMut<Grid>,
    mut bag: ResMut<BlockBag>,
) {
    // 1 秒に 1 回だけ処理するタイマー
    if !timer.0.tick(time.delta()).is_finished() {
        return;
    }

    let mut need_spawn_new = false;
    for (entity, mut transform, mut position, mut block_state) in block_query.iter_mut() {
        match block_state.state {
            BlockStateEnum::Landed => continue,
            BlockStateEnum::Falling => {}
        }

        let new_x = position.x;
        let new_y = position.y - 1; // 下方向へ 1 マス

        if grid.can_move_entity(entity, block_state.shape, position.x, position.y, new_x, new_y) {
            grid.move_entity(entity, block_state.shape, position.x, position.y, new_x, new_y);
            position.x = new_x;
            position.y = new_y;
            transform.translation = grid.grid_to_world(new_x, new_y);
        } else {
            // if cannot move down, change state to Landed and request spawn
            block_state.state = BlockStateEnum::Landed;
            need_spawn_new = true;
            break;
        }
    }

    if need_spawn_new {
        // Spawn a new falling block at the initial spawn position
        let shape = bag.next();
        spawn_block::spawn_block(shape, commands, grid);
    }
}