use bevy::prelude::*;
use bevy_pkv::PkvStore;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        // remove reset_database later
        app.add_startup_system(reset_database)
            .insert_resource(SettingValues::default())
            .insert_resource(StatValues::default())
            .add_systems(Startup, setup_database);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mode {
    Auto,
    Manual,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct SettingValues {
    pub base_trials: u32,
    pub trial_factor: u32,
    pub trial_exponent: u32,
    pub mode: Mode,
    pub raise_threshold: f32,
    pub lower_threshold: f32,
    pub chance_of_interference: f32,
    pub chance_of_guaranteed_match: f32,
}

impl Default for SettingValues {
    fn default() -> Self {
        Self {
            base_trials: 20,
            trial_factor: 1,
            trial_exponent: 2,
            mode: Mode::Auto,
            raise_threshold: 80.0,
            lower_threshold: 50.0,
            chance_of_interference: 12.5,
            chance_of_guaranteed_match: 12.5,
        }
    }
}

#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub struct StatValues {
    pub current_level: u32,
    pub average_level_today: f32,
    pub sessions_today: u32,
    pub total_sessions: u32,
}

impl Default for StatValues {
    fn default() -> Self {
        Self {
            current_level: 1,
            average_level_today: 0.0,
            sessions_today: 0,
            total_sessions: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayEntry {
    pub day: DateTime<Utc>,
    pub average_level: f32,
    pub max_level: u32,
}

#[derive(Debug, Default, Resource, Serialize, Deserialize)]
pub struct EntryValues {
    pub day_entries: Vec<DayEntry>,
}

pub struct DatabasePlugin;

fn reset_database(mut pkv: ResMut<PkvStore>) {
    info!("Clearing Database");
    pkv.clear().expect("failed to clear database");
}

fn setup_database(mut commands: Commands, mut pkv: ResMut<PkvStore>) {
    if let Ok(settings) = pkv.get::<SettingValues>("settingValues") {
        info!("Loaded Prior Settings");
        commands.insert_resource(settings);
    } else {
        info!("Initialized Default Settings");
        let default_settings = SettingValues::default();

        pkv.set("settingValues", &default_settings)
            .expect("failed to store trials");

        commands.insert_resource(default_settings);
    }

    if let Ok(stats) = pkv.get::<StatValues>("statValues") {
        info!("Loaded Prior Stats");
        commands.insert_resource(stats);
    } else {
        info!("Initialized Stats");
        let default_stats = StatValues::default();

        pkv.set("statValues", &default_stats)
            .expect("failed to store trials");

        commands.insert_resource(default_stats);
    }

    if let Ok(entries) = pkv.get::<EntryValues>("entryValues") {
        info!("Loaded Day Entries");
        commands.insert_resource(entries);
    } else {
        info!("Initialized Day Entries");
        let default_entries = EntryValues::default();

        pkv.set("entryValues", &default_entries)
            .expect("failed to store trials");

        commands.insert_resource(default_entries);
    }
}
