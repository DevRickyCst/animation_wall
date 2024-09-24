mod components;

use crate::components::*;
use bevy::prelude::*;



fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_plugins(DefaultPlugins)
        .add_plugins(ComponentsPlugin) // Ajout du plugin custom
        .run();
}


fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}