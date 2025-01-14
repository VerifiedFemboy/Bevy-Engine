use bevy::{app::{App, PluginGroup, Startup, Update}, asset::Assets, color::Color, input::ButtonInput, prelude::{Camera, Camera2d, Circle, Commands, Component, IntoSystemConfigs, KeyCode, Mesh, Mesh2d, Query, Res, ResMut, Transform, With}, sprite::{ColorMaterial, MeshMaterial2d}, window::{Window, WindowPlugin}, DefaultPlugins};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins::set(DefaultPlugins, 
        WindowPlugin {
            primary_window: Some(Window {
                title: "Input Example".to_string(),
                ..Default::default()
            }),
            ..Default::default()
    }))
    .add_systems(Startup, (setup_camera, setup_player).chain())
    .add_systems(Update, input_keyboard)
    .run();
}

// Tworzymy kamere w wymiarze 2D
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera {
        hdr: true,
        ..Default::default()
    }));
}

// Tworzymy ECS Gracza
#[derive(Component)]
struct Player {
    pub speed: f32,
}

fn setup_player(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        Player { speed: 1. },
        Mesh2d(meshes.add(Circle::new(10.))),
        MeshMaterial2d(materials.add(Color::WHITE))
    ));
}

// System aka funkcja pod input z klawiatury      Wysyłamy zapytanie do silnika aby zmodfykować pozycje gracza
fn input_keyboard(
    input: Res<ButtonInput<KeyCode>>, 
    mut player_move: Query<&mut Transform, With<Player>>, 
    mut player_speed: Query<&mut Player>
) {
    for mut transform in player_move.iter_mut() {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 1. * player_speed.single_mut().speed;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 1. * player_speed.single_mut().speed;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 1. * player_speed.single_mut().speed;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 1. * player_speed.single_mut().speed;
        }
    }

    // Zmiana prędkości gracza
    if input.pressed(KeyCode::ShiftLeft) {
        player_speed.single_mut().speed = 5.;
    }

    // Resetowanie prędkości gracza
    if input.just_released(KeyCode::ShiftLeft) {
        player_speed.single_mut().speed = 1.;
    }
}