use bevy::prelude::*;

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
        .add_systems(Startup, (setup_camera, spawn_block).chain())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(Grid::new());
}

const GRID_WITH: usize = 10;
const GRID_HEIGHT: usize = 20;
const CELL_SIZE: f32 = 24.0;

#[derive(Resource)]
struct Grid {
    cells: [[Option<BlockShape>; GRID_WITH]; GRID_HEIGHT],
    width: usize,
    height: usize,
    cell_size: f32,
}

impl Grid {
    fn new() -> Self {
        Self {
            cells: [[None; GRID_WITH]; GRID_HEIGHT],
            width: GRID_WITH,
            height: GRID_HEIGHT,
            cell_size: CELL_SIZE,
        }
    }

    fn idx(&self, x: i32, y: i32) -> Option<&Option<BlockShape>> {
        if (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y) {
            Some(&self.cells[y as usize][x as usize])
        } else {
            None
        }
    }

    fn grid_to_world(&self, x: i32, y: i32) -> Vec3 {
        Vec3::new(
            (x as f32 - self.width as f32 / 2.0) * self.cell_size,
            (y as f32 - self.height as f32 / 2.0) * self.cell_size,
            0.0,
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum BlockShape {
    T,
    I,
    O,
    L,
    J,
    S,
    Z,
}

const T_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(-1, 0), IVec2::new(1, 0), IVec2::new(0, 1)];
const I_OFFSETS: [IVec2; 4] = [IVec2::new(-2, 0), IVec2::new(-1, 0), IVec2::new(0, 0), IVec2::new(1, 0)];
const O_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(1, 0), IVec2::new(0, 1), IVec2::new(1, 1)];
const L_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(-1, 0), IVec2::new(1, 0), IVec2::new(1, 1)];
const J_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(-1, 0), IVec2::new(1, 0), IVec2::new(-1, 1)];
const S_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(1, 0), IVec2::new(0, 1), IVec2::new(-1, 1)];
const Z_OFFSETS: [IVec2; 4] = [IVec2::new(0, 0), IVec2::new(-1, 0), IVec2::new(0, 1), IVec2::new(1, 1)];

impl BlockShape {
    fn offsets(&self) -> &'static [IVec2; 4] {
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

#[derive(Component)]
struct Block (BlockShape);

fn spawn_block(mut commands: Commands, grid: Res<Grid>) {
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
            InheritedVisibility::default(),
            Block(*shape),
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