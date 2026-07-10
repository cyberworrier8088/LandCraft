use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub const CHUNK_SIZE: usize = 16;

/// Represents the type of block to generate a mesh for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlockType {
    #[default]
    Air,
    Cobblestone,
    Grass,
}

/// Creates a voxel block mesh with custom vertices, normals, and UV coordinates
/// mapped to a 2-tile horizontal texture atlas (Left = Cobblestone, Right = Grass).
pub fn create_block_mesh(block_type: BlockType) -> Mesh {
    // Initialize the mesh with TriangleList topology and default render asset usages
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    // 24 vertices for the 6 faces of the cube (4 vertices per face)
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Front (Z = +0.5)
            [-0.5, -0.5,  0.5],
            [ 0.5, -0.5,  0.5],
            [ 0.5,  0.5,  0.5],
            [-0.5,  0.5,  0.5],

            // Back (Z = -0.5)
            [ 0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [-0.5,  0.5, -0.5],
            [ 0.5,  0.5, -0.5],

            // Left (X = -0.5)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5,  0.5],
            [-0.5,  0.5,  0.5],
            [-0.5,  0.5, -0.5],

            // Right (X = +0.5)
            [ 0.5, -0.5,  0.5],
            [ 0.5, -0.5, -0.5],
            [ 0.5,  0.5, -0.5],
            [ 0.5,  0.5,  0.5],

            // Top (Y = +0.5)
            [-0.5,  0.5,  0.5],
            [ 0.5,  0.5,  0.5],
            [ 0.5,  0.5, -0.5],
            [-0.5,  0.5, -0.5],

            // Bottom (Y = -0.5)
            [-0.5, -0.5, -0.5],
            [ 0.5, -0.5, -0.5],
            [ 0.5, -0.5,  0.5],
            [-0.5, -0.5,  0.5],
        ],
    );

    // 36 indices defining the 12 triangles of the cube
    mesh.insert_indices(Indices::U32(vec![
        // Front
        0, 1, 2,
        0, 2, 3,
        
        // Back
        4, 5, 6,
        4, 6, 7,

        // Left
        8, 9, 10,
        8, 10, 11,

        // Right
        12, 13, 14,
        12, 14, 15,

        // Top
        16, 17, 18,
        16, 18, 19,

        // Bottom
        20, 21, 22,
        20, 22, 23,
    ]));

    // 24 face-normal vectors pointing outward from the cube's center
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Front
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],

            // Back
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],

            // Left
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],

            // Right
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],

            // Top
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],

            // Bottom
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
    );

    // 24 texture coordinates mapped to the 16x16 grid terrain atlas.
    let uv_vec = match block_type {
        BlockType::Air => vec![[0.0, 0.0]; 24],
        BlockType::Cobblestone => {
            // Cobblestone/Stone block uses tile at Col 1, Row 0 for all 6 faces
            let cobble_uvs = get_tile_uvs(1, 0);
            let mut uvs = Vec::with_capacity(24);
            for _ in 0..6 {
                uvs.extend_from_slice(&cobble_uvs);
            }
            uvs
        }
        BlockType::Grass => {
            // Grass block uses Grass tile at Col 0, Row 0 for all 6 faces
            let grass_uvs = get_tile_uvs(0, 0);
            let mut uvs = Vec::with_capacity(24);
            for _ in 0..6 {
                uvs.extend_from_slice(&grass_uvs);
            }
            uvs
        }
    };

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_vec);

    mesh
}

/// Helper function to calculate the UV coordinates for a single cube face
/// mapped to a specific tile coordinates (col, row) in a 16x16 grid.
fn get_tile_uvs(col: u32, row: u32) -> [[f32; 2]; 4] {
    let u_min = col as f32 * 0.0625;
    let u_max = (col + 1) as f32 * 0.0625;
    let v_min = row as f32 * 0.0625;
    let v_max = (row + 1) as f32 * 0.0625;
    [
        [u_min, v_max], // Bottom-Left
        [u_max, v_max], // Bottom-Right
        [u_max, v_min], // Top-Right
        [u_min, v_min], // Top-Left
    ]
}

/// Generates a single combined mesh for an entire chunk using Face Culling.
/// Faces are only added if the adjacent block in that direction is Air.
pub fn create_chunk_mesh(blocks: &[BlockType; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    let mut vertex_index = 0;

    let index_fn = |x: usize, y: usize, z: usize| -> usize {
        x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE
    };

    let get_block = |x: i32, y: i32, z: i32| -> BlockType {
        if x < 0 || x >= CHUNK_SIZE as i32 || y < 0 || y >= CHUNK_SIZE as i32 || z < 0 || z >= CHUNK_SIZE as i32 {
            BlockType::Air
        } else {
            blocks[index_fn(x as usize, y as usize, z as usize)]
        }
    };

    for lz in 0..CHUNK_SIZE {
        for ly in 0..CHUNK_SIZE {
            for lx in 0..CHUNK_SIZE {
                let block = get_block(lx as i32, ly as i32, lz as i32);
                if block == BlockType::Air {
                    continue;
                }

                let x = lx as f32;
                let y = ly as f32;
                let z = lz as f32;

                // Determine tile UVs based on block type
                let tile_uvs = match block {
                    BlockType::Cobblestone => get_tile_uvs(1, 0),
                    BlockType::Grass => get_tile_uvs(0, 0),
                    BlockType::Air => unreachable!(),
                };

                // Front (Z = +0.5) - check neighbor (lx, ly, lz + 1)
                if get_block(lx as i32, ly as i32, lz as i32 + 1) == BlockType::Air {
                    positions.push([-0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y,  0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y,  0.5 + z]);
                    for _ in 0..4 {
                        normals.push([0.0, 0.0, 1.0]);
                    }
                    uvs.extend_from_slice(&tile_uvs);
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Back (Z = -0.5) - check neighbor (lx, ly, lz - 1)
                if get_block(lx as i32, ly as i32, lz as i32 - 1) == BlockType::Air {
                    positions.push([ 0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([-0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y, -0.5 + z]);
                    for _ in 0..4 {
                        normals.push([0.0, 0.0, -1.0]);
                    }
                    uvs.extend_from_slice(&tile_uvs);
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Left (X = -0.5) - check neighbor (lx - 1, ly, lz)
                if get_block(lx as i32 - 1, ly as i32, lz as i32) == BlockType::Air {
                    positions.push([-0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([-0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y,  0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y, -0.5 + z]);
                    for _ in 0..4 {
                        normals.push([-1.0, 0.0, 0.0]);
                    }
                    uvs.extend_from_slice(&tile_uvs);
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Right (X = +0.5) - check neighbor (lx + 1, ly, lz)
                if get_block(lx as i32 + 1, ly as i32, lz as i32) == BlockType::Air {
                    positions.push([ 0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y,  0.5 + z]);
                    for _ in 0..4 {
                        normals.push([1.0, 0.0, 0.0]);
                    }
                    uvs.extend_from_slice(&tile_uvs);
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Top (Y = +0.5) - check neighbor (lx, ly + 1, lz)
                if get_block(lx as i32, ly as i32 + 1, lz as i32) == BlockType::Air {
                    positions.push([-0.5 + x,  0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y, -0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y, -0.5 + z]);
                    for _ in 0..4 {
                        normals.push([0.0, 1.0, 0.0]);
                    }
                    uvs.extend_from_slice(&tile_uvs);
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Bottom (Y = -0.5) - check neighbor (lx, ly - 1, lz)
                if get_block(lx as i32, ly as i32 - 1, lz as i32) == BlockType::Air {
                    positions.push([-0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([-0.5 + x, -0.5 + y,  0.5 + z]);
                    for _ in 0..4 {
                        normals.push([0.0, -1.0, 0.0]);
                    }
                    uvs.extend_from_slice(&tile_uvs);
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }
            }
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}



