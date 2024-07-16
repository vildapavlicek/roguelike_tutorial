pub mod log;
mod tooltip;

use bevy::prelude::*;

#[derive(Debug, Copy, Clone, Component)]

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((log::LogUiPlugin, tooltip::TooltipPlugin));
    }
}
