use crate::{GameMode, GamePaused};
use bevy::prelude::*;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameMode::Play)
                .with_system(spawn)
                .with_system(unpause_game),
        )
        .add_system_set(SystemSet::on_exit(GameMode::Play).with_system(despawn));
    }
}

/// A marker component for all entities in the Play scene
#[derive(Component)]
struct Play;

fn spawn() {}

fn despawn(mut commands: Commands, query: Query<Entity, With<Play>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn unpause_game(mut paused: ResMut<GamePaused>) {
    *paused = GamePaused::Unpaused;
}
