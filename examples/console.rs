use bevy::{app::{App, Startup}, ecs::{event::EventWriter, schedule::IntoSystemConfigs}, DefaultPlugins};
use bevy_console::{ConsolePlugin, ConsoleSet, PrintConsoleLine};

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, ConsolePlugin))
    .add_systems(Startup, write_to_console.after(ConsoleSet::ConsoleUI))
        .run();
}

fn write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
    console_line.send(PrintConsoleLine::new("Hello World".into()));
}