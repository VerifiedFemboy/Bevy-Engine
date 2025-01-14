use bevy::{app::{App, PluginGroup, Startup}, window::{Window, WindowPlugin}, DefaultPlugins};

fn main() {

    // Uruchamianie gry wraz z domyślnymi pluginami (np. obsługa okna, renderowanie, obsługa wejścia)
    App::new()
        .add_plugins(DefaultPlugins::set(
            DefaultPlugins, WindowPlugin { // Modyfikowanie configuracji pluginu Window (np. tytuł okna)
            primary_window: Some(Window {
                title: "Hello world!".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, helloworld) // Startup to schedule który uruchomi system/funkcję przy starcie programu
        .run();
    

}

fn helloworld() {
   println!("Hello world from HELLOWORLD SYSTEM!!"); 
}