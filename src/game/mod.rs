use bevy::app::Plugin;

pub mod prelude;

mod core;
pub mod system;
pub mod plugins;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(plugins::startup::StartupPlugin)
            .add_plugins(plugins::debug::DebugPlugin);
    }
}