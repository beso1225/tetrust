use bevy::prelude::*;

use crate::game::prelude::*;

#[derive(Resource)]
pub struct Grid {
    // Each cell stores (parent_entity, child_entity) pair
    cells: [[Option<(Entity, Entity)>; GRID_WIDTH]; GRID_HEIGHT],
    width: usize,
    height: usize,
    cell_size: f32,
}

#[allow(dead_code)]
impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [[None; GRID_WIDTH]; GRID_HEIGHT],
            width: GRID_WIDTH,
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

    pub fn set_entity(&mut self, x: i32, y: i32, parent: Entity, child: Entity) {
        if (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y) {
            self.cells[y as usize][x as usize] = Some((parent, child));
        }
    }

    pub fn clear_cell(&mut self, x: i32, y: i32) {
        if (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y) {
            self.cells[y as usize][x as usize] = None;
        }
    }

    pub fn replace_cells(&mut self, new_cells: [[Option<(Entity, Entity)>; GRID_WIDTH]; GRID_HEIGHT]) {
        self.cells = new_cells;
    }

    pub fn can_move(&mut self, old_x: i32, old_y: i32, new_x: i32, new_y: i32) -> bool {
        if self.idx(old_x, old_y).is_some_and(|s| s.is_some()) {
            match self.idx(new_x, new_y) {
                    Some(None) => {return true;}
                _ => {}
            }
        }
        false
    }

    pub fn can_move_entity(&mut self, entity: Entity, shape: BlockShape, old_x: i32, old_y: i32, new_x: i32, new_y: i32) -> bool {
        let old_origin = IVec2::new(old_x, old_y);
        for offset in shape.offsets() {
            let cell = old_origin + *offset;
            if let Some(Some((owner, _))) = self.idx(cell.x, cell.y) {
                if *owner == entity {
                    let new_cell = IVec2::new(new_x, new_y) + *offset;
                    match self.idx(new_cell.x, new_cell.y) {
                        Some(None) => {}
                        Some(Some((other_entity, _))) if *other_entity == entity => {}
                        _ => {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn move_entity(&mut self, entity: Entity, shape: BlockShape, old_x: i32, old_y: i32, new_x: i32, new_y: i32) {
        // First, collect child entities from old positions
        let mut children_to_move = Vec::new();
        let old_origin = IVec2::new(old_x, old_y);
        for offset in shape.offsets() {
            let cell = old_origin + *offset;
            if let Some(Some((owner, child))) = self.idx(cell.x, cell.y) {
                if *owner == entity {
                    children_to_move.push((*owner, *child));
                    self.clear_cell(cell.x, cell.y);
                }
            }
        }

        // Then set new positions
        let new_origin = IVec2::new(new_x, new_y);
        let mut index = 0;
        for offset in shape.offsets() {
            let cell = new_origin + *offset;
            if index < children_to_move.len() {
                let (parent, child) = children_to_move[index];
                self.set_entity(cell.x, cell.y, parent, child);
                index += 1;
            }
        }
    }

    pub fn idx(&self, x: i32, y: i32) -> Option<&Option<(Entity, Entity)>> {
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

    pub fn despawn(&mut self, mut commands: Commands, x: i32, y: i32) {
        if let Some(Some((_parent, child))) = self.idx(x, y) {
            commands.entity(*child).despawn();
            self.clear_cell(x, y);
        }
    }

    pub fn move_single(&mut self, old_x: i32, old_y: i32, new_x: i32, new_y: i32) {
        if let Some(Some((parent, child))) = self.idx(old_x, old_y) {
            self.set_entity(new_x, new_y, *parent, *child);
            self.clear_cell(old_x, old_y);
        }
    }
}

#[derive(Resource)]
pub struct AutoMoveTimer(pub Timer);

#[derive(Resource)]
pub struct BlockBag {
    bag: Vec<BlockShape>,
    next_bag: Vec<BlockShape>,
}

#[allow(dead_code)]
impl BlockBag {
    pub fn new() -> Self {
        let mut bag = Self::create_new_bag();
        bag.reverse(); // reverse so we can pop from end
        let mut next_bag = Self::create_new_bag();
        next_bag.reverse();
        Self { bag, next_bag }
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
            // Switch to the next bag and prepare a new next_bag
            self.bag = std::mem::take(&mut self.next_bag);
            self.next_bag = Self::create_new_bag();
            self.next_bag.reverse();
        }
        self.bag.pop().expect("bag should not be empty")
    }

    pub fn peek(&mut self) -> BlockShape {
        if self.bag.is_empty() {
            // Switch to the next bag and prepare a new next_bag
            self.bag = std::mem::take(&mut self.next_bag);
            self.next_bag = Self::create_new_bag();
            self.next_bag.reverse();
        }
        *self.bag.last().expect("bag should not be empty")
    }

    pub fn peek_multiple(&self, count: usize) -> Vec<BlockShape> {
        let mut result = Vec::with_capacity(count);
        let mut remaining = count;
        let bag_len = self.bag.len();

        // First, get items from current bag (reversed, so iterate from end)
        let available = bag_len.min(remaining);
        for i in 0..available {
            result.push(self.bag[bag_len - 1 - i]);
        }
        remaining -= available;

        // If we need more, get from next_bag
        if remaining > 0 {
            let next_bag_len = self.next_bag.len();
            let take = next_bag_len.min(remaining);
            for i in 0..take {
                result.push(self.next_bag[next_bag_len - 1 - i]);
            }
            remaining -= take;
        }

        // If we still need more, simulate additional future bags
        while remaining > 0 {
            let future_bag = Self::create_new_bag();
            let take = future_bag.len().min(remaining);
            // Take from the end since we'll reverse it
            for i in (future_bag.len() - take..future_bag.len()).rev() {
                result.push(future_bag[i]);
            }
            remaining -= take;
        }

        result
    }
}