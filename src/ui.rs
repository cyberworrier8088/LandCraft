use bevy::prelude::*;



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


pub fn setup_hotbar(
    mut commands: Commands,
) {
    commands.spawn((
        Hotbar,
        Node {
            position_type: PositionType::Absolute,

            bottom: Val::Px(20.0),

            left: Val::Percent(50.0),

            width: Val::Px(450.0),
            height: Val::Px(50.0),

            ..default()
        },

        BackgroundColor(Color::srgb(0.18, 0.12, 0.08)),
    ));
}
