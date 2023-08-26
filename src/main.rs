use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pkv::PkvStore;
use leafwing_input_manager::prelude::*;

use colors::*;
use database::*;
use menu::*;
use progress::*;
use session::*;

mod colors;
mod database;
mod menu;
mod progress;
mod session;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Menu,
    Session,
    Settings,
    Progress,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn main() {
    App::new()
        .insert_resource(PkvStore::new("Bevy_DNB", "Bevy_DNB_config"))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        // .add_plugins(InputManagerPlugin::<Action>::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<AppState>()
        // .add_state::<SessionState>()
        .add_systems(Startup, (setup_camera))
        .add_plugins(DatabasePlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SessionPlugin)
        .run();
}
