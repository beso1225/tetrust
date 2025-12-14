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

fn spawn_blocks(mut commands: Commands, grid: Res<Grid>) {
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
            Block{
                shape: *shape,
                state: BlockState::Falling,
            },
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
            ));
        }
        pos.y -= 3;
    }
}

fn spawn_t_block(mut commands: Commands, grid: Res<Grid>) {
    let shape = BlockShape::T;
    let pos = IVec2::new(5, 12);
    let translation = grid.grid_to_world(pos.x, pos.y);
    let origin = IVec2::new(GRID_WITH as i32 / 2, GRID_HEIGHT as i32 - 2);

    let block_entity = commands.spawn((
        Transform::from_translation(translation),
        Visibility::default(),
        Block{
            shape,
            state: BlockState::Falling,
        },
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
}

fn move_block(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut block_transform: Single<&mut Transform, With<Block>>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            block_transform.translation.x -= CELL_SIZE;
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            block_transform.translation.x += CELL_SIZE;
    }
}