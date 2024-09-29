use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct AnimalExemple {
    pub positions: Vec<Vec2>,
    pub radii: Vec<f32>,
    pub shape: Vec<Vec2>,
}

// Direction of the animal (1st point)
#[derive(Component)]
pub struct MovementDirection {
    pub angle: f32,
}

#[derive(Component)]
pub struct RingTag;

#[derive(Component)]
pub struct CircleTag;

fn compute_radius(animal: Animals) -> Vec<f32> {
    let lizard_radius = vec![
        14.0, 29.0, 20.0, 30.0, 34.0, 35.5, 32.5, 14.0, 7.5, 5.5, 4.5, 3.5, 3.5,
    ];

    let mut snake_radius = vec![25.0, 30.0];

    let mut value = 25.0;
    for _ in 0..30 {
        value -= 0.75;
        snake_radius.push(value);
    }

    match animal {
        Animals::Lizard => lizard_radius,
        Animals::Snake => snake_radius,
    }
}


pub fn spawn_components(
    mut commands: Commands,
    mut query: Query<(Entity, &AnimalExemple)>,
    current_animal: Res<State<AnimalsRace>>,
) {
    for (entity, _) in query.iter_mut() {
        commands.entity(entity).despawn();
    }

    let default_animal = match current_animal.get() {
        AnimalsRace::Snake => Animals::Snake,
        AnimalsRace::Lizard => Animals::Lizard,
    };
    let distance = 25.0;

    let radii_values: Vec<f32> = compute_radius(default_animal);

    let start_position = Vec2::new(200.0, 0.0);
    println!("Components spawn");

    let mut positions = Vec::new();
    let mut radii = Vec::new();

    for i in 0..radii_values.len() {
        let new_position = Vec2::new(
            start_position.x - i as f32 * (radii_values[i] + distance),
            start_position.y,
        );
        positions.push(new_position);
        radii.push(radii_values[i]);
    }

    let shape = compute_animal_shape(&positions, &radii);

    commands.spawn((
        AnimalExemple {
            positions: positions,
            radii,
            shape,
        },
        MovementDirection { angle: 0.0 },
    ));
}
