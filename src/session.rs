use crate::{colors, despawn_screen, AppState, SettingValues, StatValues};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter;
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ResourceInspectorPlugin::<TrialCount>::default())
            .add_plugin(ResourceInspectorPlugin::<StatValues>::default())
            .add_plugin(ResourceInspectorPlugin::<Score>::default())
            .add_plugin(ResourceInspectorPlugin::<StimuliGeneration>::default())
            .insert_resource(TrialTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
            .add_state::<SessionState>()
            .add_systems(
                OnEnter(AppState::Session),
                (
                    setup_session_state,
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
            .add_systems(OnEnter(SessionState::Exit), exit_session_system)
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

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SessionState {
    Exit,
    #[default]
    Active,
}

#[derive(EnumIter, Component, Debug, Copy, Clone, Eq, PartialEq, Reflect)]
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

#[derive(EnumIter, Component, Debug, Copy, Clone, Eq, PartialEq, Reflect)]
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

pub fn setup_session_state(mut session_state: ResMut<NextState<SessionState>>) {
    session_state.set(SessionState::Active);
}

pub fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: colors::SECONDARY_COLOR,
                custom_size: Some(Vec2::new(GRID_THICKNESS, GRID_LENGTH)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(CELL_SIZE / 2.0, VERTICAL_OFFSET, 0.)),
            ..Default::default()
        },
        OnSessionScreen,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: colors::SECONDARY_COLOR,
                custom_size: Some(Vec2::new(GRID_THICKNESS, GRID_LENGTH)),
                ..Default::default()
            },
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
            sprite: Sprite {
                color: colors::SECONDARY_COLOR,
                custom_size: Some(Vec2::new(GRID_LENGTH, GRID_THICKNESS)),
                ..Default::default()
            },
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
            sprite: Sprite {
                color: colors::SECONDARY_COLOR,
                custom_size: Some(Vec2::new(GRID_LENGTH, GRID_THICKNESS)),
                ..default()
            },
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

pub fn exit_session_system(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    score: ResMut<Score>,
    mut stats: ResMut<StatValues>,
    settings: ResMut<SettingValues>,
    trial_count: ResMut<TrialCount>,
) {
    let percent_score = (score.position_correct + score.audio_correct) as f32
        / (2 * (trial_count.total_count - stats.current_level)) as f32;

    println!("Percent Score: {}", percent_score);
    let mut new_stats = stats.clone();

    stats.average_level_today = (stats.average_level_today * stats.sessions_today as f32
        + stats.current_level as f32)
        / (stats.sessions_today + 1) as f32;

    println!("Average Level Today: {}", new_stats.average_level_today);

    stats.sessions_today += 1;
    stats.total_sessions += 1;

    if percent_score > settings.raise_threshold / 100.0 {
        stats.current_level += 1;
        println!("Level Up!");
    } else if percent_score < settings.lower_threshold / 100.0 {
        if stats.current_level > 1 {
            stats.current_level -= 1;
        }
        println!("Level Down!");
    }

    app_state.set(AppState::Menu);
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
        (&Interaction, &mut MatchState),
        (Changed<Interaction>, With<StimuliButton>),
    >,
) {
    for (interaction, mut match_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *match_state = MatchState::Match;
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

#[derive(Component, PartialEq)]
pub enum MatchState {
    Match,
    NonResponse,
    Inactive,
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
            MatchState::Inactive,
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

// #[derive(Resource)]
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct TrialCount {
    pub current_count: u32,
    pub total_count: u32,
    pub final_trial: bool,
}

// #[derive(Resource)]
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Score {
    pub position_correct: u32,
    pub audio_correct: u32,
}

#[derive(Component)]
pub struct TrialLabel;

// #[derive(Resource)]
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct StimuliGeneration {
    stimuli: Vec<(TargetLocation, TargetAudio)>,
    previous: Vec<(TargetLocation, TargetAudio)>,
    index: usize,
}

pub fn setup_trial(
    mut commands: Commands,
    settings: Res<SettingValues>,
    stats: Res<StatValues>,
    asset_server: Res<AssetServer>,
) {
    let total_count = settings.base_trials
        + (settings.trial_factor * stats.current_level.pow(settings.trial_exponent));

    commands.insert_resource(TrialCount {
        current_count: total_count,
        total_count,
        final_trial: false,
    });

    let stimuli = iter::repeat_with(|| (TargetLocation::random(), TargetAudio::random()))
        .take(stats.current_level as usize)
        .collect::<Vec<(TargetLocation, TargetAudio)>>();

    commands.insert_resource(StimuliGeneration {
        stimuli: stimuli,
        previous: Vec::new(),
        index: 0,
    });
    commands.insert_resource(Score {
        position_correct: 0,
        audio_correct: 0,
    });

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
            text.sections[0].value = format!("Trials Left: {}", trial_count.current_count);
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
            MatchState::Match | MatchState::Inactive => Visibility::Hidden,
            MatchState::NonResponse => Visibility::Visible,
        }
    }
}

pub fn trial_progression_system(
    mut target_query: Query<
        (&TargetLocation, &mut Visibility, &mut DisplayTargetTime),
        With<TargetLocation>,
    >,
    mut stimuli_button_query: Query<(&mut MatchState, &StimuliButtonAction), With<StimuliButton>>,
    mut commands: Commands,
    mut timer: ResMut<TrialTimer>,
    mut stimuli_generation: ResMut<StimuliGeneration>,
    mut session_state: ResMut<NextState<SessionState>>,
    mut trial_count: ResMut<TrialCount>,
    mut score: ResMut<Score>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    settings: Res<SettingValues>,
    stats: Res<StatValues>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let trials_completed = trial_count.total_count - trial_count.current_count;

        let generation_index = (trials_completed % stats.current_level) as usize;
        let mut new_stimuli = Vec::new();

        for (mut match_state, stimuli_button_action) in &mut stimuli_button_query {
            if trials_completed > stats.current_level {
                let index = stimuli_generation.index;
                if let StimuliButtonAction::MatchPosition = *stimuli_button_action {
                    if stimuli_generation.stimuli[index].0 == stimuli_generation.previous[index].0 {
                        if let MatchState::Match = *match_state {
                            println!("Position match");
                            score.position_correct += 1;
                        } else {
                            println!("Wrong position match");
                        }
                    } else {
                        if let MatchState::NonResponse = *match_state {
                            println!("Position match");
                            score.position_correct += 1;
                        } else {
                            println!("Wrong position match");
                        }
                    }
                } else if let StimuliButtonAction::MatchAudio = *stimuli_button_action {
                    if stimuli_generation.stimuli[index].1 == stimuli_generation.previous[index].1 {
                        if *match_state == MatchState::Match {
                            println!("Audio match");
                            score.audio_correct += 1;
                        } else {
                            println!("Wrong audio match");
                        }
                    } else {
                        if *match_state == MatchState::NonResponse {
                            println!("Audio match");
                            score.audio_correct += 1;
                        } else {
                            println!("Wrong audio match");
                        }
                    }
                }
            }
            if trials_completed >= stats.current_level {
                *match_state = MatchState::NonResponse;
            }
        }

        if trial_count.final_trial {
            println!("Final trial finished {}", trial_count.current_count);
            session_state.set(SessionState::Exit);
            return;
        }

        if generation_index == 0 && trial_count.current_count != trial_count.total_count {
            let mut rng = rand::thread_rng();

            let n_level = stats.current_level as usize;

            for i in 0..n_level {
                let mut target_location = TargetLocation::random();
                let mut target_audio = TargetAudio::random();

                let mut location_roll: f32 = rng.gen();
                if location_roll < (settings.chance_of_guaranteed_match / 100.0) {
                    target_location = stimuli_generation.stimuli[i].0;
                    println!("Guaranteed location match: {:?}", target_location);
                } else if n_level != 1
                    && location_roll
                        < ((settings.chance_of_guaranteed_match + settings.chance_of_interference)
                            / 100.0)
                {
                    let left_back_roll: f32 = rng.gen();
                    if (i == n_level - 1) || left_back_roll < 0.5 && i != 0 {
                        target_location = stimuli_generation.stimuli[i - 1].0;
                    } else {
                        target_location = stimuli_generation.stimuli[i + 1].0;
                    }
                    println!("Interference location match: {:?}", target_location);
                }

                // TODO: Remove code duplication
                let mut audio_roll: f32 = rng.gen();
                if audio_roll < (settings.chance_of_guaranteed_match / 100.0) {
                    target_audio = stimuli_generation.stimuli[i].1;
                    println!("Guaranteed audio match: {:?}", target_audio);
                } else if n_level != 1
                    && audio_roll
                        < ((settings.chance_of_guaranteed_match + settings.chance_of_interference)
                            / 100.0)
                {
                    let left_back_roll: f32 = rng.gen();
                    if (i == n_level - 1) || left_back_roll < 0.5 && i != 0 {
                        target_audio = stimuli_generation.stimuli[i - 1].1;
                    } else {
                        target_audio = stimuli_generation.stimuli[i + 1].1;
                    }
                }
                new_stimuli.push((target_location, target_audio));
            }
            commands.insert_resource(StimuliGeneration {
                stimuli: new_stimuli.clone(),
                previous: stimuli_generation.stimuli.clone(),
                index: generation_index,
            });
        } else {
            stimuli_generation.index = generation_index;
        }

        let mut current_stimuli = &stimuli_generation.stimuli[generation_index];
        if generation_index == 0 && trial_count.current_count != trial_count.total_count {
            current_stimuli = &new_stimuli[0]
        }

        for (target_location, mut target_visibility, mut display_target_time) in &mut target_query {
            if *target_location == current_stimuli.0 {
                *target_visibility = Visibility::Visible;
                display_target_time.timer = Timer::from_seconds(0.5, TimerMode::Once);
            } else {
                *target_visibility = Visibility::Hidden;
            }
        }

        play_sound(&mut commands, &asset_server, current_stimuli.1);

        trial_count.current_count = trial_count.current_count - 1;
        if trial_count.current_count == 0 {
            println!("Final trial");
            trial_count.final_trial = true;
        }
    }
}

pub fn play_sound(commands: &mut Commands, asset_server: &Res<AssetServer>, audio: TargetAudio) {
    println!("Playing sound: {:?}", audio);

    let audio_file = match audio {
        TargetAudio::C => "letters/c.wav",
        TargetAudio::H => "letters/h.wav",
        TargetAudio::K => "letters/k.wav",
        TargetAudio::L => "letters/l.wav",
        TargetAudio::Q => "letters/q.wav",
        TargetAudio::R => "letters/r.wav",
        TargetAudio::S => "letters/s.wav",
        TargetAudio::T => "letters/t.wav",
    };
    commands.spawn(AudioBundle {
        source: asset_server.load(audio_file),
        ..Default::default()
    });
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
