use bevy::{app::{Plugin, Startup, Update}, asset::{AssetServer, Assets}, color::Color, core_pipeline::bloom::Bloom, input::ButtonInput, prelude::{Camera, Camera2d, Commands, Component, IntoSystemConfigs, KeyCode, Mesh, Mesh2d, Query, Rectangle, Res, ResMut, Text, Transform, With}, sprite::{ColorMaterial, MeshMaterial2d}, text::TextFont, ui::{Node, PositionType, Val}};

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, (test_system, setup_camera, setup_mashes).chain());
        app.add_systems(Update, (input_keyboard, update_text).chain());
    }
}

fn test_system() {
    println!("Hello from test_system!");
}

fn setup_camera(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn((Camera2d, Camera {
        hdr: true,
        ..Default::default()
    },
    Bloom::NATURAL));

    commands.spawn((Text::new("Current player speed: "), Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(12.0),
        left: Val::Px(12.0),
        ..Default::default()
    },
    TextFont { // For loading font from assets/fonts/opensans.ttf
        font: asset.load("fonts/opensans.ttf"),
        font_size: 20.0,
        ..Default::default()
    }));
}

fn setup_mashes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));


    // Player cube
    commands.spawn((
        Player {
            speed: 2.,
        },
        Mesh2d(meshes.add(Rectangle::new(50., 50.))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(0., 0., 2.),
    ));
}

#[derive(Component)]
struct Player {
    pub speed: f32,
}

fn input_keyboard(input: Res<ButtonInput<KeyCode>>, mut player_move: Query<&mut Transform, With<Player>>, mut player_speed: Query<&mut Player>) {
    

    for key in input.get_pressed() {
        
        match key {
            KeyCode::KeyW => {
                player_move.iter_mut().for_each(|mut transform| {
                    transform.translation.y += 1.5 * player_speed.single_mut().speed;
                });
            }
            KeyCode::KeyS => {
                player_move.iter_mut().for_each(|mut transform| {
                    transform.translation.y -= 1.5 * player_speed.single_mut().speed;
                });
            }
            KeyCode::KeyA => {
                player_move.iter_mut().for_each(|mut transform| {
                    transform.translation.x -= 1.5 * player_speed.single_mut().speed;
                });
            }
            KeyCode::KeyD => {
                player_move.iter_mut().for_each(|mut transform| {
                    transform.translation.x += 1.5 * player_speed.single_mut().speed;
                });
            }
            KeyCode::ShiftLeft => {
                player_speed.single_mut().speed = 5.;
            }
            _ => {}
        }
    }

    for key in input.get_just_released() {
        match key {
            KeyCode::ShiftLeft => {
                player_speed.single_mut().speed = 2.;
            }
            _ => {}
        }
    }
}

fn update_text(mut text: Query<&mut Text, With<Node>>, player_speed: Query<&Player>) {
    text.iter_mut().for_each(|mut text| {
        text.0 = format!("Current player speed: {}", player_speed.single().speed);
    });
    
}