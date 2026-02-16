use bevy::prelude::*;

fn spawn_camera(mut commands: Commands) {
    // Spawn a light above the scene
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 0.0),
    ));

    // Spawn the camera positioned behind and above the boat, looking at the origin
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 15.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}