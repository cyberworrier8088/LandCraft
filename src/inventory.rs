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
                None,
                None,
                None,
                None,
                None,
                None,
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
    if keyboard.just_pressed(KeyCode::Digit1) {
        inventory.selected_slot = 0;
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        inventory.selected_slot = 1;
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        inventory.selected_slot = 2;
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        inventory.selected_slot = 3;
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        inventory.selected_slot = 4;
    }
    if keyboard.just_pressed(KeyCode::Digit6) {
        inventory.selected_slot = 5;
    }
    if keyboard.just_pressed(KeyCode::Digit7) {
        inventory.selected_slot = 6;
    }
    if keyboard.just_pressed(KeyCode::Digit8) {
        inventory.selected_slot = 7;
    }
    if keyboard.just_pressed(KeyCode::Digit9) {
        inventory.selected_slot = 8;
    }
    println!("Selected slot: {}", inventory.selected_slot);
}