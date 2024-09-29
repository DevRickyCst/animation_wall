use super::*;
use bevy::prelude::*;

pub fn toggle_animals_race(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<AnimalsRace>>,
    mut next_state: ResMut<NextState<AnimalsRace>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        match simulation_state.get() {
            AnimalsRace::Snake => next_state.set(AnimalsRace::Lizard),
            AnimalsRace::Lizard => next_state.set(AnimalsRace::Snake),
        };
        println!("Toggle animals race !")
    }
}

pub fn handle_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementDirection>,
) {
    for mut direction in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.angle += 0.1;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.angle -= 0.1;
        }
    }
}
