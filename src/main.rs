use bevy::prelude::*;

mod playerboat;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, playerboat::PlayerBoatPlugin))
        .run();
}
