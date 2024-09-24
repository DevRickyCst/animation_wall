use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};



pub struct ComponentsPlugin;


impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_components)
        .add_systems(Update, (move_right_system,draw_circle_system,handle_input_system));
    }
}

#[derive(Component)]
struct ComponentExemple{
    positions: Vec<Vec2>,
}
#[derive(Component)]
struct CircleTag;

#[derive(Component)]
struct MovementDirection {
    angle: f32, // Angle de déplacement en radians
}

fn spawn_components(
    mut commands: Commands,
) {

    // Distance fixe entre chaque point
    let distance = 25.0;

    // Point de départ
    let start_position = Vec2::new(200.0, 0.0);

    // Générer 5 points distancés de `distance` unités
    let mut positions = Vec::new();
    for i in 0..15 {
        let new_position = Vec2::new(start_position.x - i as f32 * distance, start_position.y);
        positions.push(new_position);
    }
    // Créer une entité avec le cercle, sa couleur et sa position
    // Ajouter le composant contenant les positions
    commands.spawn((ComponentExemple { positions: positions},         MovementDirection { angle: 0.0 } // Angle initial
    ));
}


fn draw_circle_system(
    mut commands: Commands,
    query: Query<&ComponentExemple>,
    circle_query: Query<Entity, With<CircleTag>>, // Rechercher les entités avec le tag CircleTag
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // Supprimer les anciens cercles
    for entity in circle_query.iter() {
        commands.entity(entity).despawn();
    }
    let circle_radius: f32 = 10.0; // Rayon du cercle
    let color = Color::srgb(1.0, 1.0, 1.0);
    let circle = Mesh2dHandle(meshes.add(Mesh::from(Circle::new(circle_radius)))); // Utilisation correcte de Mesh::from(shape)

    let component = query.single();
    for position in &component.positions {
        commands.spawn((
            // Ajoute un cercle centré sur chaque point
            MaterialMesh2dBundle {
                mesh: circle.clone(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
                ..default()
            },
            CircleTag
        ));
    }
}


fn constrain_points_on_circle(positions: &mut Vec<Vec2>, distance: f32) {
    for i in 1..positions.len() {
        let prev_position = positions[i - 1]; // Récupérer la position du point précédent
        let current_position = positions[i];  // Position actuelle du point

        // Calculer la direction vers laquelle le point doit se déplacer
        let direction = (prev_position - current_position).normalize_or_zero(); 

        // Calculer la nouvelle position avec la distance
        positions[i] = prev_position - direction * distance;
    }
}


fn move_right_system(
    time: Res<Time>, 
    mut query: Query<(&mut ComponentExemple, &mut MovementDirection)>, // Ajout de MovementDirection
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
        constrain_points_on_circle(&mut component.positions, 25.0); // Ajuste la distance entre les points
    }
}

fn handle_input_system(
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
