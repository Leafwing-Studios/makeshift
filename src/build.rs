use crate::GameMode;
use bevy::prelude::*;

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameMode::Build).with_system(spawn))
            .add_system_set(SystemSet::on_exit(GameMode::Build).with_system(despawn));
    }
}

/// A marker component for all entities in the Build scene
#[derive(Component)]
struct Build;

fn spawn() {}

fn despawn(mut commands: Commands, query: Query<Entity, With<Build>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
