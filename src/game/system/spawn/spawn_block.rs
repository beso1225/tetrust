use bevy::prelude::*;
use crate::game::prelude::*;

pub fn spawn_t_block(mut commands: Commands, mut grid: ResMut<Grid>) {
    let shape = BlockShape::T;
    let pos = IVec2::new(5, 12);
    let translation = grid.grid_to_world(pos.x, pos.y);

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
        commands.spawn((
            Sprite {
                color: Color::srgb(0.6, 0.2, 0.8),
                custom_size: Some(Vec2::splat(CELL_SIZE)),
                ..default()
            },
            Transform::from_translation(local),
            ChildOf(block_entity),
        ));
        grid.set_entity(cell.x, cell.y, Some(block_entity));
    }
}

pub fn spawn_l_block(mut commands: Commands, mut grid: ResMut<Grid>) {
    let shape = BlockShape::L;
    let pos = IVec2::new(5, 10);
    let translation = grid.grid_to_world(pos.x, pos.y);

    let block_entity = commands.spawn((
        Transform::from_translation(translation),
        Visibility::default(),
        Block,
        BlockState{ state: BlockStateEnum::Landed, shape },
        Position { x: pos.x, y: pos.y },
    )).id();

    for offset in shape.offsets() {
        let cell = pos + *offset;
        let local = Vec3::new(
            (*offset).x as f32 * CELL_SIZE,
            (*offset).y as f32 * CELL_SIZE,
            0.0,
        );
        commands.spawn((
            Sprite {
                color: Color::srgb(0.6, 0.3, 0.2),
                custom_size: Some(Vec2::splat(CELL_SIZE)),
                ..default()
            },
            Transform::from_translation(local),
            ChildOf(block_entity),
        ));
        grid.set_entity(cell.x, cell.y, Some(block_entity));
    }
}

fn spawn_blocks(mut commands: Commands, mut grid: ResMut<Grid>) {
    let blocks = vec![
        BlockShape::T,
        BlockShape::I,
        BlockShape::O,
        BlockShape::L,
        BlockShape::J,
        BlockShape::S,
        BlockShape::Z,
    ];

    let mut pos = IVec2::new(5, 18);

    for shape in &blocks {
        info!("Block Shape: {:?}", shape);
        let translation = grid.grid_to_world(pos.x, pos.y);
        let origin = IVec2::new(GRID_WITH as i32 / 2, GRID_HEIGHT as i32 - 2);
        let block_entity = commands.spawn((
            Transform::from_translation(translation),
            Visibility::default(),
            Block,
            BlockState{ state: BlockStateEnum::Falling, shape: *shape },
            Position { x: pos.x, y: pos.y },
        )).id();

        for offset in shape.offsets() {
            info!("Offset: {:?}", offset);
            let cell = origin + *offset;
            let translation = grid.grid_to_world(cell.x, cell.y);
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.6, 0.2, 0.8),
                    custom_size: Some(Vec2::splat(CELL_SIZE)),
                    ..default()
                },
                Transform::from_translation(translation),
                ChildOf(block_entity),
                Position { x: cell.x, y: cell.y },
            ));
            grid.set_entity(cell.x, cell.y, Some(block_entity));
        }
        pos.y -= 3;
    }
}