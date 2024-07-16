use crate::{
    components::{MainCamera, Name, Position},
    consts::{DEFAULT_TEXT_COLOR, FONT_SIZE},
    resources::CursorPosition,
};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Debug, Clone, Copy, Component)]
pub struct TooltipText;

pub(super) struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_cursor_position, update_tooltip).chain());
    }
}

fn update_cursor_position(
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_position: ResMut<crate::resources::CursorPosition>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_position.set(
            Position::from_coords(world_position.x, world_position.y, 99f32),
            window.cursor_position().unwrap_or_default(),
        );
        // debug!(?cursor_position, "updated cursor position");
    }
}

fn update_tooltip(
    mut cmd: Commands,
    cursor_position: Res<CursorPosition>,
    query: Query<(&Position, &Name)>,
    mut tooltip: Query<(&mut Visibility, &mut Text, &mut Style), With<TooltipText>>,
) {
    fn names_to_text_sections(names: Vec<String>) -> Vec<TextSection> {
        names
            .into_iter()
            .map(|name| {
                TextSection::new(
                    name,
                    TextStyle {
                        font_size: FONT_SIZE,
                        color: DEFAULT_TEXT_COLOR,
                        ..default()
                    },
                )
            })
            .collect::<Vec<_>>()
    }

    let names = query
        .iter()
        .filter_map(|(position, name)| {
            (cursor_position.world_position() == position).then_some(name.to_string())
        })
        .collect::<Vec<String>>();

    match tooltip.get_single_mut() {
        // No names to display, hide the text
        Ok((mut visibility, _, _))
            if names.is_empty() && !matches!(*visibility, Visibility::Hidden) =>
        {
            *visibility = Visibility::Hidden;
        }
        // we have some names to display and we also already have an tooltip, so we update names and
        // make sure that the tooltip is visible
        Ok((mut visibility, mut text, mut style)) => {
            let sections = names_to_text_sections(names);

            text.sections = sections;
            style.left = Val::Px(cursor_position.window_position.x + 10f32);
            style.top = Val::Px(cursor_position.window_position.y);

            if !matches!(*visibility, Visibility::Visible) {
                *visibility = Visibility::Visible;
            }
        }
        // no tooltip exists, so we have to crate a new one
        Err(_) if !names.is_empty() => {
            let sections = names_to_text_sections(names);
            cmd.spawn((
                TextBundle {
                    text: Text::from_sections(sections),
                    background_color: BackgroundColor(Color::BLACK),
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(cursor_position.window_position.x),
                        top: Val::Px(cursor_position.window_position.y),
                        ..default()
                    },
                    ..default()
                },
                TooltipText,
            ));
        }
        // there is no tooltip and there should also be no names to handle, so we do nothing
        Err(_) => (),
    }
}
