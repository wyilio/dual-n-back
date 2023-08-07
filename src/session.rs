use crate::{despawn_screen, AppState};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Session), setup_grid)
            .add_systems(OnExit(AppState::Session), despawn_screen::<OnSessionScreen>);
    }
}

pub const GRID_THICKNESS: f32 = 2.0;
pub const GRID_LENGTH: f32 = 500.0;
pub const GRID_COLOR: Color = Color::rgb(0.773, 0.773, 0.773);
pub const CELL_SIZE: f32 = 75.0;

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
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: GRID_COLOR,
                custom_size: Some(Vec2::new(GRID_THICKNESS, GRID_LENGTH)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(CELL_SIZE, 0., 0.)),
            ..default()
        },
        OnSessionScreen,
    ));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: GRID_COLOR,
            custom_size: Some(Vec2::new(GRID_THICKNESS, GRID_LENGTH)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-1.0 * CELL_SIZE, 0., 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: GRID_COLOR,
            custom_size: Some(Vec2::new(GRID_LENGTH, GRID_THICKNESS)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -1.0 * CELL_SIZE, 0.)),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: GRID_COLOR,
            custom_size: Some(Vec2::new(GRID_LENGTH, GRID_THICKNESS)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, CELL_SIZE, 0.)),
        ..default()
    });
}
