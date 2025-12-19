use bevy::prelude::*;
use std::collections::HashMap;

use crate::game::prelude::*;

pub fn clear_full_lines(
    mut commands: Commands,
    mut grid: Mut<Grid>,
    mut query: Query<(Entity, &mut Transform, &mut Position, &mut BlockState), With<Block>>,
) {
    let mut full_lines = Vec::new();

    // Detect fully occupied lines
    for y in 0..GRID_HEIGHT {
        let mut is_full = true;
        for x in 0..GRID_WIDTH {
            if let Some(None) = grid.idx(x as i32, y as i32) {
                is_full = false;
                break;
            }
        }
        if is_full {
            full_lines.push(y as i32);
        }
    }

    if full_lines.is_empty() {
        return;
    }

    // Collect all child entities on full lines, then despawn them (keep parents)
    let mut children_to_remove = Vec::new();
    for &line in &full_lines {
        for x in 0..GRID_WIDTH {
            if let Some(Some((_, child))) = grid.idx(x as i32, line) {
                children_to_remove.push(*child);
            }
        }
    }

    for child in children_to_remove {
        commands.entity(child).despawn();
    }

    // Clear grid cells for cleared lines
    for &line in &full_lines {
        for x in 0..GRID_WIDTH {
            grid.clear_cell(x as i32, line);
        }
    }

    // Rebuild grid in one pass to avoid overwrite; track parent drops
    let mut new_cells: [[Option<(Entity, Entity)>; GRID_WIDTH]; GRID_HEIGHT] =
        [[None; GRID_WIDTH]; GRID_HEIGHT];
    let mut parent_drop: HashMap<Entity, i32> = HashMap::new();

    for y in 0..GRID_HEIGHT as i32 {
        let drop = full_lines
            .iter()
            .filter(|&&cleared_y| cleared_y < y)
            .count() as i32;
        for x in 0..GRID_WIDTH as i32 {
            if let Some(Some((parent, child))) = grid.idx(x, y) {
                let parent = *parent;
                let child = *child;
                let new_y = y - drop;

                if drop > 0 {
                    parent_drop
                        .entry(parent)
                        .and_modify(|d| *d = (*d).max(drop))
                        .or_insert(drop);
                }

                if new_y >= 0 {
                    new_cells[new_y as usize][x as usize] = Some((parent, child));
                }
            }
        }
    }

    grid.replace_cells(new_cells);

    // Apply accumulated drop to parent transforms/positions once
    for (parent, drop) in parent_drop {
        if let Ok((_e, mut transform, mut position, _state)) = query.get_mut(parent) {
            position.y -= drop;
            transform.translation = grid.grid_to_world(position.x, position.y);
        }
    }
}
