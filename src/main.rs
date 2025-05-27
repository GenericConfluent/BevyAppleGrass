use bevy::prelude::*;

fn say_hello() {
    println!("Hello");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, say_hello)
        .run();
}
