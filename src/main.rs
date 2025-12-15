use bevy::prelude::*;

mod game;
use game::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetrust".into(),
                resolution: (1200, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        // .add_systems(Startup, (setup_grid, spawn_blocks).chain())
        .add_systems(Startup, (
            setup_grid,
            spawn_l_block,
            spawn_t_block,
        ).chain())
        .add_systems(Update, move_block)
        .add_systems(Update, show_grid)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_grid(mut commands: Commands) {
    commands.insert_resource(Grid::new());
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

fn spawn_t_block(mut commands: Commands, mut grid: ResMut<Grid>) {
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
        info!("Offset: {:?}", offset);
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
        info!("Cell: {:?}", cell);
        grid.set_entity(cell.x, cell.y, Some(block_entity));
    }
}

fn spawn_l_block(mut commands: Commands, mut grid: ResMut<Grid>) {
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
        info!("Offset: {:?}", offset);
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
        info!("Cell: {:?}", cell);
        grid.set_entity(cell.x, cell.y, Some(block_entity));
    }
}

fn move_block(
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

fn show_grid(keyboard_input: Res<ButtonInput<KeyCode>>, grid: Res<Grid>) {
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