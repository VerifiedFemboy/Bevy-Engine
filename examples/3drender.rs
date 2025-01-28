use bevy::prelude::*;
use bevy_console::{ConsolePlugin, PrintConsoleLine};

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, ConsolePlugin))
    .add_systems(Startup, (setup_player, spawn_plate).chain())
    .add_systems(Update, move_player)
    .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCamera;

fn setup_player(mut commands: Commands) {
    commands.spawn((Camera3d::default(), PlayerCamera, Player, Transform::default().with_translation(Vec3::new(0., 5., 0.))));
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

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
    mut console_line: EventWriter<PrintConsoleLine>
) {
    for mut transform in query.iter_mut() {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.z -= 1.;
            console_line.send(PrintConsoleLine::new(format!("Player position: {:?}", transform.translation)));
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.z += 1.;
            console_line.send(PrintConsoleLine::new(format!("Player position: {:?}", transform.translation)));
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 1.;
            console_line.send(PrintConsoleLine::new(format!("Player position: {:?}", transform.translation)));
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 1.;
            console_line.send(PrintConsoleLine::new(format!("Player position: {:?}", transform.translation)));
        }
    }
}