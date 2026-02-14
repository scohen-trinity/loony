use bevy::mesh::Meshable;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::*;

/// Struct for the player boat object
#[derive(Component)]
pub struct PlayerBoat;

/// Struct for the velocity of the boat
#[derive(Component)]
pub struct Velocity3D {
    pub lin: Vec3,    // forward speed
    pub ang_yaw: f32, // y
}

/// Struct for the controls of the boat
#[derive(Component)]
pub struct BoatControl {
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_speed: f32,
    pub drag: f32,
}

/// Struct for the current state of the boat
#[derive(Component)]
pub struct BoatState {
    pub current_speed: f32,
    pub target_speed: f32,
    pub is_rowing: bool,
}

/// Method for spawning the boat
pub fn spawn_player_boat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO - upgrade this to something besides a square
    let boat_mesh = meshes.add(Cuboid::new(2.0, 0.5, 4.0).mesh().build());
    let boat_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.4, 0.2),
        ..Default::default()
    });

    commands.spawn((
        PlayerBoat,
        Mesh3d(boat_mesh),
        MeshMaterial3d(boat_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Velocity3D {
            lin: Vec3::ZERO,
            ang_yaw: 0.0,
        },
        BoatControl {
            max_speed: 5.0,
            acceleration: 3.0,
            turn_speed: 1.5,
            drag: 1.0,
        },
        BoatState {
            current_speed: 0.0,
            target_speed: 0.0,
            is_rowing: false,
        },
    ));
}

/// Method responsible for handling the boat's input system
fn boat_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&PlayerBoat, &mut BoatState)>,
) {
    for (_, mut state) in &mut query {
        // TODO - read input, tweak target_speed and is_rowing
    }
}

/// Method responsible for moving the boat on the screen
fn boat_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &PlayerBoat,
        &mut Transform,
        &mut Velocity3D,
        &BoatControl,
        &mut BoatState,
    )>,
) {
    for (_, mut transform, mut vel, control, mut state) in &mut query {
        // TODO - Use the BoatState + BoatControl to update the velocity and transform
    }
}

pub struct PlayerBoatPlugin;

impl Plugin for PlayerBoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_boat);
    }
}
