use crate::GameMode;
use bevy::prelude::*;

pub struct LearnPlugin;

impl Plugin for LearnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameMode::Learn).with_system(spawn))
            .add_system_set(SystemSet::on_exit(GameMode::Learn).with_system(despawn));
    }
}

/// A marker component for all entities in the Learn scene
#[derive(Component)]
struct Learn;

fn spawn() {}

fn despawn(mut commands: Commands, query: Query<Entity, With<Learn>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
