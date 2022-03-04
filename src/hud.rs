use bevy::{core::Stopwatch, ecs::system::OptionResState, prelude::*};

use crate::{
    advantage::{Advantage, EnemyAdvantage, PlayerAdvantage},
    Hp,
};

#[derive(Component)]
pub struct HpLabel;

#[derive(Component)]
pub struct AdvantageLabel;

#[derive(Component)]
pub struct HintLabel;

const HINT: &str = r#"
WASD to move and jump.
Each level has a random Unfair Advantage
for you or your enemies.
Take use of your advantage to gain resources.
Try to survive when your enemies have the advantage.
Find and reach the end of the level to win.
Good luck!
"#;

#[derive(Debug)]
pub struct HintStopwatch(Timer);

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            spawn_hint(parent, &asset_server);

            // space
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            });

            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "advantage",
                        TextStyle {
                            font: asset_server.load("PublicPixel-0W6DP.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(AdvantageLabel);

            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "health",
                        TextStyle {
                            font: asset_server.load("PublicPixel-0W6DP.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(HpLabel);
        });
}

pub fn update_hp_meter(mut hp_label: Query<&mut Text, With<HpLabel>>, hp: Res<Hp>) {
    let section = &mut hp_label.single_mut().sections[0];
    section.value = format!("hp {}", hp.0);
    section.style.color = match hp.0 {
        0..=9 => Color::RED,
        _ => Color::WHITE,
    };
}

pub fn update_advantage(mut label: Query<&mut Text, With<AdvantageLabel>>, adv: Res<Advantage>) {
    let section = &mut label.single_mut().sections[0];
    let (color, text) = match adv.into_inner() {
        Advantage::Player(pa) => (
            Color::AQUAMARINE,
            match pa {
                PlayerAdvantage::DoubleJump => "Double Jump",
                PlayerAdvantage::DoubleInitialHp => "Double HP",
            },
        ),
        Advantage::Enemy(ea) => (
            Color::ORANGE,
            match ea {
                EnemyAdvantage::DoubleBite => "Painful bites",
                EnemyAdvantage::DoubleSpeed => "Fast enemies",
            },
        ),
    };
    section.style.color = color;
    section.value = text.to_string();
}

pub fn spawn_hint(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn_bundle(TextBundle {
            style: Style {
                margin: Rect::all(Val::Px(5.0)),
                ..Default::default()
            },
            text: Text::with_section(
                HINT,
                TextStyle {
                    font: asset_server.load("PublicPixel-0W6DP.ttf"),
                    font_size: 18.0,
                    color: Color::GOLD,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(HintLabel);
}

pub fn fade_out_hint(
    mut commands: Commands,
    time: Res<Time>,
    mut hint: Query<&mut Text, With<HintLabel>>,
    stopwatch: Option<ResMut<HintStopwatch>>,
) {
    match stopwatch {
        None => commands.insert_resource(HintStopwatch(Timer::from_seconds(10.0, false))),
        Some(sw) if sw.0.finished() => {
            let style = &mut hint.single_mut().sections[0].style;
            style.color.set_a(style.color.a() * 0.96);
        }
        Some(mut sw) => {
            sw.0.tick(time.delta());
        }
    }
}
