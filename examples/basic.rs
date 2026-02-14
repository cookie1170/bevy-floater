//! This example shows off a basic character controller

use avian2d::prelude::*;
use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_floater::{Controller, ControllerPlugin};

fn move_player(
    mut player: Single<(&mut LinearVelocity, &mut Player, &mut Controller)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    player.1.coyote_timer -= time.delta_secs();
    player.1.buffer_timer -= time.delta_secs();
    player.1.jump_timer -= time.delta_secs();
    let mut dir = 0.0;

    if keyboard.pressed(KeyCode::KeyA) {
        dir -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        dir += 1.0;
    }

    let goal = dir * Player::MAX_SPEED;
    let accel = if (player.1.goal_velocity * goal) < 0.0 {
        Player::ACCEL * Player::TURNAROUND_MULT
    } else {
        Player::ACCEL
    };
    player.1.goal_velocity = move_towards(player.1.goal_velocity, goal, accel * time.delta_secs());

    let needed_force = player.1.goal_velocity - player.0.x;
    player.0.0 += Vec2::X * needed_force;

    if player.2.is_grounded() {
        player.1.coyote_timer = Player::COYOTE_TIME;
    }

    if player.1.coyote_timer > 0.0 && player.1.buffer_timer > 0.0 && player.1.jump_timer <= 0.0 {
        player.0.0.y = Player::JUMP_VELOCITY.max(player.0.0.y + Player::JUMP_VELOCITY); // this makes it so you can jump properly while walking up slopes!
        player.1.coyote_timer = 0.0;
        player.1.buffer_timer = 0.0;
        player.1.jump_timer = Player::JUMP_SKIP_TIME;
    }

    player.2.skip_acceleration = player.1.jump_timer > 0.0;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Transform::from_xyz(0.0, 128.0, 0.0),
        Collider::capsule(32.0, 24.0),
        Controller::new(64.0).get_bundle(),
        TransformInterpolation,
        Mesh2d(meshes.add(Capsule2d::new(32.0, 64.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
        Player::default(),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, -256.0, 0.0),
        Collider::rectangle(1024.0, 16.0),
        RigidBody::Static,
        Mesh2d(meshes.add(Rectangle::new(1024.0, 16.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    ));

    commands.spawn((
        Transform::from_xyz(512.0, -128.0, 0.0)
            .with_rotation(Quat::from_rotation_z(30f32.to_radians())),
        Collider::rectangle(512.0, 16.0),
        RigidBody::Static,
        Mesh2d(meshes.add(Rectangle::new(512.0, 16.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    ));
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin,
        ControllerPlugin,
    ));
    app.insert_resource(Gravity(Vec2::NEG_Y * 980.0));

    app.add_systems(Startup, setup)
        .add_systems(Update, update_jump_buffer)
        .add_systems(FixedUpdate, move_player);

    app.run();
}

#[derive(Component, Default)]
struct Player {
    coyote_timer: f32,
    buffer_timer: f32,
    goal_velocity: f32,
    jump_timer: f32,
}

impl Player {
    const MAX_SPEED: f32 = 512.0;
    const ACCEL: f32 = 1024.0;
    const JUMP_VELOCITY: f32 = 512.0;
    const TURNAROUND_MULT: f32 = 2.0;
    const COYOTE_TIME: f32 = 0.2;
    const BUFFER_TIME: f32 = 0.25;
    const JUMP_SKIP_TIME: f32 = 0.5;
}

fn update_jump_buffer(mut player: Single<&mut Player>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        player.buffer_timer = Player::BUFFER_TIME
    }
}

fn move_towards(current: f32, target: f32, delta: f32) -> f32 {
    let diff = target - current;
    let abs = diff.abs();

    if delta > abs {
        return target;
    }

    current + (diff / abs * delta)
}
