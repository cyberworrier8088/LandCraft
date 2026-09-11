use bevy::prelude::*;
use crate::mesh::BlockType;


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
                Some(BlockType::Stone),
                Some(BlockType::Dirt),
                Some(BlockType::Planks),
                Some(BlockType::Sapling),
                Some(BlockType::Water),
                Some(BlockType::Lava),
                None,
            ],
            selected_slot: 0,
        }
    }
}


pub fn change_selected_slot(
    keyboard: Res<ButtonInput<KeyCode>>,
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
}