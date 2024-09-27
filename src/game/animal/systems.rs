use bevy::prelude::*;

use super::*;

pub fn move_right_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimalExemple, &mut MovementDirection)>, // Ajout de MovementDirection
) {
    let delta_time = time.delta_seconds();
    let speed = 100.0; // Vitesse de déplacement du premier point

    for (mut component, direction) in query.iter_mut() {
        if let Some(first_position) = component.positions.get_mut(0) {
            // Calculer la nouvelle position selon l'angle
            first_position.x += direction.angle.cos() * speed * delta_time;
            first_position.y += direction.angle.sin() * speed * delta_time;
        }

        // Appel de la fonction pour confiner les autres points comme un serpent
        constrain_points_on_ring(&mut component.positions, 25.0); // Ajuste la distance entre les points

        // Étape 2 : Recalculer la forme (shape) après avoir mis à jour les positions
        let new_shape = compute_animal_shape(&component.positions, &component.radii);

        // Mettre à jour la shape après le calcul
        component.shape = new_shape;
    }
}

fn constrain_points_on_ring(positions: &mut Vec<Vec2>, distance: f32) {
    for i in 1..positions.len() {
        let prev_position = positions[i - 1]; // Récupérer la position du point précédent
        let current_position = positions[i]; // Position actuelle du point

        // Calculer la direction vers laquelle le point doit se déplacer
        let direction = (prev_position - current_position).normalize_or_zero();

        // Calculer la nouvelle position avec la distance
        positions[i] = prev_position - direction * distance;
    }
}
