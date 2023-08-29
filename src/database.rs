use crate::AppState;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_pkv::PkvStore;
use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SettingValues::default())
            .insert_resource(StatValues::default())
            .add_systems(OnEnter(AppState::Menu), sync_stats)
            .add_systems(Startup, (setup_database, setup_time));
    }
}

#[derive(Default, Clone, Copy, Debug, Resource, Serialize, Deserialize, PartialEq)]
pub enum Mode {
    #[default]
    Auto,
    Manual,
}

#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct CurrentDate {
    pub date: NaiveDate,
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    pub date: NaiveDate,
    pub level: u32,
    pub percent_score: u32,
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct RecentSessions {
    pub sessions: VecDeque<Session>,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct SettingValues {
    pub base_trials: u32,
    pub trial_factor: u32,
    pub trial_exponent: u32,
    pub mode: Mode,
    pub raise_threshold: f32,
    pub lower_threshold: f32,
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
    pub last_sync_date: NaiveDate,
}

impl Default for StatValues {
    fn default() -> Self {
        let now = Local::now();
        Self {
            current_level: 1,
            average_level_today: 0.0,
            sessions_today: 0,
            total_sessions: 0,
            last_sync_date: NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayEntry {
    pub date: NaiveDate,
    pub average_level: f32,
    pub sessions_completed: u32,
    pub max_level: u32,
}

#[derive(Debug, Default, Clone, Resource, Serialize, Deserialize)]
pub struct EntryValues {
    pub day_entries: HashMap<NaiveDate, DayEntry>,
}

pub fn clear_database(mut pkv: ResMut<PkvStore>) {
    info!("Clearing Database");
    pkv.clear().expect("failed to clear database");
}

fn setup_time(mut commands: Commands) {
    let now = Local::now();
    let date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap();

    commands.insert_resource(CurrentDate { date });
}

pub fn setup_database(mut commands: Commands, mut pkv: ResMut<PkvStore>) {
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
            .expect("failed to store stats");

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

    if let Ok(recent_sessions) = pkv.get::<RecentSessions>("recentSessions") {
        info!("Loaded Recent Sessions");
        commands.insert_resource(recent_sessions);
    } else {
        info!("Initialized Recent Sessions");
        let default_recent_sessions = RecentSessions::default();

        pkv.set("recentSessions", &default_recent_sessions)
            .expect("failed to store trials");

        commands.insert_resource(default_recent_sessions);
    }
}

fn sync_stats(mut pkv: ResMut<PkvStore>, stats: ResMut<StatValues>) {
    let date_today = Local::now().naive_local().date();
    if stats.last_sync_date != date_today {
        let mut new_stats = stats.clone();
        new_stats.sessions_today = 0;
        new_stats.average_level_today = 0.0;
        new_stats.last_sync_date = date_today;

        pkv.set("statValues", &new_stats)
            .expect("failed to store stats");
    } else {
        pkv.set("statValues", &*stats)
            .expect("failed to store stats");
    }
}
