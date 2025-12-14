use bevy::{gizmos::grid, prelude::*};

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
        .add_systems(Startup, (setup_grid, spawn_t_block).chain())
        .add_systems(Update, move_block)
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

    let mut pos = IVec2::new(5, 12);

    for shape in &blocks {
        info!("Block Shape: {:?}", shape);
        let translation = grid.grid_to_world(pos.x, pos.y);
        let origin = IVec2::new(GRID_WITH as i32 / 2, GRID_HEIGHT as i32 - 2);
        let block_entity = commands.spawn((
            Transform::from_translation(translation),
            Visibility::default(),
            Block{ shape: *shape },
            BlockState{ state: BlockStateEnum::Falling },
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
            grid.set(cell.x, cell.y, Some(*shape));
        }
        pos.y -= 3;
    }
}

fn spawn_t_block(mut commands: Commands, mut grid: ResMut<Grid>) {
    let shape = BlockShape::T;
    let pos = IVec2::new(5, 12);
    let translation = grid.grid_to_world(pos.x, pos.y);
    let origin = IVec2::new(GRID_WITH as i32 / 2, GRID_HEIGHT as i32 - 2);

    let block_entity = commands.spawn((
        Transform::from_translation(translation),
        Visibility::default(),
        Block{ shape },
        BlockState{ state: BlockStateEnum::Falling },
        Position { x: pos.x, y: pos.y },
    )).id();

    for offset in shape.offsets() {
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
    }
    grid.set(pos.x, pos.y, Some(shape));
}

fn move_block(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    block_query: Single<(&mut Transform, &mut Position, &BlockState), With<Block>>,
    mut grid: ResMut<Grid>,
) {
    let (mut transform, mut position, block_state) = block_query.into_inner();
    match grid.idx(position.x, position.y) {
        Some(_) => {
            let old_position = position.clone();
            let mut new_position = position.clone();
            if let BlockStateEnum::Falling = block_state.state {
                if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                    new_position.x -= 1;
                }
                if keyboard_input.just_pressed(KeyCode::ArrowRight) {
                    new_position.x += 1;
                }
                if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                    new_position.y -= 1;
                }

                let new_translation = grid.grid_to_world(new_position.x, new_position.y);
                if grid.can_move(old_position.x, old_position.y, new_position.x, new_position.y) {
                    grid.move_shape(old_position.x, old_position.y, new_position.x, new_position.y);
                    position.x = new_position.x;
                    position.y = new_position.y;
                    transform.translation = new_translation;
                }
            }
        },
        _ => {}
    }
}