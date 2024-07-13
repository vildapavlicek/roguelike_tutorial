use crate::components::Name;
use bevy::prelude::*;
use chrono::Local;

const TEXT_SIZE: f32 = 14.;
const DEFAULT_TEXT_COLOR: Color = Color::WHITE;

#[derive(Debug)]
pub struct LogUiPlugin;

impl Plugin for LogUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogMessage>()
            .add_systems(Startup, spawn_log_ui.run_if(run_once()))
            .add_systems(Update, update_log_texts);
    }
}

#[derive(Debug, Component, Copy, Clone)]
pub struct LogContainer;

#[derive(Debug, Component, Copy, Clone)]
pub struct LogText;

/// Entity IDs of messagess that should be displayed by the combat log
#[derive(Debug, Component, Copy, Clone)]
pub struct Messages([Option<Entity>; 5]);

impl Messages {
    /// We want to only print X messages, so when we push new one, to be displayed, the last one has to be removed
    /// Hence this returns the [Entity] of the message that should be despawned.
    pub fn add(&mut self, entity: Entity) -> Option<Entity> {
        let last = self.0.last().map(Clone::clone).flatten();

        self.0 = [Some(entity), self.0[0], self.0[1], self.0[2], self.0[3]];
        debug!(
            ?last,
            current_ents = ?self.0,
            "checking what messages to display"
        );
        last
    }

    pub fn to_vec(&self) -> Vec<Entity> {
        self.0
            .iter()
            .filter_map(Clone::clone)
            .collect::<Vec<Entity>>()
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

fn spawn_log_ui(mut cmd: bevy::prelude::Commands) {
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

    cmd.spawn((container, LogContainer)).add_child(init_message);

    cmd.spawn(Messages([Some(init_message), None, None, None, None]));
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
