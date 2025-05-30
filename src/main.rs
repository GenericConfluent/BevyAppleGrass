use avian2d::PhysicsPlugins;
use bevy::prelude::*;

mod camera;
mod player;
mod enemy;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            camera::CameraPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
        ))
        .run();
}
