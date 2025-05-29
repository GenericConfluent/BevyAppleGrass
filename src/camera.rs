use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct CameraTarget;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_target);
    }
}

fn follow_target(
    target_query: Query<&Transform, With<CameraTarget>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<CameraTarget>)>,
) {
    let Ok(target) = target_query.single() else {
        return;
    };

    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };

    camera.translation.x = target.translation.x;
    camera.translation.y = target.translation.y;
}
