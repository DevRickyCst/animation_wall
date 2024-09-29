use bevy::prelude::*;

use super::*;

pub fn move_components(
    time: Res<Time>,
    mut query: Query<(&mut AnimalExemple, &mut MovementDirection)>,
) {
    let delta_time = time.delta_seconds();
    let speed = 100.0;

    for (mut component, direction) in query.iter_mut() {
        if let Some(first_position) = component.positions.get_mut(0) {
            // Calculer la nouvelle position selon l'angle
            first_position.x += direction.angle.cos() * speed * delta_time;
            first_position.y += direction.angle.sin() * speed * delta_time;
        }

        constraint_following_points(&mut component.positions, 25.0);

        let new_shape = compute_animal_shape(&component.positions, &component.radii);

        component.shape = new_shape;
    }
}

fn constraint_following_points(positions: &mut Vec<Vec2>, distance: f32) {
    for i in 1..positions.len() {
        let prev_position = positions[i - 1];
        let current_position = positions[i];

        // Calculer la direction vers laquelle le point doit se déplacer
        let direction = (prev_position - current_position).normalize_or_zero();

        positions[i] = prev_position - direction * distance;
    }
}

pub fn compute_animal_shape(positions: &Vec<Vec2>, radii: &Vec<f32>) -> Vec<Vec2> {
    let mut shape_positions = Vec::new();

    for (i, &position) in positions.iter().enumerate() {
        let radius = radii[i];

        // If last position then compare with previus
        let compare_position = match i < positions.len() - 1 {
            true => positions[i + 1],
            false => positions[i - 1],
        };

        // Compute direction according to compare position
        let direction = (compare_position - position).normalize();

        // Compute different angle
        let normal_perp = Vec2::new(-direction.y, direction.x); // Perpendiculaire
        let normal_par = Vec2::new(direction.x, direction.y); // Paralèlle
        
        // Si premier point alors on push des points en plus
        if i == 0 {
            let top_point = position - normal_par * radius;
            shape_positions.push(top_point);
        }

        // Push les points a gauches et droites
        let left_point = position + normal_perp * radius;
        shape_positions.push(left_point);
        let right_point = position - normal_perp * radius;
        shape_positions.push(right_point);

        // Si dernier alors on push de points en plus
        if i == positions.len() - 1 {
            let bottom_point = position - normal_par * radius;
            shape_positions.push(bottom_point);
        }
    }

    shape_positions
}