use crate::{colors, despawn_screen, AppState, Mode, PkvStore, SettingValues};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use serde::{Deserialize, Serialize};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), setup_settings)
            .add_plugins(EguiPlugin)
            .add_systems(
                Update,
                settings_systems.run_if(state_exists_and_equals(AppState::Settings)),
            )
            .add_systems(
                OnExit(AppState::Settings),
                despawn_screen::<OnSettingsScreen>,
            );
    }
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct StagedSettingValues {
    pub manual_level: u32,
    pub base_trials: u32,
    pub trial_factor: u32,
    pub trial_exponent: u32,
    pub mode: Mode,
    pub raise_threshold: f32,
    pub lower_threshold: f32,
    pub chance_of_guaranteed_match: f32,
}

impl Default for StagedSettingValues {
    fn default() -> Self {
        Self {
            manual_level: 1,
            base_trials: 20,
            trial_factor: 2,
            trial_exponent: 2,
            mode: Mode::Auto,
            raise_threshold: 0.8,
            lower_threshold: 0.2,
            chance_of_guaranteed_match: 0.5,
        }
    }
}

#[derive(Component)]
pub struct OnSettingsScreen;

pub fn setup_settings(mut commands: Commands, settings: Res<SettingValues>) {
    println!("setting up");
    commands.insert_resource(StagedSettingValues {
        manual_level: settings.manual_level,
        base_trials: settings.base_trials,
        trial_factor: settings.trial_factor,
        trial_exponent: settings.trial_exponent,
        mode: settings.mode,
        raise_threshold: settings.raise_threshold,
        lower_threshold: settings.lower_threshold,
        chance_of_guaranteed_match: settings.chance_of_guaranteed_match,
    });
}

pub fn settings_systems(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut staged_settings: ResMut<StagedSettingValues>,
    mut pkv: ResMut<PkvStore>,
) {
    let ctx = contexts.ctx_mut();
    let screen_size = ctx.available_rect();
    let window_size = egui::Vec2 {
        x: screen_size.width(),
        y: screen_size.height(),
    };
    let position = egui::Pos2 {
        x: screen_size.width() / 2.0,
        y: screen_size.height() / 2.0,
    };

    egui::Window::new("Settings")
        .pivot(egui::Align2::CENTER_CENTER)
        .fixed_pos(position) // Center the window
        .default_size(egui::Vec2 { x: 500.0, y: 500.0 })
        .collapsible(false) // Make it non-collapsible
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width(screen_size.width() / 2.0);
            ui.set_height(screen_size.height() / 2.0);
            ui.separator();

            let mut selected_mode = &mut staged_settings.mode;
            ui.horizontal(|ui| {
                ui.label("Mode:");
                ui.selectable_value(selected_mode, Mode::Auto, "Auto");
                ui.selectable_value(selected_mode, Mode::Manual, "Manual");
            });

            if *selected_mode == Mode::Manual {
                let mut manual_level = &mut staged_settings.manual_level;
                ui.add(egui::Slider::new(manual_level, 1..=10).text("Manual Level"));
            }

            ui.separator();

            let mut base_trials = &mut staged_settings.base_trials;
            ui.add(egui::Slider::new(base_trials, 1..=100).text("Base Trials"));

            let mut trial_factor = &mut staged_settings.trial_factor;
            ui.add(egui::Slider::new(trial_factor, 1..=10).text("Trial Factor"));

            let mut trial_exponent = &mut staged_settings.trial_exponent;
            ui.add(egui::Slider::new(trial_exponent, 1..=10).text("Trial Exponent"));

            ui.separator();

            let mut raise_threshold = &mut staged_settings.raise_threshold;
            ui.add(egui::Slider::new(raise_threshold, 0.5..=1.0).text("Raise Threshold"));

            let mut lower_threshold = &mut staged_settings.lower_threshold;
            ui.add(egui::Slider::new(lower_threshold, 0.0..=0.49).text("Lower Threshold"));

            let mut chance_of_guaranteed_match = &mut staged_settings.chance_of_guaranteed_match;
            ui.add(
                egui::Slider::new(chance_of_guaranteed_match, 0.0..=1.0)
                    .text("Chance of Guaranteed Match"),
            );

            ui.separator();

            if ui.button("Save").clicked() {
                let settingValues = SettingValues {
                    manual_level: staged_settings.manual_level,
                    base_trials: staged_settings.base_trials,
                    trial_factor: staged_settings.trial_factor,
                    trial_exponent: staged_settings.trial_exponent,
                    mode: staged_settings.mode,
                    raise_threshold: staged_settings.raise_threshold,
                    lower_threshold: staged_settings.lower_threshold,
                    chance_of_guaranteed_match: staged_settings.chance_of_guaranteed_match,
                };
                pkv.set("settings", &settingValues)
                    .expect("failed to store settings");
                commands.insert_resource(settingValues);
            }
        });
}
