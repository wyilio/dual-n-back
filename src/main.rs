#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy_pkv::PkvStore;
use winit::window::Icon;

use colors::*;
use database::*;
use menu::*;
use session::*;
use settings::*;

mod colors;
mod database;
mod menu;
mod session;
mod settings;

pub fn set_window_icon(
    main_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let Some(primary) = windows.get_window(main_window.single()) else {return};

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("brain.ico")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}

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
        .add_state::<AppState>()
        .add_systems(Startup, (setup_camera, set_window_icon))
        .add_plugins(DatabasePlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SessionPlugin)
        .add_plugins(SettingsPlugin)
        .run();
}
