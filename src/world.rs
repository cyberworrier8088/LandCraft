use bevy::math::primitives::Cuboid;
use bevy::prelude::*;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    let grass = materials.add(Color::srgb(0.3, 0.8, 0.3));


    for x in 0..64 {
        for z in 0..64 {
            commands.spawn((
                Mesh3d(cube.clone()),
                MeshMaterial3d(grass.clone()),
                Transform::from_xyz(x as f32, 0.0, z as f32),

            ));
        }
    }
}