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
    Dirt,
    Stone,
    Cobblestone,
    Planks,
    Bedrock,
    Sand,
    Gravel,
    Wood,
    Leaves,
    Glass,
    Water,
    Lava,
    Sapling,
    Rose,
    Dandelion,
    CoalOre,
    IronOre,
    GoldOre,
    DiamondOre,
    RedstoneOre,
    Bricks,
    Tnt,
    Bookshelf,
    MossyCobblestone,
    Obsidian,
    CraftingTable,
    Furnace,
    WhiteWool,
    Snow,
    Ice,
    Clay,
    Netherrack,
    SoulSand,
    Glowstone,
    IronBlock,
    GoldBlock,
    DiamondBlock,
}

impl BlockType {
    pub fn is_solid(self) -> bool {
        !matches!(
            self,
            BlockType::Air
                | BlockType::Water
                | BlockType::Lava
                | BlockType::Sapling
                | BlockType::Rose
                | BlockType::Dandelion
        )
    }

    pub fn is_opaque(self) -> bool {
        !matches!(
            self,
            BlockType::Air
                | BlockType::Water
                | BlockType::Lava
                | BlockType::Sapling
                | BlockType::Rose
                | BlockType::Dandelion
                | BlockType::Leaves
                | BlockType::Glass
                | BlockType::Ice
        )
    }

    pub fn is_unbreakable(self) -> bool {
        matches!(self, BlockType::Bedrock)
    }

    pub fn is_cross_plant(self) -> bool {
        matches!(
            self,
            BlockType::Sapling | BlockType::Rose | BlockType::Dandelion
        )
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
        BlockType::Dirt => (2, 0),
        BlockType::Stone => (1, 0),
        BlockType::Cobblestone => (0, 1),
        BlockType::Planks => (4, 0),
        BlockType::Bedrock => (1, 1),
        BlockType::Sand => (2, 1),
        BlockType::Gravel => (3, 1),
        BlockType::Wood => match face {
            CubeFace::Top | CubeFace::Bottom => (5, 1),
            _ => (4, 1),
        },
        BlockType::Leaves => (4, 3),
        BlockType::Glass => (1, 3),
        BlockType::Water => (14, 0),
        BlockType::Lava => (13, 14),
        BlockType::Sapling => (15, 0),
        BlockType::Rose => (12, 0),
        BlockType::Dandelion => (13, 0),
        BlockType::CoalOre => (2, 2),
        BlockType::IronOre => (1, 2),
        BlockType::GoldOre => (0, 2),
        BlockType::DiamondOre => (2, 3),
        BlockType::RedstoneOre => (3, 3),
        BlockType::Bricks => (7, 0),
        BlockType::Tnt => match face {
            CubeFace::Top => (9, 0),
            CubeFace::Bottom => (10, 0),
            _ => (8, 0),
        },
        BlockType::Bookshelf => match face {
            CubeFace::Top | CubeFace::Bottom => (4, 0),
            _ => (3, 2),
        },
        BlockType::MossyCobblestone => (4, 2),
        BlockType::Obsidian => (5, 2),
        BlockType::CraftingTable => match face {
            CubeFace::Top => (11, 2),
            CubeFace::Bottom => (4, 0),
            CubeFace::Front => (11, 3),
            _ => (12, 3),
        },
        BlockType::Furnace => match face {
            CubeFace::Front => (12, 2),
            CubeFace::Top => (14, 3),
            CubeFace::Bottom => (1, 0),
            _ => (13, 2),
        },
        BlockType::WhiteWool => (0, 4),
        BlockType::Snow => (2, 4),
        BlockType::Ice => (3, 4),
        BlockType::Clay => (8, 4),
        BlockType::Netherrack => (7, 6),
        BlockType::SoulSand => (8, 6),
        BlockType::Glowstone => (9, 6),
        BlockType::IronBlock => (6, 1),
        BlockType::GoldBlock => (7, 1),
        BlockType::DiamondBlock => (8, 1),
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

                if block.is_cross_plant() {
                    let [c0, c1, c2, c3] = get_block_face_uvs(block, CubeFace::Front);
                    let norm = std::f32::consts::FRAC_1_SQRT_2;
                    // Diagonal 1: (-0.5, -0.5, -0.5) to (0.5, 0.5, 0.5)
                    positions.push([-0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y,  0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y, -0.5 + z]);
                    for _ in 0..4 {
                        normals.push([-norm, 0.0, norm]);
                    }
                    uvs.extend_from_slice(&[c0, c1, c2, c3]);
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                        vertex_index, vertex_index + 2, vertex_index + 1,
                        vertex_index, vertex_index + 3, vertex_index + 2,
                    ]);
                    vertex_index += 4;

                    // Diagonal 2: (-0.5, -0.5, 0.5) to (0.5, 0.5, -0.5)
                    positions.push([-0.5 + x, -0.5 + y,  0.5 + z]);
                    positions.push([ 0.5 + x, -0.5 + y, -0.5 + z]);
                    positions.push([ 0.5 + x,  0.5 + y, -0.5 + z]);
                    positions.push([-0.5 + x,  0.5 + y,  0.5 + z]);
                    for _ in 0..4 {
                        normals.push([norm, 0.0, norm]);
                    }
                    uvs.extend_from_slice(&[c0, c1, c2, c3]);
                    indices.extend_from_slice(&[
                        vertex_index, vertex_index + 1, vertex_index + 2,
                        vertex_index, vertex_index + 2, vertex_index + 3,
                        vertex_index, vertex_index + 2, vertex_index + 1,
                        vertex_index, vertex_index + 3, vertex_index + 2,
                    ]);
                    vertex_index += 4;
                    continue;
                }

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



