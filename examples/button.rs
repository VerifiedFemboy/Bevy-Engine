use bevy::{app::{App, PluginGroup, Startup}, color::Color, prelude::{BuildChildren, Button, Camera2d, ChildBuild, Commands, Text}, text::TextFont, ui::{AlignItems, BackgroundColor, BorderColor, BorderRadius, JustifyContent, Node, UiRect, Val}, window::{Window, WindowPlugin}, DefaultPlugins};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins::set(DefaultPlugins, 
        WindowPlugin {
            primary_window: Some(Window {
                title: "Menu Example".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {

    commands.spawn(Camera2d);
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn((Button, 
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(Color::BLACK)));
    }).with_child((Text::new("Start Game"),
        TextFont {
            font_size: 20.,
            ..Default::default()
        }))
    ;



}