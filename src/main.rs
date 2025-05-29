use avian2d::PhysicsPlugins;
use bevy::prelude::*;

mod camera;
mod player;

fn spawn_square(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rect = meshes.add(Rectangle::new(50.0, 50.0));
    let mat = materials.add(Color::linear_rgb(0.0, 1.0, 0.0));

    commands.spawn((
        Mesh2d(rect),
        MeshMaterial2d(mat),
        Transform::from_scale(Vec3::splat(0.3)),
    ));
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            camera::CameraPlugin,
            player::PlayerPlugin,
        ))
        .add_systems(Startup, spawn_square)
        .run();
}
