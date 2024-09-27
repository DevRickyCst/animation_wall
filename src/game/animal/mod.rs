mod components;
mod input;
mod systems;
mod visuels;

use visuels::*;

use crate::game::SimulationState;
use bevy::prelude::*;
use components::*;
use input::*;
use systems::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AnimalsRace>()
            .add_systems(OnEnter(AnimalsRace::Snake), spawn_components) // Runs when entering Snake state
            .add_systems(OnEnter(AnimalsRace::Lizard), spawn_components)
            .add_systems(
                Update,
                (
                    move_right_system,
                    draw_ring_system,
                    handle_input_system,
                    draw_shape_circle_system,
                    toggle_animals_race,
                )
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

pub enum Animals {
    Lizard,
    Snake,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AnimalsRace {
    Snake,
    #[default]
    Lizard,
}
