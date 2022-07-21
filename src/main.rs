use bevy::prelude::*;
use bevy::window::WindowMode;

mod build;
mod learn;
mod menu;
mod play;

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
        .add_state(GameMode::Menu)
        .insert_resource(GamePaused::Paused)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(build::BuildPlugin)
        .add_plugin(learn::LearnPlugin)
        .add_plugin(play::PlayPlugin)
        .run();
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum GameMode {
    Menu,
    Build,
    Learn,
    Play,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum GamePaused {
    Paused,
    Unpaused,
}
