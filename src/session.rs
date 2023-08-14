use crate::{despawn_screen, AppState};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

pub struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Session),
            (setup_grid, setup_input_display),
        )
        .add_systems(OnExit(AppState::Session), despawn_screen::<OnSessionScreen>);
    }
}

pub const CELL_SIZE: f32 = 150.0;
pub const CELL_COLOR: Color = Color::rgb(0.08, 0.08, 0.08);
pub const VERTICAL_OFFSET: f32 = 75.0;
pub const GRID_THICKNESS: f32 = 2.0;
pub const GRID_LENGTH: f32 = 3.0 * CELL_SIZE;
pub const GRID_COLOR: Color = Color::rgb(0.773, 0.773, 0.773);

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
            1 => TargetAudio::C,
            2 => TargetAudio::H,
            3 => TargetAudio::K,
            4 => TargetAudio::L,
            5 => TargetAudio::R,
            6 => TargetAudio::S,
            7 => TargetAudio::T,
            _ => unreachable!(),
        }
    }
}

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
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: GRID_COLOR,
                custom_size: Some(Vec2::new(GRID_THICKNESS, GRID_LENGTH)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(CELL_SIZE / 2.0, VERTICAL_OFFSET, 0.)),
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
        transform: Transform::from_translation(Vec3::new(
            -1.0 * CELL_SIZE / 2.0,
            VERTICAL_OFFSET,
            0.,
        )),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: GRID_COLOR,
            custom_size: Some(Vec2::new(GRID_LENGTH, GRID_THICKNESS)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            0.0,
            -1.0 * CELL_SIZE / 2.0 + VERTICAL_OFFSET,
            0.,
        )),
        ..default()
    });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: GRID_COLOR,
            custom_size: Some(Vec2::new(GRID_LENGTH, GRID_THICKNESS)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            0.0,
            CELL_SIZE / 2.0 + VERTICAL_OFFSET,
            0.,
        )),
        ..default()
    });
}

pub fn setup_input_display(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let target_location = TargetLocation::random();
    let target_coordinates = get_target_coordinates(target_location);

    commands.spawn(
        (MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_translation(Vec3::new(
                target_coordinates.0,
                target_coordinates.1 + VERTICAL_OFFSET,
                0.0,
            ))
            .with_scale(Vec3::splat(128.)),
            material: materials.add(ColorMaterial::from(CELL_COLOR)),
            ..default()
        }),
    );
}
