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

#[derive(Resource)]
pub struct RenderSettings {
    pub distance_level: usize, // 0: Tiny (1), 1: Short (2), 2: Normal (3), 3: Far (4)
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self { distance_level: 1 }
    }
}

impl RenderSettings {
    pub fn chunk_distance(&self) -> i32 {
        match self.distance_level {
            0 => 1,
            1 => 2,
            2 => 3,
            _ => 4,
        }
    }

    pub fn cycle(&mut self) {
        self.distance_level = (self.distance_level + 1) % 4;
    }
}

pub const SEA_LEVEL: i32 = 8;

// function for setup world using chunks.
pub fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let atlas_texture = asset_server.load("block/200905192307_terrain.png");

    let block_material = materials.add(StandardMaterial {
        base_color_texture: Some(atlas_texture.clone()),
        perceptual_roughness: 0.9,
        reflectance: 0.1,
        ..default()
    });
    
    // We register BlockAssets for compatibility (e.g. highlight or other queries)
    commands.insert_resource(BlockAssets {
        material: block_material.clone(),
    });
    commands.insert_resource(RenderSettings::default());
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

                if y_world <= 0 {
                    blocks[idx] = BlockType::Bedrock;
                } else if y_world < height - 3 {
                    blocks[idx] = BlockType::Stone;
                } else if y_world < height {
                    blocks[idx] = BlockType::Dirt;
                } else if y_world == height {
                    if y_world < SEA_LEVEL {
                        blocks[idx] = BlockType::Dirt;
                    } else {
                        blocks[idx] = BlockType::Grass;
                    }
                } else if y_world <= SEA_LEVEL {
                    blocks[idx] = BlockType::Water;
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
    render_settings: Res<RenderSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    block_assets: Res<BlockAssets>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let player_chunk = world_to_chunk(player_transform.translation);
    let render_dist = render_settings.chunk_distance();

    for x in -render_dist..=render_dist {
        for z in -render_dist..=render_dist {
            for y in 0..=3 {
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

pub fn cycle_render_distance(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut render_settings: ResMut<RenderSettings>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        render_settings.cycle();
    }
}

pub fn regenerate_world(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    chunks: Query<Entity, With<Chunk>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyN) {
        for entity in &chunks {
            commands.entity(entity).despawn();
        }
        loaded_chunks.chunks.clear();
    }
}