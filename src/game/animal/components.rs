use bevy::prelude::*;

pub enum Animals {
    Lizard,
    Snake,
}

#[derive(Component)]
pub struct AnimalExemple {
    pub positions: Vec<Vec2>,
    pub radii: Vec<f32>,
    pub shape: Vec<Vec2>,
}

// Direction of the animal (1st point)
#[derive(Component)]
pub struct MovementDirection {
    pub angle: f32, // Angle de déplacement en radians
}

#[derive(Component)]
pub struct RingTag;

#[derive(Component)]
pub struct CircleTag;

fn compute_radius(animal: Animals) -> Vec<f32> {
    let lizard_radius = vec![
        14.0, 29.0, 20.0, 30.0, 34.0, 35.5, 32.5, 14.0, 7.5, 5.5, 4.5, 3.5, 3.5,
    ];

    let mut snake_radius = vec![25.0, 30.0]; // Commence avec 25 et 30

    let mut value = 25.0; // Valeur initiale
    for _ in 0..30 {
        // Répéter 40 fois
        value -= 0.75; // Soustraire 0.25
        snake_radius.push(value); // Ajouter la valeur au Vec
    }

    match animal {
        Animals::Lizard => lizard_radius,
        Animals::Snake => snake_radius,
    }
}

pub fn compute_animal_shape(positions: &Vec<Vec2>, radii: &Vec<f32>) -> Vec<Vec2> {
    let mut shape_positions = Vec::new();

    for (i, &position) in positions.iter().enumerate() {
        let radius = radii[i];

        // Pour chaque point, calculer les positions à gauche et à droite du cercle
        // Utilisons la normale pour trouver la direction des points gauche et droite.
        // Supposons que les points sont ordonnés en ligne droite.

        let compare_position = match i < positions.len() - 1 {
            true => positions[i + 1], // Comparer avec la prochaine position si on n'est pas au dernier élément
            false => positions[i - 1], // Revenir à la première position si on est au dernier élément
        };

        // Calculer le vecteur directionnel entre le point actuel et le suivant
        let direction = (compare_position - position).normalize();

        // Calculer la normale (perpendiculaire) au vecteur directionnel
        let normal_perp = Vec2::new(-direction.y, direction.x);
        let normal_par = Vec2::new(direction.x, direction.y);
        if i == 0 {
            // Premier point : placer un point en haut
            let top_point = position - normal_par * radius;
            shape_positions.push(top_point);
        }

        // Calculer les points à gauche et à droite en utilisant la normale et le rayon
        let left_point = position + normal_perp * radius;
        let right_point = position - normal_perp * radius;

        // Ajouter les points calculés à gauche et à droite dans le vecteur de positions finales
        shape_positions.push(left_point);
        shape_positions.push(right_point);


    
        if i == positions.len() - 1 {
            let bottom_point = position - normal_par * radius; // Point en bas
            shape_positions.push(bottom_point);
        }
    }

    shape_positions
}

pub fn spawn_components(mut commands: Commands) {
    let default_animal = Animals::Snake;

    // Distance fixe entre chaque point
    let distance = 25.0;

    // Définir ici les rayons des cercles
    let radii_values: Vec<f32> = compute_radius(default_animal);

    // Point de départ
    let start_position = Vec2::new(200.0, 0.0);
    println!("Components spawn");

    // Générer 5 points distancés de `distance` unités
    let mut positions = Vec::new();
    let mut radii = Vec::new();

    for i in 0..radii_values.len() {
        let new_position = Vec2::new(
            start_position.x - i as f32 * (radii_values[i] + distance),
            start_position.y,
        );
        positions.push(new_position);
        radii.push(radii_values[i]); // Ajouter le rayon correspondant
    }

    let shape = compute_animal_shape(&positions, &radii);
    // Créer une entité avec le cercle, sa couleur et sa position
    // Ajouter le composant contenant les positions
    commands.spawn((
        AnimalExemple {
            positions: positions,
            radii,
            shape,
        },
        MovementDirection { angle: 0.0 }, // Angle initial
    ));
}
