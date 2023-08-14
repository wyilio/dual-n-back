use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use menu::*;
use progress::*;
use session::*;
use target::*;
use title::*;

mod menu;
mod progress;
mod session;
mod target;
mod title;

const BACKGROUND_COLOR: Color = Color::rgb(0.922, 0.922, 0.918);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Menu,
    Session,
    Settings,
    Progress,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum SessionState {
    Active,
    Paused,
    Restart,
    #[default]
    Inactive,
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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(TrialTimer(Timer::new(
            Duration::from_millis(3000),
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<AppState>()
        .add_state::<SessionState>()
        .add_systems(Startup, setup_camera)
        .add_plugins(MenuPlugin)
        .add_plugins(SessionPlugin)
        .run();
}
