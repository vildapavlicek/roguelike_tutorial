use bevy::prelude::Color;

// SPRITE RELATED
pub const SPRITE_SIZE: f32 = 32f32;

// Z LAYER INDEX
pub const PLAYER_Z: f32 = 20f32;
pub const FLOOR_Z: f32 = 0f32;
pub const WALL_Z: f32 = 1f32;
pub const ITEM_Z: f32 = 2f32;
pub const MONSTER_Z: f32 = 10f32;

// MISC
/// This is how much opacity the sprite should have when hidden by Fog of War
pub const FOW_ALPHA: f32 = 0.25;

// UI
pub const FONT_SIZE: f32 = 14.;
pub const DEFAULT_TEXT_COLOR: Color = Color::WHITE;
