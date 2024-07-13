use crate::components::{combat::Health, ui::*, Name, Player};
use bevy::prelude::*;
use chrono::Local;

const TEXT_SIZE: f32 = 14.;
const DEFAULT_TEXT_COLOR: Color = Color::WHITE;

#[derive(Debug)]
pub struct LogUiPlugin;

impl Plugin for LogUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogMessage>()
            .add_systems(
                Startup,
                spawn_log_ui
                    .run_if(run_once())
                    .after(crate::systems::PlayerInitSet),
            )
            .add_systems(Update, (update_log_texts, update_hp_bar));
    }
}

/// This is a list of messages that can be printed into our log
#[derive(Debug, Clone, Event)]
pub enum LogMessage {
    /// When attack happens and we want to log it
    AttackMessage {
        /// Time when the attack occured
        time: chrono::DateTime<Local>,
        /// Who does the attacking
        attacker: Name,
        /// Who is being attacked
        defender: Name,
        /// How much damage defender suffers
        damage: i32,
    },
    Death {
        time: chrono::DateTime<Local>,
        name: Name,
    },
}

impl From<&LogMessage> for TextBundle {
    fn from(value: &LogMessage) -> TextBundle {
        match value {
            LogMessage::AttackMessage {
                time,
                attacker,
                defender,
                damage,
            } => TextBundle::from_sections([
                TextSection {
                    value: format!("{}: ", time.format("%H:%M:%S%.3f")),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: DEFAULT_TEXT_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("{attacker}"),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: Color::YELLOW,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(" attacked "),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: DEFAULT_TEXT_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("{defender}"),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: Color::YELLOW,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(" for"),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: DEFAULT_TEXT_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(" {damage}"),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: Color::CRIMSON,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(" damage."),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: DEFAULT_TEXT_COLOR,
                        ..default()
                    },
                },
            ]),
            LogMessage::Death { time, name } => TextBundle::from_sections([
                TextSection {
                    value: format!("{}: ", time.format("%H:%M:%S%.3f")),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: DEFAULT_TEXT_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("{name}"),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: Color::YELLOW,
                        ..default()
                    },
                },
                TextSection {
                    value: format!(" has "),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: DEFAULT_TEXT_COLOR,
                        ..default()
                    },
                },
                TextSection {
                    value: format!("died."),
                    style: TextStyle {
                        font_size: TEXT_SIZE,
                        color: Color::RED,
                        ..default()
                    },
                },
            ]),
        }
    }
}

fn spawn_log_ui(mut cmd: bevy::prelude::Commands, player_hp: Query<&Health, With<Player>>) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(60f32),
            height: Val::Percent(10f32),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            position_type: PositionType::Absolute,
            bottom: Val::Percent(0.0),
            right: Val::Percent(20.0),
            padding: UiRect {
                left: Val::Px(10f32),
                ..default()
            },
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0.16, 0.16, 0.16, 0.75)),
        ..default()
    };

    let init_message = cmd
        .spawn((
            TextBundle {
                text: Text::from_section(
                    "Welcome!",
                    TextStyle {
                        color: Color::CYAN,
                        font_size: TEXT_SIZE,
                        ..default()
                    },
                ),
                ..Default::default()
            },
            Label,
            LogText,
        ))
        .id();

    cmd.spawn((container, LogContainer))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(100f32),
                            height: Val::Px(25f32),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Stretch,
                            position_type: PositionType::Absolute,
                            bottom: Val::Percent(9.5f32),
                            left: Val::Percent(33f32),
                            justify_content: JustifyContent::Center,
                            align_content: AlignContent::Stretch,
                            padding: UiRect::all(Val::Px(10f32)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.50, 0f32, 0f32, 0.50)),
                        ..default()
                    },
                    HpNode,
                ))
                .with_children(|parent| {
                    let player_hp = player_hp.single();
                    parent.spawn((
                        TextBundle::from_section(
                            format!("HP: {}/{}", player_hp.current, player_hp.max),
                            TextStyle {
                                color: Color::WHITE,
                                font_size: TEXT_SIZE,
                                ..default()
                            },
                        ),
                        HpText,
                    ));
                });
        })
        .push_children(&[init_message]);
    cmd.spawn(Messages::new([Some(init_message), None, None, None, None]));
}

fn update_log_texts(
    mut cmd: Commands,
    mut events: EventReader<LogMessage>,
    mut messages: Query<&mut Messages>,
    container: Query<Entity, With<LogContainer>>,
) {
    let mut messages = messages.single_mut();
    let container = container.single();
    cmd.entity(container).clear_children();

    for event in events.read() {
        let entity = cmd.spawn((TextBundle::from(event), Label, LogText)).id();
        if let Some(to_remove) = messages.add(entity) {
            debug!(?to_remove, "removing entity");
            cmd.entity(to_remove).despawn();
        }
    }

    cmd.entity(container).push_children(&messages.to_vec());
}

fn update_hp_bar(
    mut hp_text: Query<&mut Text, With<HpText>>,
    player_hp: Query<&Health, With<Player>>,
) {
    let player_health = player_hp.single();
    let mut health_text = hp_text.single_mut();

    health_text.sections = vec![
        TextSection::new(
            "HP: ",
            TextStyle {
                font_size: TEXT_SIZE,
                color: Color::WHITE,
                ..default()
            },
        ),
        TextSection::new(
            format!("{}", player_health.current),
            TextStyle {
                font_size: TEXT_SIZE,
                color: match player_health.current {
                    health if health <= get_percentage(player_health.max, 0.25) => Color::RED,
                    health if health <= get_percentage(player_health.max, 0.33) => Color::ORANGE,
                    health if health <= get_percentage(player_health.max, 0.5) => Color::YELLOW,
                    health if health <= get_percentage(player_health.max, 0.75) => {
                        Color::YELLOW_GREEN
                    }
                    _ => Color::GREEN,
                },
                ..default()
            },
        ),
        TextSection::new(
            "/",
            TextStyle {
                font_size: TEXT_SIZE,
                color: Color::WHITE,
                ..default()
            },
        ),
        TextSection::new(
            format!("{}", player_health.max),
            TextStyle {
                font_size: TEXT_SIZE,
                color: Color::WHITE,
                ..default()
            },
        ),
    ];
}

#[inline]
fn get_percentage(value: i32, percent: f32) -> i32 {
    (value as f32 * percent) as i32
}
