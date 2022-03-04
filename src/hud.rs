use bevy::prelude::*;

use crate::Hp;


#[derive(Component)]
pub struct HpLabel;

pub fn spawn_hp_meter(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn_bundle(UiCameraBundle::default());

    // root node
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            style: Style {
                margin: Rect::all(Val::Px(5.0)),
                ..Default::default()
            },
            text: Text::with_section(
                "hello!",
                TextStyle {
                    font: asset_server.load("PublicPixel-0W6DP.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        }).insert(HpLabel);
    });

}

pub fn update_hp_meter(mut hp_label: Query<&mut Text, With<HpLabel>>, hp: Res<Hp>) {
    let section = &mut hp_label.single_mut().sections[0];
    section.value = format!("hp {}", hp.0);
    section.style.color = match hp.0 {
        0..=9 => Color::RED,
        _ => Color::WHITE
    };
}