use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{colors, despawn_screen, AppState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_state::<MenuState>()
            .add_systems(Update, menu_action)
            .add_systems(Update, menu_button_system.run_if(in_state(AppState::Menu)))
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
pub struct OnMenuScreen;

#[derive(Component)]
pub struct OnSettingsScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    Start,
    Settings,
    Progress,
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

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
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
