use avian2d::prelude::*;
use bevy::prelude::*;

use crate::camera::CameraTarget;

pub struct EnemyPlugin;

#[derive(Component)]
struct Enemy;

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rect = meshes.add(Rectangle::new(50.0, 50.0));
    let mat = materials.add(Color::linear_rgb(0.0, 1.0, 0.0));

    commands.spawn((
        Mesh2d(rect),
        MeshMaterial2d(mat),
        RigidBody::Kinematic,
        Collider::rectangle(50.0, 50.0),
        Transform::from_scale(Vec3::splat(0.3)),
        LinearVelocity(Vec2::ZERO),
        TransformInterpolation,
        Enemy,
    ));
}

fn follow_player(
    player_query: Query<&Transform, With<Player>>, //probably not the best way to do it if the camera ever targets anything else
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<CameraTarget>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        const SPEED: f32 = 1.0; //for some reason the way this calculation is done provides a much lower magnitude compared to player.rs

        for mut enemy_transform in enemy_query.iter_mut() {
            let direction = player_transform.translation - enemy_transform.translation;
            let movement = direction.truncate().normalize_or_zero() * SPEED;
            enemy_transform.translation += movement.extend(0.0);
        }
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
        app.add_systems(FixedUpdate, follow_player);
    }
}

