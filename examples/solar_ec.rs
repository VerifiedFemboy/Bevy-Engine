use std::f32::consts::FRAC_PI_2;

use bevy::{app::{App, PluginGroup, Startup, Update}, 
asset::Assets, color::{Color, LinearRgba}, core_pipeline::{bloom::Bloom, core_3d::Camera3d, tonemapping::Tonemapping}, 
ecs::{component::Component, query::With, schedule::IntoSystemConfigs, system::{Commands, Query, Res, ResMut}}, 
input::mouse::AccumulatedMouseMotion, math::{primitives::Sphere, EulerRot, Quat, Vec2, Vec3}, 
pbr::{MeshMaterial3d, StandardMaterial}, render::{camera::{Camera, ClearColor}, mesh::{Mesh, Mesh3d}}, 
transform::components::Transform, ui::{widget::Text, AlignItems, JustifyContent, Node, UiRect, Val}, 
window::{Window, WindowPlugin}, DefaultPlugins};

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins::set(DefaultPlugins, 
    WindowPlugin {
        primary_window: Some(Window {
            title: "Solar Ecliptic".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }))
    .add_systems(Startup, (spawn_camera, spawn_star, spawn_planets, spawn_hud).chain())
    .add_systems(Update, rotate_camera)
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
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Venus".to_string()),
            position: Vec3::new(200., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Earth".to_string()),
            position: Vec3::new(300., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Mars".to_string()),
            position: Vec3::new(400., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
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
    Tonemapping::TonyMcMapface,
    Transform::default().with_translation(Vec3::new(50., 0., 0.))
    .looking_at(Vec3::ZERO, Vec3::Y),
    bloom));
}

//TODO: Lock cursor to center of screen when rotating camera
fn rotate_camera(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<&mut Transform, With<Camera3d>>
) {
    let mut camera_transform = query.single_mut();

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

#[derive(Component)]
struct DirectionText;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((Text("Direction: ".to_string()), 
    Node {
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Start,
        flex_grow: 1.,
        margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
        ..Default::default()
    },
    DirectionText));
}
