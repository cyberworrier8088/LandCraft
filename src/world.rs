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
    let atlas_texture = asset_server.load("block/201101101828_terrain.png");

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
                    // Ore veins in stone layer
                    if y_world <= 8 && ((x_world * 67 + z_world * 47 + y_world * 17) % 59).abs() == 0 {
                        blocks[idx] = BlockType::DiamondOre;
                    } else if y_world <= 10 && ((x_world * 37 + z_world * 53 + y_world * 19) % 43).abs() == 0 {
                        blocks[idx] = BlockType::RedstoneOre;
                    } else if y_world <= 14 && ((x_world * 43 + z_world * 19 + y_world * 11) % 41).abs() == 0 {
                        blocks[idx] = BlockType::GoldOre;
                    } else if y_world < height - 4 && ((x_world * 29 + z_world * 31 + y_world * 7) % 27).abs() == 0 {
                        blocks[idx] = BlockType::IronOre;
                    } else if ((x_world * 17 + z_world * 23 + y_world * 13) % 19).abs() == 0 {
                        blocks[idx] = BlockType::CoalOre;
                    } else {
                        blocks[idx] = BlockType::Stone;
                    }
                } else if y_world <= height {
                    if height <= SEA_LEVEL + 1 {
                        if y_world == height {
                            blocks[idx] = BlockType::Sand;
                        } else {
                            blocks[idx] = BlockType::Gravel;
                        }
                    } else if y_world == height {
                        blocks[idx] = BlockType::Grass;
                    } else {
                        blocks[idx] = BlockType::Dirt;
                    }
                } else if y_world <= SEA_LEVEL {
                    blocks[idx] = BlockType::Water;
                } else if y_world == height + 1 && height > SEA_LEVEL + 1 {
                    let flower_hash = (x_world * 79 + z_world * 37).abs();
                    if flower_hash % 47 == 0 {
                        blocks[idx] = BlockType::Rose;
                    } else if flower_hash % 43 == 1 {
                        blocks[idx] = BlockType::Dandelion;
                    } else if flower_hash % 67 == 2 {
                        blocks[idx] = BlockType::Sapling;
                    } else {
                        blocks[idx] = BlockType::Air;
                    }
                } else {
                    blocks[idx] = BlockType::Air;
                }

                // Procedural oak tree generation (trunk + canopy)
                for cx in -1..=1 {
                    for cz in -1..=1 {
                        let cell_x = x_world.div_euclid(14) + cx;
                        let cell_z = z_world.div_euclid(14) + cz;
                        let tx = cell_x * 14 + 7;
                        let tz = cell_z * 14 + 7;

                        if ((tx * 31 + tz * 17) % 2).abs() == 0 {
                            let th = terrain_height(tx, tz);
                            if th > SEA_LEVEL + 1 {
                                if x_world == tx && z_world == tz && y_world > th && y_world <= th + 5 {
                                    blocks[idx] = BlockType::Wood;
                                } else if y_world >= th + 3 && y_world <= th + 4 {
                                    let dx = (x_world - tx).abs();
                                    let dz = (z_world - tz).abs();
                                    if dx <= 2 && dz <= 2 && !(dx == 2 && dz == 2) && blocks[idx] == BlockType::Air {
                                        blocks[idx] = BlockType::Leaves;
                                    }
                                } else if y_world >= th + 5 && y_world <= th + 6 {
                                    let dx = (x_world - tx).abs();
                                    let dz = (z_world - tz).abs();
                                    if dx <= 1 && dz <= 1 && blocks[idx] == BlockType::Air {
                                        blocks[idx] = BlockType::Leaves;
                                    }
                                }
                            }
                        }
                    }
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
                        if below_block == BlockType::Air || below_block.is_cross_plant() {
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
                                if neighbor == BlockType::Air || neighbor.is_cross_plant() {
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