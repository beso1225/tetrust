use bevy::prelude::*;
use crate::game::prelude::*;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AutoMoveTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(BlockBag::new());
    }
}