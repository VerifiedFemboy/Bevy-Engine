use bevy::prelude::*;
use bevy_console::{ConsolePlugin, PrintConsoleLine};

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, ConsolePlugin))
    .add_systems(Startup, (setup_player, spawn_plate).chain())
    .add_systems(Update, (move_player, rotate_player).chain())
    .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct PlayerCamera;

fn setup_player(mut commands: Commands) {
    commands.spawn((Camera3d::default(), PlayerCamera, Player {speed: 1.0}, Transform::default().with_translation(Vec3::new(0., 5., 0.))));
}

fn spawn_plate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>> 
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5000., 5000.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// By rotation the camera, we can move the player in the direction of the camera
fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    mut console_line: EventWriter<PrintConsoleLine>
) {
    for mut query in query.iter_mut() {
        if input.pressed(KeyCode::KeyW) {
            query.0.translation.z -= 1. * query.1.speed;
            console_line.send(PrintConsoleLine::new(format!("Player position: {:?}", query.0.translation)));
        }
        if input.pressed(KeyCode::KeyS) {
            query.0.translation.z += 1. * query.1.speed;
            console_line.send(PrintConsoleLine::new(format!("Player position: {:?}", query.0.translation)));
        }
        if input.pressed(KeyCode::KeyA) {
            query.0.translation.x -= 1. * query.1.speed;
            console_line.send(PrintConsoleLine::new(format!("Player position: {:?}", query.0.translation)));
        }
        if input.pressed(KeyCode::KeyD) {
            query.0.translation.x += 1. * query.1.speed;
            console_line.send(PrintConsoleLine::new(format!("Player position: {:?}", query.0.translation)));
        }
        if input.pressed(KeyCode::ShiftLeft) {
            query.1.speed = 5.0;
            console_line.send(PrintConsoleLine::new(format!("Player speed: {:?}", query.1.speed)));
        }

        if input.just_released(KeyCode::ShiftLeft) {
            query.1.speed = 1.0;
            console_line.send(PrintConsoleLine::new(format!("Player speed: {:?}", query.1.speed)));
        }
    }
}

fn rotate_player(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
    mut console_line: EventWriter<PrintConsoleLine>
) {
    for mut transform in query.iter_mut() {
        if input.pressed(KeyCode::KeyQ) {
            transform.rotation *= Quat::from_rotation_y(0.1);
            console_line.send(PrintConsoleLine::new(format!("Player rotation: {:?}", transform.rotation)));
        }
        if input.pressed(KeyCode::KeyE) {
            transform.rotation *= Quat::from_rotation_y(-0.1);
            console_line.send(PrintConsoleLine::new(format!("Player rotation: {:?}", transform.rotation)));
        }
    }
}