use bevy::{app::{App, PluginGroup, Startup}, asset::Assets, color::Color, core_pipeline::bloom::Bloom, prelude::{Camera, Camera2d, Circle, Commands, Component, IntoSystemConfigs, Mesh, Mesh2d, ResMut, Transform}, sprite::{ColorMaterial, MeshMaterial2d}, window::{Window, WindowPlugin}, DefaultPlugins};

fn main() {
    App::new().add_plugins(DefaultPlugins::set(DefaultPlugins, 
        WindowPlugin {
            primary_window: Some(Window {
                title: "Solar System".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (spawn_camera, background, spawn_objects).chain())
        .run();
}

fn background(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(
        (Transform::default(),
        Mesh2d(meshes.add(Circle::new(1000.))), 
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))))
    );
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera {
        hdr: true,
        ..Default::default()
    }, Bloom::NATURAL));
}

fn spawn_objects(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((Object {
        name: "Sun".to_string(),
        kind: ObjectKind::Star,
        mass: 1.989e30,
        radius: 6.9634e8,
        velocity: (0., 0.),
        acceleration: (0., 0.),
    },
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 0.))),
        Transform::from_xyz(0., 0., 1.0)
    ));

    let planets = vec![
        (Object {
            name: "Mercury".to_string(),
            kind: ObjectKind::Planet,
            mass: 3.30e23,
            radius: 2.4397e6,
            velocity: (0., 4.787e4),
            acceleration: (0., 0.),
        },
            Mesh2d(meshes.add(Circle::new(1.))),
            MeshMaterial2d(materials.add(Color::srgb(1., 0.5, 0.))),
            Transform::from_xyz(57., 0., 1.)
        ),
        (Object {
            name: "Venus".to_string(),
            kind: ObjectKind::Planet,
            mass: 4.87e24,
            radius: 6.0518e6,
            velocity: (0., 3.502e4),
            acceleration: (0., 0.),
        },
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(materials.add(Color::srgb(1., 0., 0.))),
            Transform::from_xyz(108., 0., 1.)
        ),
        (Object {
            name: "Earth".to_string(),
            kind: ObjectKind::Planet,
            mass: 5.97e24,
            radius: 6.371e6,
            velocity: (0., 2.978e4),
            acceleration: (0., 0.),
        },
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(materials.add(Color::srgb(0., 0., 1.))),
            Transform::from_xyz(149.6, 0., 0.1)
        )
        ];

    for planet in planets {
        commands.spawn(planet);
    }
}

#[derive(Component)]
struct Object {
    name: String,
    kind: ObjectKind,
    mass: f32,
    radius: f32,
    velocity: (f32, f32),
    acceleration: (f32, f32),
}

enum ObjectKind {
    Planet,
    Star,
    Moon,
}