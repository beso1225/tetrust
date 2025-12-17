use bevy::prelude::*;
use crate::game::prelude::*;

pub fn spawn_block(shape: BlockShape, mut commands: Commands, mut grid: ResMut<Grid>) {
    let pos = IVec2::new(INITIAL_SPAWN_GRID_X, INITIAL_SPAWN_GRID_Y);
    // check either can spawn
    for offset in shape.offsets() {
        let cell = pos + *offset;
        if let Some(Some(_)) = grid.idx(cell.x, cell.y) {
            // occupied
            info!("Cannot spawn block {:?} at {:?}, occupied", shape, cell);
            return;
        }
    }
    let translation = grid.grid_to_world(pos.x, pos.y);
    let color = match shape {
        BlockShape::T => T_COLOR,
        BlockShape::I => I_COLOR,
        BlockShape::O => O_COLOR,
        BlockShape::L => L_COLOR,
        BlockShape::J => J_COLOR,
        BlockShape::S => S_COLOR,
        BlockShape::Z => Z_COLOR,
    };

    let block_entity = commands.spawn((
        Transform::from_translation(translation),
        Visibility::default(),
        Block,
        BlockState{ state: BlockStateEnum::Falling, shape},
        Position { x: pos.x, y: pos.y },
    )).id();

    for offset in shape.offsets() {
        let cell = pos + *offset;
        let local = Vec3::new(
            (*offset).x as f32 * CELL_SIZE,
            (*offset).y as f32 * CELL_SIZE,
            0.0,
        );
        let child_entity = commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::splat(CELL_SIZE)),
                ..default()
            },
            Transform::from_translation(local),
            ChildBlock,
            ChildOf(block_entity),
        )).id();
        grid.set_entity(cell.x, cell.y, block_entity, child_entity);
    }
}

pub fn spawn_first_block(commands: Commands, grid: ResMut<Grid>, mut bag: ResMut<BlockBag>) {
    let shape = bag.next();
    spawn_block(shape, commands, grid);
}