use bevy::prelude::*;
use bevy_pkv::PkvStore;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Mode {
    Auto,
    Manual,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct SettingValues {
    pub trials: u32,
    pub mode: Mode,
}

impl Default for SettingValues {
    fn default() -> Self {
        Self {
            trials: 20,
            mode: Mode::Auto,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayEntry {
    pub day: DateTime<Utc>,
    pub average_level: f32,
    pub max_level: u32,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct StatValues {
    pub current_level: u32,
    pub average_level_today: f32,
    pub sessions_today: u32,
    pub total_sessions: u32,
    pub total_days: u32,
    pub total_time: u32,
    pub day_entries: Vec<DayEntry>,
}

impl Default for StatValues {
    fn default() -> Self {
        Self {
            current_level: 1,
            average_level_today: 0.0,
            sessions_today: 0,
            total_sessions: 0,
            total_days: 0,
            total_time: 0,
            day_entries: Vec::new(),
        }
    }
}

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SettingValues::default())
            .insert_resource(StatValues::default())
            .add_systems(Startup, setup_database);
    }
}

fn setup_database(mut commands: Commands, mut pkv: ResMut<PkvStore>) {
    if let Ok(settings) = pkv.get::<SettingValues>("settingValues") {
        info!("Loaded Prior Settings");
        commands.insert_resource(settings);
    } else {
        let default_settings = SettingValues::default();

        pkv.set("settingValues", &default_settings)
            .expect("failed to store trials");

        commands.insert_resource(default_settings);
    }

    if let Ok(stats) = pkv.get::<StatValues>("statValues") {
        info!("Loaded Prior Stats");
        commands.insert_resource(stats);
    } else {
        let default_stats = StatValues::default();

        pkv.set("statValues", &default_stats)
            .expect("failed to store trials");

        commands.insert_resource(default_stats);
    }
}
