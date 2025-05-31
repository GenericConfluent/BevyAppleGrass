use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod camera;
mod enemy;
mod player;

fn load_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map/world.ldtk").into(),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            LdtkPlugin,
            camera::CameraPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
        ))
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, load_world)
        .run();
}
