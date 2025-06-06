use avian2d::prelude::*;
use bevy::prelude::*;

use crate::camera::CameraTarget;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rect = meshes.add(Rectangle::new(50.0, 50.0));
    let mat = materials.add(Color::hsl(0.0, 1.0, 0.5));

    commands.spawn((
        Mesh2d(rect),
        MeshMaterial2d(mat),
        RigidBody::Kinematic,
        Collider::rectangle(50.0, 50.0),
        Transform::from_scale(Vec3::splat(0.3)),
        LinearVelocity(Vec2::ZERO),
        TransformInterpolation,
        CameraTarget,
        Player,
    ));
    commands.spawn(Camera2d);
}

fn move_player(
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut velocity) = player_query.single_mut() else {
        return;
    };

    const SPEED: f32 = 210.0;
    let mut input = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        input.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }
    if input == Vec2::ZERO {
        velocity.0 = Vec2::ZERO;
    }

    velocity.0 = input.normalize_or_zero() * SPEED;
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, move_player);
    }
}
