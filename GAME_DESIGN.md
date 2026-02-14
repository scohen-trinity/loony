# Walden Loon Game – Design Blueprint

## High-level concept

A small 3D scene on Walden Pond where the player, in a **boat**, quietly maneuvers to approach a **loon** without startling it. The focus is on atmosphere, simple controls, and emergent behavior rather than combat or complex simulation.

Core tech: Rust + Bevy (3D, ECS, simple custom movement/“fake” physics).[1][2][3]

***

## Core entities and components

### PlayerBoat entity

Represents Thoreau in his boat as a single controllable object.

**Components**

- `PlayerBoat` – marker component.
- `Transform` – position and orientation on the pond; constrained to water surface (fixed Y).[3]
- `Velocity3D { lin: Vec3, ang: f32 }` – forward speed and yaw turn speed.
- `BoatControl`  
  - `max_speed: f32`  
  - `acceleration: f32`  
  - `turn_speed: f32`  
  - `drag: f32`
- `BoatState`  
  - `current_speed: f32`  
  - `target_speed: f32`  
  - `is_rowing: bool`
- `BuoyancyTag` (optional later) – indicates that this object should bob slightly with waves.
- `Visibility` / `Handle<StandardMaterial>` / `Handle<Mesh>` – for rendering.[3]

**Responsibilities**

- Respond to player input (gentle acceleration/turning, no strafing).
- Stay constrained to pond bounds (no going on land).
- Provide a “soft” feeling of inertia (slow start/stop, turning radius).

***

### Loon entity

The loon swims on the pond, dives, and surfaces at new locations, reacting to the boat’s presence.

**Components**

- `Loon` – marker.
- `Transform` – position on the water surface.
- `LoonState`  
  - `is_diving: bool`  
  - `time_in_state: f32`  
  - `target_position: Vec3`
- `LoonBehavior`  
  - `comfort_radius: f32` (distance where loon starts to get wary)  
  - `panic_radius: f32` (too close → dive)  
  - `cruise_speed: f32`  
  - `escape_speed: f32`
- `Velocity3D` – for motion over the water.
- `BuoyancyTag` – for subtle bobbing.
- `AnimationHandle` / `CurrentAnim` (future) – for idle, swim, dive.

**Responsibilities**

- Idle swim within a region of the pond.
- React to distance from boat:
  - Far: idle, relaxed.
  - Near comfort radius: move away slowly.
  - Inside panic radius: dive and resurface far from the boat.
- Occasionally call, creating an audio cue.

***

### Pond and environment

The pond is both visual and logical (bounds, water level).

**Entities**

- `PondSurface` – large plane mesh with water material.[3]
- `PondBounds` – invisible shape used by logic to clamp positions.
- `Shoreline` – static meshes/heightfield; maybe collisions to keep boat inside.

**Components**

- `Pond` – marker for the main water surface.
- `WaterProperties` (resource or component on `Pond`)  
  - `level_y: f32`  
  - `friction: f32` (extra drag for things touching water)  
  - `wave_strength: f32` (for bobbing effect)
- `PondBounds` data (resource)  
  - `min_x: f32, max_x: f32`  
  - `min_z: f32, max_z: f32`
- Standard meshes, materials, and lights for terrain, trees, sky.[2][1]

**Responsibilities**

- Define navigable area for boat and loon.
- Provide the visual “stage” (water, shore, trees, skybox).
- Optionally provide data for future wave/buoyancy effects.

***

### Camera and lighting

**Entities**

- `GameCamera` – a 3D camera following the boat in third-person (or offset top‑down).[4][5]
- `SunLight` – directional light approximating an outdoor scene.[1][2]

**Components**

- `Camera3d` and `Transform` – view of the scene.[6]
- `FollowBoat { offset: Vec3, stiffness: f32 }` – smooth following.  
- Directional `Light` with shadows for time-of-day feel.[2][1]

**Responsibilities**

- Keep the boat in frame, slightly offset to see the loon and surroundings.
- Provide a natural outdoor lighting setup with soft shadows.

***

### Optional atmospheric entities

- `AmbientSound` (resource or component) – loop of gentle wind/water.
- `LoonCall` – audio triggered by Loon behavior.
- `FogSettings` / environment settings – light mist over water (when supported).

***

## Systems overview

### Input and movement systems

**`boat_input_system`**

- Reads keyboard/gamepad (e.g., W/S to accelerate, A/D to turn).[7]
- Updates `BoatState.target_speed` and desired turning direction.
- Applies smoothing so speed changes feel gradual.

**`boat_movement_system`**

- For entities with (`PlayerBoat`, `Transform`, `Velocity3D`, `BoatState`):
  - Integrates acceleration/drag into `BoatState.current_speed`.
  - Computes forward direction from `Transform`.
  - Updates `Transform.translation` and yaw.
  - Clamps position to `PondBounds`.  

**`loon_behavior_system`**

- For (`Loon`, `Transform`, `LoonState`, `LoonBehavior`, `Velocity3D`), with read access to `PlayerBoat` transform:
  - Compute distance to boat.
  - Choose state: idle, wary, escape, dive.
  - Set `Velocity3D.lin` and `LoonState.target_position` accordingly.
  - Handle timers for dive duration and resurfacing.

**`bobbing_system` (optional)**

- For entities with `BuoyancyTag`:
  - Slightly oscillate Y position around `WaterProperties.level_y` using time‑based sine wave.
  - Creates subtle floating without full physics simulation.

***

### Camera and presentation systems

**`camera_follow_system`**

- For (`GameCamera`, `Transform`, `FollowBoat`), read `PlayerBoat` transform:
  - Desired camera position = boat position + offset rotated into boat’s frame.
  - Interpolate towards desired position with `stiffness`.
  - Look at the boat.

**`time_of_day_system` (later)**

- Gradually rotate `SunLight` or tweak ambient color over time to hint at passing time.[1]

***

### Game state and progression

Use Bevy `State` or `AppState` to gate logic.

**States**

- `AppState::Loading` – loading meshes, textures, audio.
- `AppState::Intro` – short fade‑in or text.
- `AppState::Playing` – main loop.
- `AppState::Paused` – optional.
- `AppState::Reflection` – after some condition, fade to a quiet end scene.

**Systems**

- `setup_scene_system` (startup in `Loading` → `Intro`):
  - Spawn pond, shore, boat, loon, camera, light.[3]
- `transition_to_playing_system` – e.g., after key press or text timeout.
- `win_condition_system` (optional):
  - If player reaches within some “quiet” distance of the loon without spooking it for N seconds, trigger reflection state.
- `ui_hint_system` (optional):
  - Minimal on‑screen hint: “Row gently; the loon will dive if you rush.”

***

## Data modeling and ECS guidelines

### Components vs resources

- Use **components** for properties tied to specific entities: movement, behavior, state flags.
- Use **resources** for:
  - Global settings: `PondBounds`, `WaterProperties`, input config.
  - Game‑wide state: `AppState` (Bevy state), time of day, audio settings.[8][6]

### Composition over inheritance

- Avoid big “Boat” or “Loon” structs with lots of fields embedded in one component.
- Prefer small, reusable components:
  - `Velocity3D`, `BuoyancyTag`, `Steerable`, `FleeBehavior`.
- This keeps systems generic and testable.

### System design

- Keep systems focused:
  - Input → “intent” (desired speed/turn).
  - Movement → apply velocity/drag.
  - AI → decide target positions and velocities.
  - Presentation → camera, bobbing, animation, audio.
- Use system ordering only where necessary (e.g., input before movement).

***

## Initial file/module layout

You can start single‑file and then move to something like:

- `src/main.rs`
  - `App` setup, add plugins, states, startup system.
- `src/scene.rs`
  - Spawning pond, shore, camera, light.
- `src/boat.rs`
  - `PlayerBoat` components, input and movement systems.
- `src/loon.rs`
  - `Loon` components, behavior systems.
- `src/environment.rs`
  - `Pond`, `PondBounds`, `WaterProperties`, bobbing system.
- `src/state.rs`
  - `AppState` definition, simple transitions, intro/reflection logic.

As the project grows, turn each module into a plugin (e.g., `BoatPlugin`, `LoonPlugin`) and register them in `main`.[9][10]

***

## Implementation priorities (MVP)

1. Flat pond plane, basic boat mesh, simple loon mesh, directional light, camera.[1][3]
2. Boat controls on flat water: accelerate, turn, clamp to bounds.[7]
3. Loon that idles and flees away from the boat based on distance.  
4. Third‑person camera follow.[5]
5. Simple bobbing and ambient audio for atmosphere.
