use bevy::{app::{App, PluginGroup, Startup, Update}, asset::Assets, color::Color, core_pipeline::bloom::Bloom, prelude::{Camera, Camera2d, Circle, Commands, Component, IntoSystemConfigs, Mesh, Mesh2d, Query, ResMut, Transform}, sprite::{ColorMaterial, MeshMaterial2d}, text::Text2d, window::{Window, WindowPlugin, WindowResolution}, DefaultPlugins};

fn main() {
    App::new().add_plugins(DefaultPlugins::set(DefaultPlugins, 
        WindowPlugin {
            primary_window: Some(Window {
                title: "Solar System".to_string(),
                resizable: false,
                resolution: WindowResolution::new(1920., 1080.),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (spawn_camera, date_spawn_text, background, spawn_objects).chain())
        .add_systems(Update, (update_date_text).chain())
        .run();
}

fn background(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(
        (Transform::default(),
        Mesh2d(meshes.add(Circle::new(5000.))), 
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))))
    );
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera {
        hdr: true,
        ..Default::default()
    }, Bloom::NATURAL,
        Transform::from_xyz(0., 0., 1000.) //TODO: SCALE THE CAMERA TO ZOOM OUT
    ));
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
            MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
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
            MeshMaterial2d(materials.add(Color::hsl(211., 0.93, 0.7))),
            Transform::from_xyz(149.6, 0., 0.1)
        ),
        (Object {
            name: "Mars".to_string(),
            kind: ObjectKind::Planet,
            mass: 6.42e23,
            radius: 3.3895e6,
            velocity: (0., 2.4077e4),
            acceleration: (0., 0.),
        },
            Mesh2d(meshes.add(Circle::new(3.))),
            MeshMaterial2d(materials.add(Color::srgb(1., 0.3, 0.))),
            Transform::from_xyz(227.9, 0., 1.)
        ),
        (Object {
            name: "Jupiter".to_string(),
            kind: ObjectKind::Planet,
            mass: 1.90e27,
            radius: 6.9911e7,
            velocity: (0., 1.307e4),
            acceleration: (0., 0.),
        },
            Mesh2d(meshes.add(Circle::new(10.))),
            MeshMaterial2d(materials.add(Color::srgb(0.8, 0.7, 0.2))),
            Transform::from_xyz(778.6, 0., 1.)
        ),
    ];

    for planet in planets {
        commands.spawn(planet);
    }
}

fn date_spawn_text(mut commands: Commands) {
    commands.spawn(
    (
        Text2d(format!("Date: {}\nTime: {}", get_current_date_and_time().0, get_current_date_and_time().1)),
        Transform::from_xyz(-300.0, 200.0, 0.0),
    ));
}

fn update_date_text(mut query: Query<&mut Text2d>) {
    for mut text in query.iter_mut() {
        text.0 = format!("Date: {}\nTime: {}", get_current_date_and_time().0, get_current_date_and_time().1);
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

fn get_current_date_and_time() -> (String, String) {
    let date = chrono::Utc::now().format("%m-%d-%Y").to_string();
    let time = chrono::Utc::now().format("%I:%M:%S %p").to_string();
    (date, time)
}