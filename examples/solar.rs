use bevy::{app::{App, PluginGroup, Startup, Update}, asset::Assets, color::Color, core_pipeline::bloom::Bloom, input::ButtonInput, prelude::{BuildChildren, Camera, Camera2d, ChildBuild, Circle, Commands, Component, IntoSystemConfigs, KeyCode, Mesh, Mesh2d, OrthographicProjection, Query, Res, ResMut, Transform, With}, sprite::{ColorMaterial, MeshMaterial2d}, text::{Text2d, TextFont}, window::{Window, WindowPlugin, WindowResolution}, DefaultPlugins};

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
        .add_systems(Update, (update_date_text, input_keys).chain())
        .run();
}

fn background(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(
        (Transform::default(),
        Mesh2d(meshes.add(Circle::new(5000.))), 
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))))
    );
}

#[derive(Component)]
struct CameraEnt;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera {
        hdr: true,
        ..Default::default()
    }, Bloom::NATURAL,
        Transform::from_xyz(0., 0., 1000.), //TODO: SCALE THE CAMERA TO ZOOM OUT
        CameraEnt
    ));
}

fn spawn_objects(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((Object {
        name: "Sun".to_string(),
        kind: ObjectKind::Star,
        mass: 1.989e30,
        radius: 6.9634e8,
        velocity: (0., 0.),
        acceleration: (0., 0.),
    },
        Mesh2d(meshes.add(Circle::new(30.))),
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
        (Object {
            name: "Saturn".to_string(),
            kind: ObjectKind::Planet,
            mass: 5.68e26,
            radius: 5.8232e7,
            velocity: (0., 9.69e3),
            acceleration: (0., 0.),
        },
            Mesh2d(meshes.add(Circle::new(8.))),
            MeshMaterial2d(materials.add(Color::srgb(0.9, 0.8, 0.2))),
            Transform::from_xyz(1433.5, 0., 1.)
        ),
        (Object {
            name: "Uranus".to_string(),
            kind: ObjectKind::Planet,
            mass: 8.68e25,
            radius: 2.5362e7,
            velocity: (0., 6.81e3),
            acceleration: (0., 0.),
        },
            Mesh2d(meshes.add(Circle::new(6.))),
            MeshMaterial2d(materials.add(Color::srgb(0.5, 0.8, 0.9))),
            Transform::from_xyz(2872.5, 0., 1.)
        ),
        (Object {
            name: "Neptune".to_string(),
            kind: ObjectKind::Planet,
            mass: 1.02e26,
            radius: 2.4622e7,
            velocity: (0., 5.43e3),
            acceleration: (0., 0.),
        },
            Mesh2d(meshes.add(Circle::new(6.))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.3, 0.9))),
            Transform::from_xyz(4495.1, 0., 1.)
        ),
    ];

    for planet in planets {
        let name = planet.0.name.clone();
        commands.spawn(planet
        ).with_children(|parent| {
            parent.spawn((Text2d(name), TextFont {font_size: 6., ..Default::default()}, Transform::from_xyz(0., 5., 0.1)));
        });
    }
}

#[derive(Component)]
struct DateText;

fn date_spawn_text(mut commands: Commands) {
    commands.spawn(
    (
        Text2d(format!("Date: {}\nTime: {}", get_current_date_and_time().0, get_current_date_and_time().1)),
        Transform::from_xyz(-300.0, 200.0, 0.0),
        DateText
    ));
}

fn update_date_text(mut query: Query<&mut Text2d, With<DateText>>) {
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

fn input_keys(
    key: Res<ButtonInput<KeyCode>>, 
    mut projection_query: Query<&mut OrthographicProjection, With<CameraEnt>>,
    mut position_query: Query<&mut Transform, With<CameraEnt>>,
) {
    let mut projection = projection_query.single_mut();
    if key.pressed(KeyCode::ArrowUp) {
        projection.scale *= 0.9;
    }
    if key.pressed(KeyCode::ArrowDown) {
        projection.scale /= 0.9;
    }

    let mut position = position_query.single_mut();

    if key.pressed(KeyCode::KeyW) {
        position.translation.y += 10.0;
    }
    
    if key.pressed(KeyCode::KeyS) {
        position.translation.y -= 10.0;
    }

    if key.pressed(KeyCode::KeyA) {
        position.translation.x -= 10.0;
    }

    if key.pressed(KeyCode::KeyD) {
        position.translation.x += 10.0;
    }
}