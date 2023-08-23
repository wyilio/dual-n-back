use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Resource, Serialize, Deserialize)]
pub struct SettingValues {
    pub trials: u32,
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SettingValues::default())
            .add_systems(Startup, setup_config);
    }
}

fn setup_config(mut commands: Commands, mut pkv: ResMut<PkvStore>) {
    if let Ok(settings) = pkv.get::<SettingValues>("settings") {
        info!("Loaded Prior Settings");
        commands.insert_resource(settings);
    } else {
        let default_settings = SettingValues { trials: 20 };

        pkv.set("settings", &default_settings)
            .expect("failed to store trials");
        commands.insert_resource(default_settings);
    }
}