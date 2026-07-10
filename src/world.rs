// src/world.rs


use crate::mesh::{create_block_mesh, BlockType};
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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let grass_mesh = meshes.add(create_block_mesh(BlockType::Grass));
    let stone_mesh = meshes.add(create_block_mesh(BlockType::Cobblestone));

    let atlas_texture = asset_server.load("block/texture_atlas_cobblestone_grass.png");

    let grass = materials.add(StandardMaterial {
        base_color_texture: Some(atlas_texture.clone()),
        ..default()
    });

    let stone = materials.add(StandardMaterial {
        base_color_texture: Some(atlas_texture.clone()),
        ..default()
    });
    
    commands.insert_resource(BlockAssets {
        mesh: grass_mesh.clone(),
        material: grass.clone(),
    });

    // it is for creating a 64x64 grid of cubes. 
    // x is maths 
    // y is maths :)
    for x in 0..16 {
        for z in 0..16 {

            let height = terrain_height(x, z);
            
            for y in 0..=height {
                let is_grass = y == height;
                spawn_block(
                    &mut commands,

                    Vec3::new(
                        x as f32,
                        y as f32,
                        z as f32,
                    ),
                    if is_grass { grass_mesh.clone() } else { stone_mesh.clone() },
                    if is_grass { grass.clone() } else { stone.clone() },
                );
            }
        }
    }
}


// adding terrain
pub fn terrain_height(x: i32, z: i32) -> i32 {
    let xf = x as f32 * 0.15;
    let zf = z as f32 * 0.15;

    ((xf.sin() * 3.0 + zf.cos() * 3.0) + 6.0) as i32
}