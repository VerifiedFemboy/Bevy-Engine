use bevy::{app::{App, PluginGroup, Startup}, prelude::{Camera, Camera2d, Commands, IntoSystemConfigs, Text}, ui::{Node, PositionType, Val}, window::{Window, WindowPlugin}, DefaultPlugins};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins::set(DefaultPlugins, 
        WindowPlugin {
            primary_window: Some(Window {
                title: "Text Example".to_string(),
                ..Default::default()
            }),
            ..Default::default()
    }))
    .add_systems(Startup, (spawn_camera, spawn_text).chain())
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera {
        hdr: true,
        ..Default::default()
    }));
}

fn spawn_text(mut commands: Commands) {
    
    commands.spawn((Text::new("Hello, Bevy!"), 
    Node {
        position_type: PositionType::Absolute,
        bottom: Val::Px(12.0),
        left: Val::Px(12.0),
        ..Default::default()
    }));
}