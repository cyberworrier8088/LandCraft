// src/world.rs


use bevy::math::primitives::Cuboid;
use bevy::prelude::*;



#[derive(Component)]
pub struct Block;


#[derive(Resource)]
pub struct BlockAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}



// function for spawn a block. for player placing and delete.
pub fn spawn_block(
    commands: &mut Commands,
    position: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
) {
    commands.spawn((
        Block,
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
    ));
}


// function for setup world.
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    let grass = materials.add(Color::srgb(0.3, 0.8, 0.3));
    
    commands.insert_resource(BlockAssets {
    mesh: cube.clone(),
    material: grass.clone(),
    });

    // it is for creating a 64x64 grid of cubes. 
    // x is maths 
    // y is maths :)
    for x in 0..16 {
        for z in 0..16 {
            spawn_block(
                &mut commands,
                Vec3::new(x as f32, 0.0, z as f32),
                cube.clone(),
                grass.clone(),
            );
        }
    }
}