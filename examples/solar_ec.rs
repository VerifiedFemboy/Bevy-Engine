use bevy::{app::{App, PluginGroup, Startup}, asset::Assets, color::{Color, LinearRgba}, core_pipeline::{bloom::Bloom, core_3d::Camera3d, tonemapping::Tonemapping}, ecs::{component::Component, schedule::IntoSystemConfigs, system::{Commands, ResMut}}, math::{primitives::Sphere, Vec3}, 
    pbr::{MeshMaterial3d, StandardMaterial}, render::{camera::{Camera, ClearColor}, mesh::{Mesh, Mesh3d}}, 
    transform::components::Transform, window::{Window, WindowPlugin}, DefaultPlugins};

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
    .add_systems(Startup, (spawn_camera, spawn_star, spawn_planets).chain())
    .run();
}

#[derive(Component)]
struct CelestialBody {
    body: CelestialBodyType,
    position: Vec3, 
    velocity: Vec3,
    acceleration: Vec3,
    color: Option<LinearRgba>,
}

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

fn spawn_planets(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let planets = vec![
        CelestialBody {
            body: CelestialBodyType::Planet("Mercury".to_string()),
            position: Vec3::new(10., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Venus".to_string()),
            position: Vec3::new(20., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Earth".to_string()),
            position: Vec3::new(30., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Mars".to_string()),
            position: Vec3::new(40., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Jupiter".to_string()),
            position: Vec3::new(50., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Saturn".to_string()),
            position: Vec3::new(60., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Uranus".to_string()),
            position: Vec3::new(70., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
        CelestialBody {
            body: CelestialBodyType::Planet("Neptune".to_string()),
            position: Vec3::new(80., 0., 0.),
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Some(LinearRgba::new(0.5, 0.5, 0.5, 1.0)),
        },
    ];

    for planet in planets {
        commands.spawn((
            Mesh3d(meshes.add(Sphere { radius: 2.0 })),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: planet.color.unwrap(),
                ..Default::default()
            })),
            planet,
            Transform::default(),
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
