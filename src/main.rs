use bevy::prelude::*;

mod playerboat;
mod camera;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, camera::CameraPlugin, playerboat::PlayerBoatPlugin))
        .run();
}
