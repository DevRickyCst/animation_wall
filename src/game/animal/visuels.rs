use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use super::components::*;

use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

fn create_ring_mesh(inner_radius: f32, outer_radius: f32, resolution: usize) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();

    for i in 0..resolution {
        let angle = i as f32 * std::f32::consts::PI * 2.0 / resolution as f32;
        let (sin, cos) = angle.sin_cos();

        positions.push([outer_radius * cos, outer_radius * sin, 0.0]);
        uvs.push([cos, sin]);

        positions.push([inner_radius * cos, inner_radius * sin, 0.0]);
        uvs.push([cos, sin]);
    }

    for i in 0..resolution {
        let next = (i + 1) % resolution;
        let outer1 = i * 2;
        let inner1 = i * 2 + 1;
        let outer2 = next * 2;
        let inner2 = next * 2 + 1;

        indices.push(outer1 as u32);
        indices.push(inner1 as u32);
        indices.push(inner2 as u32);

        indices.push(outer1 as u32);
        indices.push(inner2 as u32);
        indices.push(outer2 as u32);
    }

    // Create the mesh
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList, // Specify the topology
        RenderAssetUsages::all(),        // Specify all usages (or customize as needed)
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

pub fn draw_ring_system(
    mut commands: Commands,
    animal_query: Query<&AnimalExemple>,
    ring_query: Query<Entity, With<RingTag>>, // Rechercher les entités avec le tag ringTag
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Supprimer les anciens cercles
    for entity in ring_query.iter() {
        commands.entity(entity).despawn();
    }

    // Define color
    let color: Color = Color::srgb(0.1, 0.8, 0.1);
    let resolution = 30; // Adjust as needed for smoothness

    let thickness: f32 = 5.0;

    // Get animalComponent
    let animal = animal_query.single();

    // Iter on each point of my animal
    for (i, position) in animal.positions.iter().enumerate() {
        let inner_radius = animal.radii[i] - thickness;
        let outer_radius = animal.radii[i];

        // Create the ring mesh
        let ring_mesh = create_ring_mesh(inner_radius, outer_radius, resolution);
        let mesh_handle = meshes.add(ring_mesh);

        // Wrap the Handle<Mesh> in Mesh2dHandle
        let mesh_2d_handle = Mesh2dHandle(mesh_handle);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh_2d_handle,
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
                ..default()
            },
            RingTag,
        ));
    }
}

pub fn draw_shape_circle_system(
    mut commands: Commands,
    animal_query: Query<&AnimalExemple>,
    circle_query: Query<Entity, With<CircleTag>>, // Rechercher les entités avec le tag CircleTag
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Supprimer les anciens cercles
    for entity in circle_query.iter() {
        commands.entity(entity).despawn();
    }

    let circle_radius: f32 = 2.0; // Rayon du cercle
    let circle: Mesh2dHandle = Mesh2dHandle(meshes.add(Mesh::from(Circle::new(circle_radius)))); // Utilisation correcte de Mesh::from(shape)

    // Définir la couleur des cercles
    let color: Color = Color::srgb(1.0, 1.0, 1.0);

    // Récupérer l'AnimalExemple
    let animal = animal_query.single();

    // Itérer sur chaque point de la forme de l'animal (shape)
    for position in animal.shape.iter() {
        // Créer une entité avec un cercle à chaque point de shape
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: circle.clone(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 1.0)),
                ..default()
            },
            CircleTag,
        ));
    }
}
