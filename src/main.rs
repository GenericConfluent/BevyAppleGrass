use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct AccumulatedInput(Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PhysicalTranslation(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

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
        Transform::from_scale(Vec3::splat(0.3)),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
        Player));
    commands.spawn(Camera2d);
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut Velocity)>,
) {
    const SPEED: f32 = 210.0;
    for (mut input, mut velocity) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) { input.y += 1.0; }
        if keyboard_input.pressed(KeyCode::KeyS) { input.y -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyA) { input.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyD) { input.x += 1.0; }
        velocity.0 = input.extend(0.0).normalize_or_zero() * SPEED;
    }
}


fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut AccumulatedInput,
        &Velocity,
    )>,
) {
    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        velocity,
    ) in query.iter_mut() {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();

        input.0 = Vec2::ZERO;
    }
}

fn interpolate_rendered_transform( 
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in query.iter_mut() {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;
        let alpha = fixed_time.overstep_fraction();

        let rendered_translaton = previous.lerp(current, alpha);
        transform.translation = rendered_translaton;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(Update, (handle_input, interpolate_rendered_transform))
        .run();
}
