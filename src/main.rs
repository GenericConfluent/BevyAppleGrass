use avian2d::PhysicsPlugins;
use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            player::PlayerPlugin,
        ))
        .run();
}
