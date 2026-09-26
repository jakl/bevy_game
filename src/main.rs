use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{MonitorSelection, PresentMode, PrimaryWindow, WindowMode},
};
use leafwing_input_manager::prelude::*;

// 1. Abstract Actions
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum SystemAction {
    ToggleFullscreen,
}

// 2. Components
#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct SystemController;

// 3. Setup System
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Wrap the OrthographicProjection data inside the Projection enum component
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
    ));

    let texture_handle = asset_server.load("player.png");

    // -- Player 1: Keyboard Mapping Context --
    let mut kb_map = InputMap::default();
    kb_map.insert(PlayerAction::MoveUp, KeyCode::KeyW);
    kb_map.insert(PlayerAction::MoveDown, KeyCode::KeyS);
    kb_map.insert(PlayerAction::MoveLeft, KeyCode::KeyA);
    kb_map.insert(PlayerAction::MoveRight, KeyCode::KeyD);

    commands.spawn((
        Player { speed: 10.0 },
        kb_map,
        ActionState::<PlayerAction>::default(),
        Sprite::from_image(texture_handle.clone()),
        Transform::from_xyz(-5.0, 0.0, rand::random::<f32>()),
        RigidBody::Dynamic,
        Collider::circle(1.0),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
    ));

    // -- Player 2: Gamepad Mapping Context --
    let mut pad_map = InputMap::default();
    pad_map.insert(PlayerAction::MoveUp, GamepadButton::DPadUp);
    pad_map.insert(PlayerAction::MoveDown, GamepadButton::DPadDown);
    pad_map.insert(PlayerAction::MoveLeft, GamepadButton::DPadLeft);
    pad_map.insert(PlayerAction::MoveRight, GamepadButton::DPadRight);

    commands.spawn((
        Player { speed: 10.0 },
        pad_map,
        ActionState::<PlayerAction>::default(),
        Sprite::from_image(texture_handle),
        Transform::from_xyz(5.0, 0.0, rand::random::<f32>()),
        RigidBody::Dynamic,
        Collider::circle(1.0),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
    ));

    // -- System Mapping Context --
    let mut sys_map = InputMap::default();
    sys_map.insert(SystemAction::ToggleFullscreen, KeyCode::F11);

    commands.spawn((
        SystemController,
        sys_map,
        ActionState::<SystemAction>::default(),
    ));
}

// 4. Game Logic Systems
fn toggle_fullscreen(
    query: Query<&ActionState<SystemAction>, With<SystemController>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(action_state) = query.single() else { return; };

    if action_state.just_pressed(&SystemAction::ToggleFullscreen) {
        let Ok(mut window) = window_query.single_mut() else { return; };
        
        // Explicitly target the primary monitor when going fullscreen
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            _ => WindowMode::Windowed,
        };
    }
}

fn apply_movement(
    mut query: Query<(&Player, &ActionState<PlayerAction>, &mut LinearVelocity)>,
) {
    for (player, action_state, mut velocity) in &mut query {
        let mut direction = Vec2::ZERO;

        if action_state.pressed(&PlayerAction::MoveUp) { direction.y += 1.0; }
        if action_state.pressed(&PlayerAction::MoveDown) { direction.y -= 1.0; }
        if action_state.pressed(&PlayerAction::MoveRight) { direction.x += 1.0; }
        if action_state.pressed(&PlayerAction::MoveLeft) { direction.x -= 1.0; }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }
        velocity.0 = direction * player.speed;
    }
}

// 5. The App
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Physics & Input Demo".into(),
                    resolution: (1600, 900).into(),
                    present_mode: PresentMode::AutoVsync, 
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default().with_length_unit(20.0), 
            InputManagerPlugin::<PlayerAction>::default(),
            InputManagerPlugin::<SystemAction>::default(),
        ))
        .insert_resource(Gravity(Vec2::ZERO)) 
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_movement, toggle_fullscreen))
        .run();
}