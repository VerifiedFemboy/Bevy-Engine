use bevy::{app::{App, PluginGroup}, window::{Window, WindowPlugin}, DefaultPlugins};
use camera::CameraPlugin;
use player::PlayerPlugin;

fn main() {
    App::new().add_plugins(DefaultPlugins::set(DefaultPlugins, 
        WindowPlugin {
            primary_window: Some(Window {
                title: "Shooter 2D".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((CameraPlugin, PlayerPlugin))
        .run();
}

mod camera {
    use bevy::prelude::*;

    pub struct CameraPlugin;

    impl Plugin for CameraPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup);
        }
    }

    fn setup(mut commands: Commands) {
        commands.spawn(Camera2d);
    }
}

mod player {
    use bevy::prelude::*;

    use crate::bullet::Bullet;

    #[derive(Component)]
    pub struct Player;

    pub struct PlayerPlugin;

    impl Plugin for PlayerPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup);
            app.add_systems(Update, (movement_input, mouse_input));
        }
    }

    fn setup(
        mut commands: Commands, 
        mut meshes: ResMut<Assets<Mesh>>, 
        mut materials: ResMut<Assets<ColorMaterial>>
    ) {
        commands.spawn((Player, Transform::default(), 
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(Color::WHITE))));
    }

    fn movement_input(
        input: Res<ButtonInput<KeyCode>>, 
        mut player_move: Query<&mut Transform, With<Player>>
    ) {
        for mut transform in player_move.iter_mut() {
            if input.pressed(KeyCode::KeyW) {
                transform.translation.y += 2.;
            }
            if input.pressed(KeyCode::KeyS) {
                transform.translation.y -= 2.;
            }
            if input.pressed(KeyCode::KeyA) {
                transform.translation.x -= 2.;
            }
            if input.pressed(KeyCode::KeyD) {
                transform.translation.x += 2.;
            }
        }
    }

    fn mouse_input(
        input: Res<ButtonInput<MouseButton>>, 
        mut commands: Commands, player_position: Query<&Transform, With<Player>>, 
        mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>
    ) {
        if input.just_pressed(MouseButton::Left) {
            commands.spawn(
            (
                Bullet,
                Transform::from_translation(player_position.iter().next().unwrap().translation),
                Mesh2d(meshes.add(Circle::new(5.))),
                MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 1.))),
            ));
        }
    }
}

mod bullet {
    use bevy::prelude::Component;


    #[derive(Component)]
    pub struct Bullet;

    
}