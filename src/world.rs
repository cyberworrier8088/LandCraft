// src/world.rs

use crate::mesh::{create_chunk_mesh, BlockType};
use crate::noise::terrain_height;
use bevy::prelude::*;

#[derive(Component)]
pub struct Block;

#[derive(Resource)]
pub struct BlockAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct Chunk {
    pub blocks: [BlockType; 16 * 16 * 16], // An array representing the local volume
    pub position: IVec3,                  // The chunk's coordinate in world space
}

// function for spawn a block. (kept for backwards compatibility if needed elsewhere)
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

// function for setup world using chunks.
pub fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let atlas_texture = asset_server.load("block/200902092053_terrain.png");

    let block_material = materials.add(StandardMaterial {
        base_color_texture: Some(atlas_texture.clone()),
        ..default()
    });
    
    // We register BlockAssets for compatibility (e.g. highlight or other queries)
    commands.insert_resource(BlockAssets {
        mesh: Handle::default(),
        material: block_material.clone(),
    });

    // Spawn a 1x2x1 grid of chunks to cover X: 0..16, Z: 0..16, Y: 0..32
    for cy in 0..2 {
        let chunk_pos = IVec3::new(0, cy, 0);
        let mut blocks = [BlockType::Air; 16 * 16 * 16];
        
        for lx in 0..16 {
            for lz in 0..16 {
                let x_world = lx as i32;
                let z_world = lz as i32;
                let height = terrain_height(x_world, z_world);
                
                for ly in 0..16 {
                    let y_world = ly as i32 + cy * 16;
                    let idx = lx + ly * 16 + lz * 256;
                    
                    if y_world < height {
                        blocks[idx] = BlockType::Cobblestone;
                    } else if y_world == height {
                        blocks[idx] = BlockType::Grass;
                    } else {
                        blocks[idx] = BlockType::Air;
                    }
                }
            }
        }
        
        let chunk_mesh = create_chunk_mesh(&blocks);
        let mesh_handle = meshes.add(chunk_mesh);
        
        commands.spawn((
            Chunk {
                blocks,
                position: chunk_pos,
            },
            Mesh3d(mesh_handle),
            MeshMaterial3d(block_material.clone()),
            Transform::from_translation(chunk_pos.as_vec3() * 16.0),
        ));
    }
}