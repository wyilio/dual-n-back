use bevy::app::AppExit;
use bevy::{prelude::*, window::WindowResized};

use crate::{colors, despawn_screen, AppState, PkvStore, RecentSessions, StatValues};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Menu),
            (setup_menu, setup_scoreboard, setup_sessionboard),
        )
        .add_state::<MenuState>()
        .add_systems(
            Update,
            (menu_button_system, menu_action, window_resize_system)
                .run_if(in_state(AppState::Menu)),
        )
        .add_systems(OnExit(AppState::Menu), despawn_screen::<OnMenuScreen>);
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}

#[derive(Component)]
pub struct MenuButton;

#[derive(Component)]
pub struct MinWindowWidth;

#[derive(Component)]
pub struct OnMenuScreen;

#[derive(Component)]
pub struct OnSettingsScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    Start,
    Settings,
    Progress,
}

pub fn window_resize_system(
    mut q: Query<&mut Visibility, With<MinWindowWidth>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.iter() {
        for mut visibility in &mut q {
            if e.width < 1000.0 {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
            }
        }
    }
}

pub fn menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MenuButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed | Interaction::None => colors::PRESSED_BUTTON_LIGHT.into(),
            Interaction::Hovered => colors::HOVERED_BUTTON_LIGHT.into(),
            Interaction::None => colors::PRIMARY_COLOR.into(),
        }
    }
}

fn spawn_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    menu_button_action: MenuButtonAction,
) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: colors::PRIMARY_COLOR.into(),
                ..Default::default()
            },
            MenuButton,
            menu_button_action,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 25.0,
                    color: Color::rgb(1.0, 1.0, 1.0).into(),
                    ..default()
                },
            ));
        });
}

fn spawn_label(builder: &mut ChildBuilder, font: Handle<Font>, text: &str, value: String) {
    builder.spawn(TextBundle::from_section(
        format!("{}: {}", text, value),
        TextStyle {
            font_size: 20.0,
            color: colors::PRIMARY_COLOR,
            ..default()
        },
    ));
}

fn spawn_spacer(builder: &mut ChildBuilder) {
    builder.spawn(TextBundle::from_section(
        format!(" "),
        TextStyle {
            font_size: 20.0,
            ..default()
        },
    ));
}
#[derive(Component)]
pub struct SessionsBoard;

fn setup_sessionboard(mut commands: Commands, asset_server: Res<AssetServer>, pkv: Res<PkvStore>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    left: Val::Percent(75.0),
                    width: Val::Px(280.0),
                    height: Val::Px(400.0),
                    margin: UiRect::all(Val::Px(40.0)),
                    padding: UiRect::top(Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
            MinWindowWidth,
            SessionsBoard,
        ))
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_section(
                    "Last 10 Sessions:",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: colors::PRIMARY_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                }),
            );

            let font = asset_server.load("fonts/FiraSans-Regular.ttf");
            let recent_sessions = pkv.get::<RecentSessions>("recentSessions").unwrap();
            for session in recent_sessions.sessions.iter().rev() {
                spawn_label(
                    builder,
                    font.clone(),
                    &session.date.to_string(),
                    format!("{}%", session.percent_score.to_string()),
                );
            }
        });
}

#[derive(Component)]
pub struct Scoreboard;

fn setup_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<StatValues>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(280.0),
                    height: Val::Px(400.0),
                    margin: UiRect::all(Val::Px(40.0)),
                    padding: UiRect::top(Val::Px(10.0)),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(1.5)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                border_color: colors::PRIMARY_COLOR.into(),
                ..default()
            },
            OnMenuScreen,
            Scoreboard,
            MinWindowWidth,
        ))
        .with_children(|builder| {
            let font = asset_server.load("fonts/FiraSans-Regular.ttf");
            builder.spawn(
                TextBundle::from_section(
                    "Scoreboard:",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: colors::PRIMARY_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                }),
            );

            spawn_label(
                builder,
                font.clone(),
                "Current Level",
                stats.current_level.to_string(),
            );
            spawn_label(
                builder,
                font.clone(),
                "Average Level Today",
                stats.average_level_today.to_string(),
            );
            spawn_label(
                builder,
                font.clone(),
                "Sessions Today",
                stats.sessions_today.to_string(),
            );

            spawn_spacer(builder);

            spawn_label(
                builder,
                font.clone(),
                "Total Sessions",
                stats.total_sessions.to_string(),
            );
        });
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|builder| {
            let menu_font = asset_server.load("fonts/FiraSans-Bold.ttf");
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(
                        TextBundle::from_section(
                            "Dual-N Back Menu",
                            TextStyle {
                                font: menu_font.clone(),
                                font_size: 45.0,
                                color: colors::TITLE_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    spawn_menu_button(builder, menu_font.clone(), "Start", MenuButtonAction::Start);

                    spawn_menu_button(
                        builder,
                        menu_font.clone(),
                        "Settings",
                        MenuButtonAction::Settings,
                    );
                    spawn_menu_button(
                        builder,
                        menu_font.clone(),
                        "Progress",
                        MenuButtonAction::Progress,
                    );
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<MenuButton>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Start => {
                    menu_state.set(MenuState::Disabled);
                    app_state.set(AppState::Session);
                }
                MenuButtonAction::Settings => {
                    app_state.set(AppState::Settings);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Progress => {
                    app_state.set(AppState::Progress);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}
