use bevy::prelude::*;

use crate::game::prelude::*;

#[derive(Resource)]
pub struct Grid {
    cells: [[Option<Entity>; GRID_WITH]; GRID_HEIGHT],
    width: usize,
    height: usize,
    cell_size: f32,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [[None; GRID_WITH]; GRID_HEIGHT],
            width: GRID_WITH,
            height: GRID_HEIGHT,
            cell_size: CELL_SIZE,
        }
    }

    pub fn show(&self) {
        println!("=== Grid State ===");
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                match self.cells[y][x] {
                    Some(_entity) => {
                        print!("[X]");
                    }
                    None => {
                        print!("[ ]");
                    }
                }
            }
            println!();
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_entity(&mut self, x: i32, y: i32, entity: Option<Entity>) {
        if (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y) {
            self.cells[y as usize][x as usize] = entity;
        }
    }

    pub fn can_move(&mut self, old_x: i32, old_y: i32, new_x: i32, new_y: i32) -> bool {
        if let Some(_entity) = self.idx(old_x, old_y).and_then(|s| *s) {
            match self.idx(new_x, new_y) {
                    Some(None) => {return true;}
                _ => {}
            }
        }
        false
    }

    pub fn can_move_entity(&mut self,entity: Entity, shape: BlockShape, old_x: i32, old_y: i32, new_x: i32, new_y: i32) -> bool {
        let old_origin = IVec2::new(old_x, old_y);
        for offset in shape.offsets() {
            let cell = old_origin + *offset;
            if let Some(Some(owner)) = self.idx(cell.x, cell.y) {
                if *owner == entity {
                    let new_cell = IVec2::new(new_x, new_y) + *offset;
                    match self.idx(new_cell.x, new_cell.y) {
                        Some(None) => {}
                        Some(Some(other_entity)) if *other_entity == entity => {}
                        _ => {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn move_entity(&mut self,entity: Entity, shape: BlockShape, old_x: i32, old_y: i32, new_x: i32, new_y: i32) {
        let old_origin = IVec2::new(old_x, old_y);
        for offset in shape.offsets() {
            let cell = old_origin + *offset;
            if let Some(Some(owner)) = self.idx(cell.x, cell.y) {
                if *owner == entity {
                    self.set_entity(cell.x, cell.y, None);
                }
            }
        }

        let new_origin = IVec2::new(new_x, new_y);
        for offset in shape.offsets() {
            let cell = new_origin + *offset;
            self.set_entity(cell.x, cell.y, Some(entity));
        }
    }

    pub fn idx(&self, x: i32, y: i32) -> Option<&Option<Entity>> {
        if (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y) {
            Some(&self.cells[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn grid_to_world(&self, x: i32, y: i32) -> Vec3 {
        let world_x = ((x as f32) - self.width as f32 / 2.0) * self.cell_size;
        let world_y = ((y as f32) - self.height as f32 / 2.0) * self.cell_size;
        Vec3::new(world_x, world_y, 0.0)
    }
}

#[derive(Resource)]
pub struct AutoMoveTimer(pub Timer);

#[derive(Resource)]
pub struct BlockBag {
    bag: Vec<BlockShape>,
}

impl BlockBag {
    pub fn new() -> Self {
        let mut bag = Self::create_new_bag();
        bag.reverse(); // reverse so we can pop from end
        Self { bag }
    }

    fn create_new_bag() -> Vec<BlockShape> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut shapes = vec![
            BlockShape::T,
            BlockShape::I,
            BlockShape::O,
            BlockShape::L,
            BlockShape::J,
            BlockShape::S,
            BlockShape::Z,
        ];
        shapes.shuffle(&mut thread_rng());
        shapes
    }

    pub fn next(&mut self) -> BlockShape {
        if self.bag.is_empty() {
            self.bag = Self::create_new_bag();
            self.bag.reverse();
        }
        self.bag.pop().expect("bag should not be empty")
    }
}