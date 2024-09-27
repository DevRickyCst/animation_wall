pub mod animal;
mod systems;

use crate::AppState;
use animal::ComponentsPlugin;
use bevy::prelude::*;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_plugins(ComponentsPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
