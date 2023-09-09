use bevy::prelude::App;
use breakout::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
