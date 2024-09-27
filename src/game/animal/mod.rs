mod components;
mod systems;
mod visuels;

use visuels::*;

use crate::game::SimulationState;
use bevy::prelude::*;
use components::*;
use systems::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_components).add_systems(
            Update,
            (
                move_right_system,
                draw_ring_system,
                handle_input_system,
                draw_shape_circle_system,
            )
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
