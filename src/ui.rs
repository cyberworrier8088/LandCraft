use bevy::prelude::*;



use crate::inventory::Inventory;
use crate::mesh::BlockType;




// set up crosshair
pub fn setup_crosshair(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,

            width: Val::Percent(100.0),
            height: Val::Percent(100.0),

            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,

            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(asset_server.load("ui/crosshair.png")),
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),

                    ..default()
                },
            ));
        });



}


#[derive(Component)]
pub struct Hotbar;

#[derive(Component)]
pub struct HotbarSlot {
    pub index: usize,
}


#[derive(Component)]
pub struct HotbarIcon {
    pub index: usize,
}

pub fn setup_hotbar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Hotbar,
        Node {
            position_type: PositionType::Absolute,

            bottom: Val::Px(20.0),

            left: Val::Percent(50.0),

            width: Val::Px(450.0),

            height: Val::Px(50.0),

            margin: UiRect::left(Val::Px(-255.0)),

            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,

            ..default()
        },

        BackgroundColor(Color::srgba(0.18, 0.12, 0.08, 1.0)),
    )).with_children(|parent| {
        for i in 0..9 {
            parent.spawn((
                HotbarSlot {
                    index: i,
                },

                HotbarIcon {
                    index: i,
                },

                ImageNode::new(asset_server.load("ui/grass-block.png")),

                Node {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },

                BackgroundColor(Color::srgba(0.30, 0.22, 0.15, 1.0)),
                BorderColor::all(Color::BLACK),
            ));
        } 
    });
    
}


pub fn update_hotbar(
    inventory: Res<Inventory>,
    mut slots: Query<(&HotbarSlot, &mut BorderColor)>,
) {
    for (slot, mut border) in &mut slots {
        if slot.index == inventory.selected_slot {
            *border = BorderColor::all(Color::WHITE);
        } else {
            *border = BorderColor::all(Color::BLACK);
        }
    }
}



pub fn update_hotbar_icons(
    inventory: Res<Inventory>,
    asset_server: Res<AssetServer>,
    mut icons: Query<(&HotbarIcon, &mut ImageNode)>,
) {
    for (icon, mut image_node) in &mut icons {
        match inventory.slots[icon.index] {
            Some(BlockType::Grass) => {
                image_node.image = asset_server.load("ui/grass-block.png");
            }
            
            Some(BlockType::Cobblestone) => {
                image_node.image = asset_server.load("ui/grass-block.png");
            
            }
            
            Some(BlockType::Air) | None => {
                
                image_node.image = asset_server.load("ui/empty.png");
            
            }
        }
    }
}