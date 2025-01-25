use bevy::{app::{App, PluginGroup, Startup, Update}, 
asset::Assets, color::Color, core_pipeline::bloom::Bloom, 
input::{mouse::MouseWheel, ButtonInput}, math::Vec3, 
prelude::{Annulus, BuildChildren, Camera, Camera2d, ChildBuild, Circle, 
    Commands, Component, EventReader, IntoSystemConfigs, KeyCode, Mesh, 
    Mesh2d, OrthographicProjection, Query, Res, ResMut, Transform, With}, 
    sprite::{ColorMaterial, MeshMaterial2d}, text::{Text2d, TextFont}, 
    time::Time, window::{Window, WindowPlugin, WindowResolution}, DefaultPlugins};

//TODO: Sync our real time to the game time

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
        .add_systems(Startup, (spawn_camera, date_spawn_text, background, spawn_objects, spawn_orbits).chain())
        .add_systems(Update, (update_date_text, input_keys, update_zoom_by_scroll, update_planets_position).chain())
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
        Transform::from_xyz(0., 0., 0.),
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
        radius: 6.9634,
        speed: 0.,
        angle: 0.,
    },
        Mesh2d(meshes.add(Circle::new(30.))),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 0.))),
        Transform::from_xyz(0., 0., 1.)
    ));

    let planets = vec![
        (Object {
            name: "Mercury".to_string(),
            kind: ObjectKind::Planet,
            radius: 57.,
            speed: 4.74,
            angle: 0.,
        },
            Mesh2d(meshes.add(Circle::new(1.))),
            MeshMaterial2d(materials.add(Color::srgb(1., 0.5, 0.))),
            Transform::from_xyz(57., 0., 1.)
        ),
        (Object {
            name: "Venus".to_string(),
            kind: ObjectKind::Planet,
            radius: 108.,
            speed: 3.5,
            angle: 0.,
        },
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
            Transform::from_xyz(108., 0., 1.)
        ),
        (Object {
            name: "Earth".to_string(),
            kind: ObjectKind::Planet,
            radius: 149.6,
            speed: 2.98,
            angle: 0.,
        },
            Mesh2d(meshes.add(Circle::new(5.))),
            MeshMaterial2d(materials.add(Color::hsl(211., 0.93, 0.7))),
            Transform::from_xyz(149.6, 0., 1.)
        ),
        (Object {
            name: "Mars".to_string(),
            kind: ObjectKind::Planet,
            radius: 227.9,
            speed: 2.41,
            angle: 0.,
        },
            Mesh2d(meshes.add(Circle::new(3.))),
            MeshMaterial2d(materials.add(Color::srgb(1., 0.3, 0.))),
            Transform::from_xyz(227.9, 0., 1.)
        ),
        (Object {
            name: "Jupiter".to_string(),
            kind: ObjectKind::Planet,
            radius: 778.6,
            speed: 13.07,
            angle: 0.,
        },
            Mesh2d(meshes.add(Circle::new(10.))),
            MeshMaterial2d(materials.add(Color::srgb(0.8, 0.7, 0.2))),
            Transform::from_xyz(778.6, 0., 1.)
        ),
        (Object {
            name: "Saturn".to_string(),
            kind: ObjectKind::Planet,
            radius: 1433.5,
            speed: 9.69,
            angle: 0.,
        },
            Mesh2d(meshes.add(Circle::new(8.))),
            MeshMaterial2d(materials.add(Color::srgb(0.9, 0.8, 0.2))),
            Transform::from_xyz(1433.5, 0., 1.)
        ),
        (Object {
            name: "Uranus".to_string(),
            kind: ObjectKind::Planet,
            radius: 2872.5,
            angle: 0.,
            speed: 6.81,
        },
            Mesh2d(meshes.add(Circle::new(6.))),
            MeshMaterial2d(materials.add(Color::srgb(0.5, 0.8, 0.9))),
            Transform::from_xyz(2872.5, 0., 1.)
        ),
        (Object {
            name: "Neptune".to_string(),
            kind: ObjectKind::Planet,
            radius: 4495.1,
            angle: 0.,
            speed: 5.43,
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
            parent.spawn((Text2d(name), TextFont {font_size: 6., ..Default::default()}, Transform::from_xyz(0., 5., 0.2)));
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
    radius: f32,
    angle: f32,
    speed: f32,
}

enum ObjectKind {
    Planet,
    Star,
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
        position.translation.y += 5.0;
    }
    
    if key.pressed(KeyCode::KeyS) {
        position.translation.y -= 5.0;
    }

    if key.pressed(KeyCode::KeyA) {
        position.translation.x -= 5.0;
    }

    if key.pressed(KeyCode::KeyD) {
        position.translation.x += 5.0;
    }
}

fn update_zoom_by_scroll(
    mut wheel_events: EventReader<MouseWheel>, 
    mut projection_query: Query<&mut OrthographicProjection, With<CameraEnt>>
) {
    for event in wheel_events.read() {
        let mut projection = projection_query.single_mut();
        projection.scale = (projection.scale + event.y * 0.1).max(0.1);
    }
}

fn update_planets_position(
    time: Res<Time>,
    mut query: Query<(&mut Object, &mut Transform), With<Object>>
) {

    for (mut object, mut transform) in query.iter_mut() {
        if matches!(object.kind, ObjectKind::Star) {
            continue;
        }

        object.angle += object.speed * time.delta().as_secs_f32();
        let x = object.angle.cos() * object.radius;
        let y = object.angle.sin() * object.radius;
        transform.translation = Vec3::new(x, y, 1.);
    }
   
}

fn spawn_orbits(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    query_planets: Query<&Transform, With<Object>>
) {
    for planet in query_planets.iter() {
        let planet_position = planet.translation;
        if planet_position.x != 0. {
            let orbit = Annulus::new(planet_position.x - 0.5, planet_position.x + 0.5);

            commands.spawn(
                (Mesh2d(meshes.add(orbit)), 
                MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
                Transform::from_xyz(0.,0.,0.1)
            ));
        }
    }
}

