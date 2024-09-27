use bevy::prelude::*;
pub mod game;
mod systems;
use systems::setup_camera;

use game::GamePlugin;

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .init_state::<AppState>()
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    Game,
}
