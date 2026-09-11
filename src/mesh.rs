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
    Grass,
    Cobblestone,
    Dirt,
    Planks,
    Stone,
    Bedrock,
    Water,
    Lava,
    Sapling,
}

impl BlockType {
    pub fn is_solid(self) -> bool {
        !matches!(self, BlockType::Air | BlockType::Water | BlockType::Lava | BlockType::Sapling)
    }

    pub fn is_opaque(self) -> bool {
        !matches!(self, BlockType::Air | BlockType::Water | BlockType::Lava | BlockType::Sapling)
    }

    pub fn is_unbreakable(self) -> bool {
        matches!(self, BlockType::Bedrock)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeFace {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

pub fn get_block_face_uvs(block: BlockType, face: CubeFace) -> [[f32; 2]; 4] {
    let (col, row) = match block {
        BlockType::Air => (0, 0),
        BlockType::Grass => match face {
            CubeFace::Top => (0, 0),
            CubeFace::Bottom => (2, 0),
            _ => (3, 0),
        },
        BlockType::Cobblestone => (0, 1),
        BlockType::Dirt => (2, 0),
        BlockType::Planks => (4, 0),
        BlockType::Stone => (1, 0),
        BlockType::Bedrock => (1, 1),
        BlockType::Water => (13, 12),
        BlockType::Lava => (13, 14),
        BlockType::Sapling => (15, 0),
    };
    get_tile_uvs(col, row)
}

/// Creates a voxel block mesh with custom vertices, normals, and UV coordinates.
pub fn create_block_mesh(block_type: BlockType) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

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

    mesh.insert_indices(Indices::U32(vec![
        0, 1, 2,  0, 2, 3,       // Front
        4, 5, 6,  4, 6, 7,       // Back
        8, 9, 10, 8, 10, 11,     // Left
        12, 13, 14, 12, 14, 15,  // Right
        16, 17, 18, 16, 18, 19,  // Top
        20, 21, 22, 20, 22, 23,  // Bottom
    ]));

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0],
            [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0], [0.0, -1.0, 0.0],
        ],
    );

    let mut uv_vec = Vec::with_capacity(24);
    for face in [CubeFace::Front, CubeFace::Back, CubeFace::Left, CubeFace::Right, CubeFace::Top, CubeFace::Bottom] {
        uv_vec.extend_from_slice(&get_block_face_uvs(block_type, face));
    }
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

    let should_render_face = |current: BlockType, neighbor: BlockType| -> bool {
        if current == BlockType::Air {
            return false;
        }
        if neighbor == BlockType::Air {
            return true;
        }
        if !neighbor.is_opaque() && current != neighbor {
            return true;
        }
        false
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

                // Front (Z = +0.5)
                if should_render_face(block, get_block(lx as i32, ly as i32, lz as i32 + 1)) {
                    positions.push([-0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y,  0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y,  0.5 + z]);
                    for _ in 0..4 {
                        normals.push([0.0, 0.0, 1.0]);
                    }
                    uvs.extend_from_slice(&get_block_face_uvs(block, CubeFace::Front));
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Back (Z = -0.5)
                if should_render_face(block, get_block(lx as i32, ly as i32, lz as i32 - 1)) {
                    positions.push([ 0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([-0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y, -0.5 + z]);
                    for _ in 0..4 {
                        normals.push([0.0, 0.0, -1.0]);
                    }
                    uvs.extend_from_slice(&get_block_face_uvs(block, CubeFace::Back));
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Left (X = -0.5)
                if should_render_face(block, get_block(lx as i32 - 1, ly as i32, lz as i32)) {
                    positions.push([-0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([-0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y,  0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y, -0.5 + z]);
                    for _ in 0..4 {
                        normals.push([-1.0, 0.0, 0.0]);
                    }
                    uvs.extend_from_slice(&get_block_face_uvs(block, CubeFace::Left));
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Right (X = +0.5)
                if should_render_face(block, get_block(lx as i32 + 1, ly as i32, lz as i32)) {
                    positions.push([ 0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y,  0.5 + z]);
                    for _ in 0..4 {
                        normals.push([1.0, 0.0, 0.0]);
                    }
                    uvs.extend_from_slice(&get_block_face_uvs(block, CubeFace::Right));
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Top (Y = +0.5)
                if should_render_face(block, get_block(lx as i32, ly as i32 + 1, lz as i32)) {
                    positions.push([-0.5 + x,  0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y, -0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y, -0.5 + z]);
                    for _ in 0..4 {
                        normals.push([0.0, 1.0, 0.0]);
                    }
                    uvs.extend_from_slice(&get_block_face_uvs(block, CubeFace::Top));
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                    ]);
                    vertex_index += 4;
                }

                // Bottom (Y = -0.5)
                if should_render_face(block, get_block(lx as i32, ly as i32 - 1, lz as i32)) {
                    positions.push([-0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([-0.5 + x, -0.5 + y,  0.5 + z]);
                    for _ in 0..4 {
                        normals.push([0.0, -1.0, 0.0]);
                    }
                    uvs.extend_from_slice(&get_block_face_uvs(block, CubeFace::Bottom));
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



