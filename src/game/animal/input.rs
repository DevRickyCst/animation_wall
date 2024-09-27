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
    keyboard_input: Res<ButtonInput<KeyCode>>, // Utilisation correcte de Input<KeyCode>
    mut query: Query<&mut MovementDirection>, // Rechercher les entités avec le composant MovementDirection
) {
    for mut direction in query.iter_mut() {
        // Si la touche flèche gauche est pressée, on réduit l'angle (tourner à gauche)
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.angle += 0.1; // Tourner légèrement vers la gauche
        }

        // Si la touche flèche droite est pressée, on augmente l'angle (tourner à droite)
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.angle -= 0.1; // Tourner légèrement vers la droite
        }
    }
}
