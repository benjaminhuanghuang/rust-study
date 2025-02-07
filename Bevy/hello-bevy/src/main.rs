use bevy::prelude::*;

pub const CLEAR: Color = Color::srgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
  let height = 720.0;
  // App is [`bevy::app::App`]
  App::new()
    .insert_resource(ClearColor(CLEAR))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "My Bevy Window".to_string(),
        resolution: (800., 600.).into(),
        resizable: false,
        ..Default::default()
      }),
      ..Default::default()
    }))
    .run();
}
