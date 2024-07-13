pub mod log;

use bevy::prelude::*;
use bevy::text::TextStyle;

#[derive(Debug, Copy, Clone, Component)]
pub struct CombatLog;

#[derive(Debug, Copy, Clone, Component)]
pub struct CombatLogWindow;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(log::LogUiPlugin);
    }
}

// fn spawn_ui_text(mut cmd: bevy::prelude::Commands) {
//     trace!("spawning text");

//     let container = NodeBundle {
//         style: Style {
//             width: Val::Percent(60f32),
//             height: Val::Percent(10f32),
//             flex_direction: FlexDirection::Column,
//             align_items: AlignItems::Stretch,
//             position_type: PositionType::Absolute,
//             bottom: Val::Percent(0.0),
//             right: Val::Percent(20.0),
//             padding: UiRect {
//                 left: Val::Px(10f32),
//                 ..default()
//             },
//             ..default()
//         },
//         background_color: BackgroundColor(Color::rgba(0.16, 0.16, 0.16, 0.75)),
//         ..default()
//     };

//     cmd.spawn((container, CombatLogWindow))
//         .with_children(|parent| {
//             parent.spawn((
//                 TextBundle {
//                     text: Text::from_section(
//                         "This is my first text",
//                         TextStyle {
//                             color: Color::RED,
//                             font_size: COMBAT_LOG_TEXT_SIZE,
//                             ..default()
//                         },
//                     ),
//                     ..Default::default()
//                 },
//                 Label,
//             ));

//             parent.spawn((
//                 TextBundle {
//                     text: Text::from_section(
//                         "What about my second text?",
//                         TextStyle {
//                             color: Color::CYAN,
//                             font_size: COMBAT_LOG_TEXT_SIZE,
//                             ..default()
//                         },
//                     ),
//                     ..Default::default()
//                 },
//                 Label,
//             ));

//             parent.spawn((
//                 TextBundle {
//                     text: Text::from_section(
//                         "\t\t\t\tWhat about tabbin' dis shit togatha!!",
//                         TextStyle {
//                             color: Color::WHITE,
//                             font_size: COMBAT_LOG_TEXT_SIZE,
//                             ..default()
//                         },
//                     ),
//                     ..Default::default()
//                 },
//                 Label,
//             ));

//             parent.spawn((
//                 TextBundle {
//                     text: Text::from_section(
//                         "What if I do \n new line?!!!",
//                         TextStyle {
//                             color: Color::WHITE,
//                             font_size: COMBAT_LOG_TEXT_SIZE,
//                             ..default()
//                         },
//                     ),
//                     ..Default::default()
//                 },
//                 Label,
//             ));
//         });

//     // let entity = cmd
//     //     .spawn((
//     //         // Create a TextBundle that has a Text with a single section.
//     //         TextBundle::from_section(
//     //             // Accepts a `String` or any type that converts into a `String`, such as `&str`
//     //             "hello\nbevy!",
//     //             TextStyle {
//     //                 // This font is loaded and will be used instead of the default font.
//     //                 font_size: 80.0,
//     //                 ..default()
//     //             },
//     //         ) // Set the justification of the Text
//     //         .with_text_justify(JustifyText::Center)
//     //         // Set the style of the TextBundle itself.
//     //         .with_style(Style {
//     //             position_type: PositionType::Absolute,
//     //             bottom: Val::Percent(0.0),
//     //             right: Val::Percent(50.0),
//     //             ..default()
//     //         }),
//     //     ))
//     //     .id();

//     // trace!(?entity, "text spawned");
// }

// fn add_text_node(
//     input: ResMut<ButtonInput<KeyCode>>,
//     mut cmd: Commands,
//     mut query: Query<(Entity, &Node), With<CombatLogWindow>>,
// ) {
//     if input.just_pressed(KeyCode::KeyA) {
//         trace!("say aAaaaaaAhhHHhhhh!!");
//         let (entity, node) = query.single();

//         cmd.entity(entity).with_children(|parent| {
//             parent.spawn((
//                 TextBundle {
//                     text: Text::from_section(
//                         "Imma spawnt bitch",
//                         TextStyle {
//                             color: Color::WHITE,
//                             font_size: COMBAT_LOG_TEXT_SIZE,
//                             ..default()
//                         },
//                     ),
//                     ..Default::default()
//                 },
//                 Label,
//             ));
//         });
//     }

//     if input.just_pressed(KeyCode::KeyS) {
//         trace!("sssssssss");
//         let (entity, node) = query.single();

//         let child_ent = cmd
//             .spawn((
//                 TextBundle {
//                     text: Text::from_section(
//                         "Don't push me :O",
//                         TextStyle {
//                             color: Color::YELLOW,
//                             font_size: COMBAT_LOG_TEXT_SIZE,
//                             ..default()
//                         },
//                     ),
//                     ..Default::default()
//                 },
//                 Label,
//             ))
//             .id();

//         cmd.entity(entity).insert_children(0, &[child_ent]);
//     }
// }
