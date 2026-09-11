// src/world.rs

use crate::mesh::{create_chunk_mesh, BlockType};
use crate::noise::terrain_height;
use crate::player::Player;
use bevy::prelude::*;


use std::collections::HashSet;


#[derive(Resource)]
pub struct BlockAssets {
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
pub struct LoadedChunks {
    pub chunks: HashSet<IVec3>,
}

#[derive(Component)]
pub struct Chunk {
    pub blocks: [BlockType; 16 * 16 * 16], // An array representing the local volume
}

// function for setup world using chunks.
pub fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let atlas_texture = asset_server.load("block/200902092053_terrain.png");

    let block_material = materials.add(StandardMaterial {
        base_color_texture: Some(atlas_texture.clone()),
        ..default()
    });
    
    // We register BlockAssets for compatibility (e.g. highlight or other queries)
    commands.insert_resource(BlockAssets {
        material: block_material.clone(),
    });
}



pub const CHUNK_SIZE: i32 = 16;


// helper function to convert world cooredenetr to chunk

pub fn world_to_chunk(pos: Vec3) -> IVec3 {
    IVec3::new(
        (pos.x / CHUNK_SIZE as f32).floor() as i32,
        (pos.y / CHUNK_SIZE as f32).floor() as i32,
        (pos.z / CHUNK_SIZE as f32).floor() as i32,
    )
}

pub fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    block_material: &Handle<StandardMaterial>,
    chunk_pos: IVec3,
) {
    let mut blocks = [BlockType::Air; 16 * 16 * 16];

    for lx in 0..16 {
        for lz in 0..16 {
            let x_world = chunk_pos.x * 16 + lx as i32;
            let z_world = chunk_pos.z * 16 + lz as i32;
            
            let height = terrain_height(x_world, z_world);
            for ly in 0..16 {
                let y_world = chunk_pos.y * 16 + ly as i32;
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
        },
        Mesh3d(mesh_handle),
        MeshMaterial3d(block_material.clone()),
        Transform::from_translation(chunk_pos.as_vec3() * 16.0),
    ));
}



pub fn update_chunks(
    player: Query<&Transform, With<Player>>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    block_assets: Res<BlockAssets>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let player_chunk = world_to_chunk(player_transform.translation);

    const RENDER_DISTANCE: i32 = 1;

    for x in -RENDER_DISTANCE..=RENDER_DISTANCE {
        for z in -RENDER_DISTANCE..=RENDER_DISTANCE {
            for y in 0..=2 {
                let chunk_pos = IVec3::new(player_chunk.x + x, y, player_chunk.z + z);

                if !loaded_chunks.chunks.contains(&chunk_pos) {
                    spawn_chunk(
                        &mut commands,
                        &mut meshes,
                        &block_assets.material,
                        chunk_pos,
                    );

                    loaded_chunks.chunks.insert(chunk_pos);
                }
            }
        }
    }
}