use bevy::prelude::*;
use crate::game::prelude::*;

/// Spawn the initial next block preview (5 blocks)
pub fn spawn_next_block_preview(
    mut commands: Commands,
    bag: Res<BlockBag>,
) {
    let next_shapes = bag.peek_multiple(5);
    spawn_previews_for_shapes(&next_shapes, &mut commands);
}

/// Update the next block preview when needed (5 blocks)
pub fn update_next_block_preview(
    mut commands: Commands,
    bag: Res<BlockBag>,
    preview_query: Query<Entity, With<NextBlockPreview>>,
) {
    // Only update when BlockBag is actually modified (via next())
    if !bag.is_changed() {
        return;
    }

    // Remove old preview (parent and all children)
    for entity in preview_query.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new preview
    let next_shapes = bag.peek_multiple(5);
    spawn_previews_for_shapes(&next_shapes, &mut commands);
}

fn spawn_previews_for_shapes(shapes: &[BlockShape], commands: &mut Commands) {
    // Spawn 5 block previews vertically
    for (index, &shape) in shapes.iter().enumerate() {
        let color = match shape {
            BlockShape::T => T_COLOR,
            BlockShape::I => I_COLOR,
            BlockShape::O => O_COLOR,
            BlockShape::L => L_COLOR,
            BlockShape::J => J_COLOR,
            BlockShape::S => S_COLOR,
            BlockShape::Z => Z_COLOR,
        };

        // Calculate vertical offset for each preview (spacing them out)
        let y_offset = NEXT_BLOCK_OFFSET_Y - (index as f32 * CELL_SIZE * 4.0);

        // Create parent entity for the preview
        let preview_parent = commands.spawn((
            Transform::from_translation(Vec3::new(NEXT_BLOCK_OFFSET_X, y_offset, 0.0)),
            Visibility::default(),
            NextBlockPreview,
        )).id();

        // Spawn the 4 cells of the block preview
        for offset in shape.offsets() {
            let local = Vec3::new(
                offset.x as f32 * CELL_SIZE,
                offset.y as f32 * CELL_SIZE,
                0.0,
            );
            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(CELL_SIZE * 0.7)), // Smaller for preview
                    ..default()
                },
                Transform::from_translation(local),
                ChildOf(preview_parent),
            ));
        }
    }
}
