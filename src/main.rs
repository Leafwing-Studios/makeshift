use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
        // Configure the game window
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: "Template".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
