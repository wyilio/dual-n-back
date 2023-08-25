use crate::{colors, despawn_screen, AppState, SettingValues, StatValues};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter;
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrialTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
            .add_systems(
                OnEnter(AppState::Session),
                (
                    setup_grid,
                    setup_stimuli_buttons,
                    setup_targets,
                    setup_trial,
                ),
            )
            .add_systems(
                Update,
                (
                    stimuli_button_system,
                    stimuli_button_action,
                    stimuli_visibility_system,
                    target_transition_system,
                    trial_progression_system,
                    trial_count_system,
                )
                    .run_if(in_state(AppState::Session)),
            )
            .add_systems(
                Update,
                exit_session.run_if(
                    state_exists_and_equals(AppState::Session)
                        .or_else(state_exists_and_equals(AppState::Menu)),
                ),
            )
            .add_systems(OnExit(AppState::Session), despawn_screen::<OnSessionScreen>);
    }
}

pub const CELL_SIZE: f32 = 150.0;
pub const VERTICAL_OFFSET: f32 = 75.0;
pub const GRID_THICKNESS: f32 = 2.0;
pub const GRID_LENGTH: f32 = 3.0 * CELL_SIZE;

#[derive(EnumIter, Component, Debug, Copy, Clone, Eq, PartialEq)]
pub enum TargetAudio {
    C,
    H,
    K,
    L,
    Q,
    R,
    S,
    T,
}

impl TargetAudio {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..8) {
            0 => TargetAudio::C,
            1 => TargetAudio::H,
            2 => TargetAudio::K,
            3 => TargetAudio::L,
            4 => TargetAudio::Q,
            5 => TargetAudio::R,
            6 => TargetAudio::S,
            7 => TargetAudio::T,
            _ => unreachable!(),
        }
    }
}

#[derive(EnumIter, Component, Debug, Copy, Clone, Eq, PartialEq)]
pub enum TargetLocation {
    TopLeft,
    TopMiddle,
    TopRight,
    CenterLeft,
    CenterMiddle,
    CenterRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

impl TargetLocation {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..9) {
            0 => TargetLocation::TopLeft,
            1 => TargetLocation::TopMiddle,
            2 => TargetLocation::TopRight,
            3 => TargetLocation::CenterLeft,
            4 => TargetLocation::CenterMiddle,
            5 => TargetLocation::CenterRight,
            6 => TargetLocation::BottomLeft,
            7 => TargetLocation::BottomMiddle,
            8 => TargetLocation::BottomRight,
            _ => unreachable!(),
        }
    }
}

pub fn get_target_coordinates(target_location: TargetLocation) -> (f32, f32) {
    match target_location {
        TargetLocation::TopLeft => (-1.0 * CELL_SIZE, CELL_SIZE),
        TargetLocation::TopMiddle => (0.0, CELL_SIZE),
        TargetLocation::TopRight => (CELL_SIZE, CELL_SIZE),
        TargetLocation::CenterLeft => (-1.0 * CELL_SIZE, 0.0),
        TargetLocation::CenterMiddle => (0.0, 0.0),
        TargetLocation::CenterRight => (CELL_SIZE, 0.0),
        TargetLocation::BottomLeft => (-1.0 * CELL_SIZE, -1.0 * CELL_SIZE),
        TargetLocation::BottomMiddle => (0.0, -1.0 * CELL_SIZE),
        TargetLocation::BottomRight => (CELL_SIZE, -1.0 * CELL_SIZE),
    }
}

#[derive(Component)]
pub struct OnSessionScreen;

#[derive(Debug, Resource)]
pub struct TrialTimer(pub Timer);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Occupied,
    Empty,
}

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grid_sprite = Sprite {
        color: colors::SECONDARY_COLOR,
        custom_size: Some(Vec2::new(GRID_LENGTH, GRID_LENGTH)),
        ..Default::default()
    };

    commands.spawn((
        SpriteBundle {
            sprite: grid_sprite.clone(),
            transform: Transform::from_translation(Vec3::new(CELL_SIZE / 2.0, VERTICAL_OFFSET, 0.)),
            ..Default::default()
        },
        OnSessionScreen,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: grid_sprite.clone(),
            transform: Transform::from_translation(Vec3::new(
                -1.0 * CELL_SIZE / 2.0,
                VERTICAL_OFFSET,
                0.,
            )),
            ..Default::default()
        },
        OnSessionScreen,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: grid_sprite.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -1.0 * CELL_SIZE / 2.0 + VERTICAL_OFFSET,
                0.,
            )),
            ..default()
        },
        OnSessionScreen,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: grid_sprite.clone(),
            transform: Transform::from_translation(Vec3::new(
                0.0,
                CELL_SIZE / 2.0 + VERTICAL_OFFSET,
                0.,
            )),
            ..default()
        },
        OnSessionScreen,
    ));
}

pub fn exit_session(
    app_state: Res<State<AppState>>,
    mut change_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let AppState::Session = app_state.get() {
            change_app_state.set(AppState::Menu);
        } else {
            change_app_state.set(AppState::Session);
        }
    }
}

pub fn stimuli_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StimuliButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => colors::PRESSED_BUTTON_DARK.into(),
            Interaction::Hovered => colors::HOVERED_BUTTON_DARK.into(),
            Interaction::None => colors::TRANSPARENT_COLOR.into(),
        }
    }
}

pub fn stimuli_button_action(
    mut interaction_query: Query<
        (&Interaction, &mut MatchState, &StimuliButtonAction),
        (Changed<Interaction>, With<StimuliButton>),
    >,
) {
    for (interaction, mut visibility_state, stimuli_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *visibility_state = MatchState::NonResponse;

                if let StimuliButtonAction::MatchPosition = *stimuli_button_action {
                    println!("Match position");
                } else if let StimuliButtonAction::MatchAudio = *stimuli_button_action {
                    println!("Match audio");
                }
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub enum StimuliButtonAction {
    MatchPosition,
    MatchAudio,
}

#[derive(Component)]
pub enum MatchState {
    Match,
    NonResponse,
}

#[derive(Component)]
pub struct StimuliButton;

fn spawn_stimuli_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    action: StimuliButtonAction,
) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                background_color: colors::TRANSPARENT_COLOR.into(),
                border_color: colors::PRIMARY_COLOR.into(),
                ..Default::default()
            },
            StimuliButton,
            OnSessionScreen,
            MatchState::NonResponse,
            action,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: colors::PRIMARY_COLOR.into(),
                },
            ));
        });
}

#[derive(Resource)]
pub struct TrialCount(pub u32);

#[derive(Component)]
pub struct TrialLabel;

#[derive(Resource)]
pub struct StimuliGeneration {
    stimuli: Vec<(TargetLocation, TargetAudio)>,
}

pub fn setup_trial(
    mut commands: Commands,
    settings: Res<SettingValues>,
    stats: Res<StatValues>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(TrialCount(
        settings.base_trials
            + (settings.trial_factor * stats.current_level.pow(settings.trial_exponent)),
    ));

    let stimuli = iter::repeat_with(|| (TargetLocation::random(), TargetAudio::random()))
        .take(stats.current_level as usize)
        .collect::<Vec<(TargetLocation, TargetAudio)>>();

    commands.insert_resource(StimuliGeneration { stimuli });

    commands.spawn((
        TextBundle::from_section(
            "Trials Left: ",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: colors::PRIMARY_COLOR.into(),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Px(30.0)),
            ..Default::default()
        }),
        OnSessionScreen,
        TrialLabel,
    ));
}

pub fn trial_count_system(
    mut commands: Commands,
    mut trial_label_query: Query<(&mut Text), (With<TrialLabel>)>,
    mut trial_count: ResMut<TrialCount>,
) {
    if trial_count.is_changed() {
        for mut text in &mut trial_label_query {
            text.sections[0].value = format!("Trials Left: {}", trial_count.0);
        }
    }
}

pub fn setup_targets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for target_location in TargetLocation::iter() {
        let target_coordinates = get_target_coordinates(target_location);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform::from_translation(Vec3::new(
                    target_coordinates.0,
                    target_coordinates.1 + VERTICAL_OFFSET,
                    0.0,
                ))
                .with_scale(Vec3::splat(128.)),
                visibility: Visibility::Hidden,
                material: materials.add(ColorMaterial::from(colors::PRIMARY_COLOR)),
                ..Default::default()
            },
            OnSessionScreen,
            DisplayTargetTime {
                timer: Timer::default(),
            },
            target_location,
        ));
    }
}

#[derive(Component)]
pub struct DisplayTargetTime {
    pub timer: Timer,
}

pub fn stimuli_visibility_system(
    mut stimuli_button_query: Query<(&mut Visibility, &MatchState), Changed<MatchState>>,
) {
    for (mut button_visibility, visibility_state) in &mut stimuli_button_query {
        *button_visibility = match *visibility_state {
            MatchState::Match => Visibility::Visible,
            MatchState::NonResponse => Visibility::Hidden,
        }
    }
}

pub fn trial_progression_system(
    mut target_query: Query<
        (&TargetLocation, &mut Visibility, &mut DisplayTargetTime),
        With<TargetLocation>,
    >,
    mut stimuli_button_query: Query<&mut MatchState, With<StimuliButton>>,
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TrialTimer>,
    mut stimuli_generation: ResMut<StimuliGeneration>,
    settings: Res<SettingValues>,
    stats: Res<StatValues>,
    mut trial_count: ResMut<TrialCount>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let previous_stimuli = stimuli_generation.stimuli.clone();
        let mut new_stimuli = Vec::new();
        let n_level = stats.current_level as usize;

        let mut rng = rand::thread_rng();
        let random_target_location = TargetLocation::random();

        for i in 0..n_level {
            let mut target_location = TargetLocation::random();
            let mut target_audio = TargetAudio::random();

            let mut location_roll: f32 = rng.gen();
            if location_roll < settings.chance_of_guaranteed_match {
                target_location = previous_stimuli[i].0;
                println!("Guaranteed location match: {:?}", target_location);
            } else {
                location_roll = rng.gen();
                if location_roll < settings.chance_of_interference {
                    let left_back_roll: f32 = rng.gen();
                    if left_back_roll < 0.5 && i != 0 || (i == n_level - 1) {
                        target_location = previous_stimuli[i - 1].0;
                    } else {
                        target_location = previous_stimuli[i + 1].0;
                    }
                    println!("Interference location match: {:?}", target_location);
                }
            }

            let mut audio_roll: f32 = rng.gen();
            if audio_roll < settings.chance_of_guaranteed_match {
                target_audio = previous_stimuli[i].1;
                println!("Guaranteed audio match: {:?}", target_audio);
            } else {
                audio_roll = rng.gen();
                if audio_roll < settings.chance_of_interference {
                    let left_back_roll: f32 = rng.gen();
                    if left_back_roll < 0.5 && i != 0 || (i == n_level - 1) {
                        target_audio = previous_stimuli[i - 1].1;
                    } else {
                        target_audio = previous_stimuli[i + 1].1;
                    }
                    println!("Interference audio match: {:?}", target_audio);
                }
            }

            new_stimuli.push((target_location, target_audio));
        }
        commands.insert_resource(StimuliGeneration {
            stimuli: new_stimuli,
        });

        for (target_location, mut target_visibility, mut display_target_time) in &mut target_query {
            if *target_location == random_target_location {
                *target_visibility = Visibility::Visible;
                display_target_time.timer = Timer::from_seconds(0.5, TimerMode::Once);
            } else {
                *target_visibility = Visibility::Hidden;
            }
        }

        for (mut stimuli_button_visibility) in &mut stimuli_button_query {
            *stimuli_button_visibility = MatchState::Match;
        }

        if trial_count.0 == 0 {
            app_state.set(AppState::Menu);
        } else {
            *trial_count = TrialCount(trial_count.0 - 1);
        }
    }
}

pub fn target_transition_system(
    mut target_query: Query<(&mut Visibility, &mut DisplayTargetTime), With<TargetLocation>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut visibility, mut display_target_time) in &mut target_query {
        if display_target_time.timer.tick(time.delta()).just_finished() {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn setup_stimuli_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Percent(80.),
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            OnSessionScreen,
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(2.0 * CELL_SIZE),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    spawn_stimuli_button(
                        builder,
                        font.clone(),
                        "Position",
                        StimuliButtonAction::MatchPosition,
                    );
                    spawn_stimuli_button(
                        builder,
                        font.clone(),
                        "Audio",
                        StimuliButtonAction::MatchAudio,
                    );
                });
        });
}
