use bevy::{app::{App, Startup}, DefaultPlugins};

fn main() {

    // Uruchamianie gry wraz z domyślnymi pluginami (np. obsługa okna, renderowanie, obsługa wejścia)
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, helloworld) // Startup to schedule który uruchomi system/funkcję przy starcie programu
        .run();

}

fn helloworld() {
   println!("Hello world from HELLOWORLD SYSTEM!!"); 
}