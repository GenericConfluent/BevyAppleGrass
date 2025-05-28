use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rect = meshes.add(Rectangle::new(50.0, 50.0));
    let mat = materials.add(Color::hsl(0.0, 1.0, 0.5));

    commands.spawn((Mesh2d(rect), MeshMaterial2d(mat), Player));
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .run();
}
