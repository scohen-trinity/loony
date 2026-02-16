use bevy::mesh::Meshable;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::*;

/// Struct for the player boat object
#[derive(Component)]
struct PlayerBoat;

/// Struct for the velocity of the boat
#[derive(Component)]
struct Velocity3D {
    pub lin: Vec3,    // forward speed
    pub ang_yaw: f32, // y
}

/// Struct for the controls of the boat
#[derive(Component)]
struct BoatControl {
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_speed: f32,
    pub drag: f32,
}

/// Struct for the current state of the boat
#[derive(Component)]
struct BoatState {
    pub current_speed: f32,
    pub target_speed: f32,
    pub is_rowing: bool,
}

/// Method for spawning the boat
fn spawn_player_boat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO - upgrade this to something besides a square
    let boat_mesh: Handle<Mesh> = meshes.add(Cuboid::new(2.0, 0.5, 4.0).mesh().build());
    let boat_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
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
    mut query: Query<(&PlayerBoat, &mut BoatState, &mut Velocity3D)>,
) {
    for (_, mut state, mut vel) in &mut query {
        state.is_rowing = false;
        state.target_speed = 0.0;

        let mut target: f32 = 0.0;

        if keyboard.pressed(KeyCode::KeyW) {
            target += 1.0;
            state.is_rowing = true;
        }

        if keyboard.pressed(KeyCode::KeyS) {
            target -= 1.0;
            state.is_rowing = true;
        }

        let mut yaw_input: f32 = 0.0;

        if keyboard.pressed(KeyCode::KeyA) {
            yaw_input -= 1.0;
        }

        if keyboard.pressed(KeyCode::KeyD) {
            yaw_input += 1.0;
        }

        vel.ang_yaw = yaw_input;

        state.target_speed = target * 1.0;
    }
}

/// Method responsible for moving the boat on the screen
fn boat_movement_system(
    time: Res<Time>,
    mut query: Query<(
        &PlayerBoat,
        &mut Transform,
        &mut Velocity3D,
        &BoatControl,
        &mut BoatState,
    )>,
) {
    for (_, mut transform, mut vel, control, mut state) in &mut query {
        let dt: f32 = time.delta_secs();

        // Apply yaw
        let yaw_speed: f32 = vel.ang_yaw * control.turn_speed;
        let delta_yaw: f32 = yaw_speed * dt;

        transform.rotate_y(delta_yaw);

        // Interpret the target speed as a fraction of max_speed
        let desired_speed: f32 = state.target_speed * control.max_speed;
        let speed_diff: f32 = desired_speed - state.current_speed;

        if state.is_rowing {
            // Accelerate towards the desired speed
            let max_delta: f32 = control.acceleration * dt;
            let delta: f32 = speed_diff.clamp(-max_delta, max_delta);
            state.current_speed += delta;
        } else {
            // Apply drag to smoothly reduce speed when not rowing
            let drag_factor: f32 = (1.0 - control.drag * dt).max(0.0);
            state.current_speed *= drag_factor;
        }

        // Clamp the actual speed to ensure that it does not exceed max_speed
        state.current_speed = state
            .current_speed
            .clamp(-control.max_speed, control.max_speed);

        // Convert current_speed into a velocity vector
        let forward: Dir3 = transform.forward();
        vel.lin = forward * state.current_speed;

        // Integrate velocity into position
        transform.translation += vel.lin * dt;
    }
}

pub struct PlayerBoatPlugin;

impl Plugin for PlayerBoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_boat);
        app.add_systems(Update, (boat_input_system, boat_movement_system));
    }
}
