use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use crate::mesh::BlockType;

pub const ALL_BLOCKS: [BlockType; 38] = [
    BlockType::Grass,
    BlockType::Dirt,
    BlockType::Stone,
    BlockType::Cobblestone,
    BlockType::Planks,
    BlockType::Bedrock,
    BlockType::Sand,
    BlockType::Gravel,
    BlockType::Wood,
    BlockType::Leaves,
    BlockType::Glass,
    BlockType::Bricks,
    BlockType::CraftingTable,
    BlockType::Furnace,
    BlockType::Bookshelf,
    BlockType::Tnt,
    BlockType::Obsidian,
    BlockType::MossyCobblestone,
    BlockType::CoalOre,
    BlockType::IronOre,
    BlockType::GoldOre,
    BlockType::DiamondOre,
    BlockType::RedstoneOre,
    BlockType::IronBlock,
    BlockType::GoldBlock,
    BlockType::DiamondBlock,
    BlockType::WhiteWool,
    BlockType::Snow,
    BlockType::Ice,
    BlockType::Clay,
    BlockType::Netherrack,
    BlockType::SoulSand,
    BlockType::Glowstone,
    BlockType::Sapling,
    BlockType::Rose,
    BlockType::Dandelion,
    BlockType::Water,
    BlockType::Lava,
];

#[derive(Resource)]
pub struct Inventory {
    pub slots: [Option<BlockType>; 9],
    pub selected_slot: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: [
                Some(BlockType::Grass),
                Some(BlockType::Cobblestone),
                Some(BlockType::Wood),
                Some(BlockType::Planks),
                Some(BlockType::Glass),
                Some(BlockType::Bricks),
                Some(BlockType::CraftingTable),
                Some(BlockType::Furnace),
                Some(BlockType::Tnt),
            ],
            selected_slot: 0,
        }
    }
}

pub fn change_selected_slot(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut inventory: ResMut<Inventory>,
) {
    let new_slot = if keyboard.just_pressed(KeyCode::Digit1) { Some(0) }
    else if keyboard.just_pressed(KeyCode::Digit2) { Some(1) }
    else if keyboard.just_pressed(KeyCode::Digit3) { Some(2) }
    else if keyboard.just_pressed(KeyCode::Digit4) { Some(3) }
    else if keyboard.just_pressed(KeyCode::Digit5) { Some(4) }
    else if keyboard.just_pressed(KeyCode::Digit6) { Some(5) }
    else if keyboard.just_pressed(KeyCode::Digit7) { Some(6) }
    else if keyboard.just_pressed(KeyCode::Digit8) { Some(7) }
    else if keyboard.just_pressed(KeyCode::Digit9) { Some(8) }
    else { None };

    if let Some(slot) = new_slot {
        inventory.selected_slot = slot;
    }

    for event in mouse_wheel_events.read() {
        if event.y < 0.0 {
            inventory.selected_slot = (inventory.selected_slot + 1) % 9;
        } else if event.y > 0.0 {
            inventory.selected_slot = (inventory.selected_slot + 8) % 9;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        let slot = inventory.selected_slot;
        let current = inventory.slots[slot].unwrap_or(BlockType::Grass);
        let next_idx = match ALL_BLOCKS.iter().position(|&b| b == current) {
            Some(idx) => (idx + 1) % ALL_BLOCKS.len(),
            None => 0,
        };
        inventory.slots[slot] = Some(ALL_BLOCKS[next_idx]);
    }
}