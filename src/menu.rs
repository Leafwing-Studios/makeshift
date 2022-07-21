use crate::GameMode;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameMode::Menu).with_system(spawn))
            .add_system_set(SystemSet::on_exit(GameMode::Menu).with_system(despawn));
    }
}

/// A marker component for all entities in the main menu scene
#[derive(Component)]
struct Menu;

fn spawn() {}

fn despawn(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
