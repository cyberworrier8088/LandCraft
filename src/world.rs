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

#[derive(Resource)]
pub struct FluidTimer {
    pub timer: Timer,
    pub tick_count: u32,
}

impl Default for FluidTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.35, TimerMode::Repeating),
            tick_count: 0,
        }
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
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    });
    
    // We register BlockAssets for compatibility (e.g. highlight or other queries)
    commands.insert_resource(BlockAssets {
        material: block_material.clone(),
    });
    commands.insert_resource(RenderSettings::default());
    commands.insert_resource(FluidTimer::default());
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
                } else if y_world <= 3 && ((x_world * 31 + z_world * 17 + y_world) % 29).abs() == 0 {
                    blocks[idx] = BlockType::Lava;
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
                } else if y_world == height + 1 && y_world > SEA_LEVEL && ((x_world * 73 + z_world * 37) % 47).abs() == 0 {
                    blocks[idx] = BlockType::Sapling;
                } else if y_world <= SEA_LEVEL {
                    blocks[idx] = BlockType::Water;
                } else {
                    blocks[idx] = BlockType::Air;
                }
            }
        }
    }

    let chunk_mesh = create_chunk_mesh(&blocks);
    let has_verts = chunk_mesh.count_vertices() > 0;

    let mut entity_cmds = commands.spawn((
        Chunk {
            blocks,
        },
        Transform::from_translation(chunk_pos.as_vec3() * 16.0),
    ));

    if has_verts {
        let mesh_handle = meshes.add(chunk_mesh);
        entity_cmds.insert((
            Mesh3d(mesh_handle),
            MeshMaterial3d(block_material.clone()),
        ));
    }
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

pub fn simulate_fluids(
    time: Res<Time>,
    mut fluid_timer: ResMut<FluidTimer>,
    mut chunks: Query<(Entity, &mut Chunk, Option<&Mesh3d>), With<Chunk>>,
    mut meshes: ResMut<Assets<Mesh>>,
    block_assets: Res<BlockAssets>,
    mut commands: Commands,
) {
    if !fluid_timer.timer.tick(time.delta()).just_finished() {
        return;
    }
    fluid_timer.tick_count = fluid_timer.tick_count.wrapping_add(1);
    let tick = fluid_timer.tick_count;

    for (entity, mut chunk, mesh3d_opt) in chunks.iter_mut() {
        let mut updates: Vec<(usize, BlockType)> = Vec::new();
        let blocks_snapshot = chunk.blocks;

        for ly in 0..16 {
            for lz in 0..16 {
                for lx in 0..16 {
                    let idx = lx + ly * 16 + lz * 256;
                    let block = blocks_snapshot[idx];

                    if block != BlockType::Water && block != BlockType::Lava {
                        continue;
                    }

                    // Lava spreads slower (every 2nd tick)
                    if block == BlockType::Lava && !tick.is_multiple_of(2) {
                        continue;
                    }

                    // 1. Flow downward
                    if ly > 0 {
                        let below_idx = lx + (ly - 1) * 16 + lz * 256;
                        let below_block = blocks_snapshot[below_idx];
                        if below_block == BlockType::Air || below_block == BlockType::Sapling {
                            updates.push((below_idx, block));
                            continue;
                        } else if block == BlockType::Water && below_block == BlockType::Lava {
                            updates.push((below_idx, BlockType::Stone));
                            continue;
                        } else if block == BlockType::Lava && below_block == BlockType::Water {
                            updates.push((below_idx, BlockType::Cobblestone));
                            continue;
                        }
                    }

                    // 2. Horizontal flow
                    let down_blocked = ly == 0 || blocks_snapshot[lx + (ly - 1) * 16 + lz * 256].is_solid();
                    if down_blocked {
                        for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let nx = lx as i32 + dx;
                            let nz = lz as i32 + dz;
                            if (0..16).contains(&nx) && (0..16).contains(&nz) {
                                let n_idx = nx as usize + ly * 16 + (nz as usize) * 256;
                                let neighbor = blocks_snapshot[n_idx];
                                if neighbor == BlockType::Air || neighbor == BlockType::Sapling {
                                    updates.push((n_idx, block));
                                } else if block == BlockType::Water && neighbor == BlockType::Lava {
                                    updates.push((n_idx, BlockType::Cobblestone));
                                } else if block == BlockType::Lava && neighbor == BlockType::Water {
                                    updates.push((n_idx, BlockType::Stone));
                                }
                            }
                        }
                    }
                }
            }
        }

        if !updates.is_empty() {
            for (idx, new_type) in updates {
                chunk.blocks[idx] = new_type;
            }
            let new_mesh = create_chunk_mesh(&chunk.blocks);
            let has_verts = new_mesh.count_vertices() > 0;
            if let Some(mesh3d) = mesh3d_opt {
                if has_verts {
                    if let Some(mut mesh) = meshes.get_mut(&mesh3d.0) {
                        *mesh = new_mesh;
                    }
                } else {
                    commands.entity(entity).remove::<(Mesh3d, MeshMaterial3d<StandardMaterial>)>();
                    meshes.remove(&mesh3d.0);
                }
            } else if has_verts {
                let handle = meshes.add(new_mesh);
                commands.entity(entity).insert((
                    Mesh3d(handle),
                    MeshMaterial3d(block_assets.material.clone()),
                ));
            }
        }
    }
}