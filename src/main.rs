use bevy::{app::App, DefaultPlugins};
use test_plugin::TestPlugin;

mod test_plugin;

fn main() {
    App::new().
    add_plugins(DefaultPlugins)
    .add_plugins(TestPlugin).run();
}
