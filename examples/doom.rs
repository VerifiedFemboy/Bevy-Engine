use bevy::{app::{App, PluginGroup, Startup}, prelude::{Camera, Camera3d, Commands, Component}, window::{Window, WindowPlugin}, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins::set(
            DefaultPlugins, WindowPlugin {
            primary_window: Some(Window {
                title: "doom.rs".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Camera {
        hdr: true,
        ..Default::default()
    }));
}

fn spawn_player(mut commands: Commands) {
    
}

#[derive(Component)]
struct Player;