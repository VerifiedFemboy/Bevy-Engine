use std::f32::consts::FRAC_PI_2;

use bevy::{app::{App, PluginGroup, Startup, Update}, asset::Assets, color::{Color, LinearRgba}, 
    core_pipeline::{bloom::Bloom, core_3d::Camera3d, tonemapping::Tonemapping}, ecs::{component::Component, entity::Entity, query::With, 
        schedule::IntoSystemConfigs, system::{Commands, Query, Res, ResMut, Single}}, 
    hierarchy::DespawnRecursiveExt, input::{keyboard::KeyCode, mouse::AccumulatedMouseMotion, ButtonInput}, 
    math::{primitives::Sphere, EulerRot, Quat, Vec2, Vec3}, pbr::{MeshMaterial3d, StandardMaterial}, 
    render::{camera::{Camera, ClearColor}, mesh::{Mesh, Mesh3d}}, text::TextFont, transform::components::Transform, 
    ui::{widget::Text, AlignItems, JustifyContent, Node, UiRect, Val}, 
    window::{CursorGrabMode, MonitorSelection, PrimaryWindow, Window, WindowMode, WindowPlugin}, DefaultPlugins};

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins::set(DefaultPlugins, 
    WindowPlugin {
        primary_window: Some(Window {
            title: "Solar Ecliptic".to_string(),
            mode: WindowMode::SizedFullscreen(MonitorSelection::Primary),
            ..Default::default()
        }),
        ..Default::default()
    }))
    .add_systems(Startup, (spawn_camera, spawn_star, spawn_planets, spawn_hud).chain())
    .add_systems(Update, (lock_cursor, update_hud, rotate_camera, input_keys).chain())
    .run();
}

#[derive(Component, Clone)]
struct CelestialBody {
    body: CelestialBodyType,
    position: Vec3, 
    velocity: Vec3,
    acceleration: Vec3,
    color: Option<LinearRgba>,
}

#[derive(Clone)]
enum CelestialBodyType {
    Star(String),
    Planet(String),
}

fn spawn_star(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere { radius: 5.0})),
        MeshMaterial3d(materials.add(StandardMaterial {
            emissive: LinearRgba::new(1.0, 1.0, 0.0, 1.0),
            ..Default::default()
        })),
        CelestialBody {
            body: CelestialBodyType::Star("Sun".to_string()),
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: None,
        },
        Transform::default(),
    ));
}

fn spawn_planets(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let planets = vec![
        CelestialBody {
            body: CelestialBodyType::Planet("Mercury".to_string()),
            position: Vec3::new(100., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 0.7)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Venus".to_string()),
            position: Vec3::new(200., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.1, 0.1, 0.5, 0.7)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Earth".to_string()),
            position: Vec3::new(300., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.1, 0.2, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Mars".to_string()),
            position: Vec3::new(400., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.3, 0.0, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Jupiter".to_string()),
            position: Vec3::new(500., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Saturn".to_string()),
            position: Vec3::new(600., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Uranus".to_string()),
            position: Vec3::new(700., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Neptune".to_string()),
            position: Vec3::new(800., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
    ];

    for planet in planets {
        let planet_clone = planet.clone();
        commands.spawn((
            Mesh3d(meshes.add(Sphere { radius: 2.0 })),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: planet.color.unwrap(),
                ..Default::default()
            })),
            planet,
            Transform::default().with_translation(planet_clone.position),
        ));
    }
}

#[derive(Component)]
struct CameraPlayer {
    paused: bool,
}

impl Default for CameraPlayer {
    fn default() -> Self {
        Self { paused: false }
    }
}

fn spawn_camera(mut commands: Commands) {
    let bloom = Bloom {
        intensity: 0.25,
        ..Default::default()
    };

    commands.spawn((Camera3d::default(),
    Camera {
        hdr: true,
        ..Default::default()
    },
    CameraPlayer::default(),
    Tonemapping::TonyMcMapface,
    Transform::default().with_translation(Vec3::new(50., 0., 0.))
    .looking_at(Vec3::ZERO, Vec3::Y),
    bloom));
}

fn lock_cursor(
    window_query: Single<&mut Window, With<PrimaryWindow>>,
    camera_query: Single<&CameraPlayer>
) {
    let mut window = window_query;
    
    if camera_query.paused {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    } else {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

fn rotate_camera(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &CameraPlayer), With<Camera3d>>
) {
    let camera_player = query.single_mut().1;
    
    if camera_player.paused {
        return;
    }
    
    let mut camera_transform = query.single_mut().0;

    let delta = mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * 0.002;
        let delta_pitch = -delta.y * 0.002;

        let (yaw, pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        let yaw = yaw + delta_yaw;
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn input_keys(
    keycode: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut CameraPlayer), With<Camera3d>>,
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if !pause_menu_query.is_empty() {
        if keycode.just_pressed(KeyCode::Escape) {
            let entity = pause_menu_query.single();
            commands.entity(entity).despawn_recursive();
            camera_query.single_mut().1.paused = false;
        }
    } else {
        if keycode.just_pressed(KeyCode::Escape) {
            spawn_pause_menu(commands);
            camera_query.single_mut().1.paused = true;
        }
    }
    
    // if is paused, then lock camera movement and rotation
    let camera_player = camera_query.single().1;
    if camera_player.paused {
        return;
    }
    
    // direction by rotation
    let mut camera_transform = camera_query.single_mut().0;
    let forward = camera_transform.rotation.mul_vec3(Vec3::Z);
    let right = camera_transform.rotation.mul_vec3(Vec3::X);
    
    let speed = speed_up_by_lshift(&keycode);
    
    if keycode.pressed(KeyCode::KeyW) {
        camera_transform.translation -= forward * speed;
    }
    if keycode.pressed(KeyCode::KeyS) {
        camera_transform.translation += forward * speed;
    }
    if keycode.pressed(KeyCode::KeyA) {
        camera_transform.translation -= right * speed;
    }
    if keycode.pressed(KeyCode::KeyD) {
        camera_transform.translation += right * speed;
    }
}

#[derive(Component)]
struct DistanceFromSunText;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((Text("Distance from Sun: ".to_string()), 
    Node {
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Start,
        flex_grow: 1.,
        margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
        ..Default::default()
    },
    DistanceFromSunText));
}

fn update_hud(
    mut query: Query<&mut Text, With<DistanceFromSunText>>,
    celestial_bodies: Query<&CelestialBody>,
    camera: Query<&Transform, With<Camera3d>>,
) {
    let camera_transform = camera.single();
    let camera_position = camera_transform.translation;

    let mut text = query.single_mut();
    for body in celestial_bodies.iter() {
        if let CelestialBodyType::Star(name) = &body.body {
            if name == "Sun" {
                let distance = (camera_position - body.position).length();
                text.0 = format!("Distance from Sun: {:.2}", distance); // TODO: Add units to distance
            }
        }
    }
}

fn speed_up_by_lshift(keycode: &Res<ButtonInput<KeyCode>>) -> f32 {
    if keycode.pressed(KeyCode::ShiftLeft) {
        return 0.5;
    } else {
        return 0.1;
    }
}

#[derive(Component)]
struct PauseMenu;

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        Text("PAUSED".to_string()),
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_grow: 1.,
            margin: UiRect::all(Val::Px(15.)),
            ..Default::default()
        },
        TextFont {
            font_size: 24.0,
            ..Default::default()
        },
        PauseMenu,
    ));
}
