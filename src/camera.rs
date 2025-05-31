use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Resource)]
pub struct Bias(Vec2);

#[derive(Component)]
pub struct CameraTarget;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_target)
            .insert_resource(CameraFollowSettings {
                ease: EaseFunction::Linear,
                duration: 0.05,
            });
    }
}

#[derive(Resource)]
struct CameraFollowSettings {
    ease: EaseFunction,
    duration: f32, // time in secconds
}

struct EaseState<T> {
    easing_curve: EasingCurve<T>,
    elapsed_secs: f32,
}

impl<T> From<EasingCurve<T>> for EaseState<T> {
    fn from(easing_curve: EasingCurve<T>) -> Self {
        Self {
            easing_curve,
            elapsed_secs: 0.0,
        }
    }
}

fn follow_target(
    target_query: Query<&Transform, With<CameraTarget>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<CameraTarget>)>,
    bias: Option<Res<Bias>>,
    follow_settings: Res<CameraFollowSettings>,
    time: Res<Time>,
    mut current_bias: Local<Vec2>,
    mut bias_ease_state: Local<Option<EaseState<Vec2>>>,
) {
    let Ok(target) = target_query.single() else {
        return;
    };

    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };

    let target_bias = bias.map(|b| b.0).unwrap_or_default();

    // Detect when target changes
    const MARGIN: f32 = 0.001;
    *current_bias = if let Some(ref mut ease_state) = *bias_ease_state {
        // We are easing
        if (target_bias - ease_state.easing_curve.domain().end()).length() > MARGIN {
            // The target changed => update the end and start
            ease_state.elapsed_secs = 0.0;
            ease_state.easing_curve =
                EasingCurve::new(*current_bias, target_bias, follow_settings.ease);
        }
        // Continue the current easing
        ease_state.elapsed_secs += time.delta_secs();
        ease_state
            .easing_curve
            .sample_clamped(ease_state.elapsed_secs / follow_settings.duration)
    } else {
        // We are not easing
        *bias_ease_state = if (target_bias - *current_bias).length() > 0.001 {
            // The target changed => start easing to target
            Some(EasingCurve::new(*current_bias, target_bias, follow_settings.ease).into())
        } else {
            // Remove the stored easing, it is complete
            None
        };
        Vec2::ZERO
    };

    let next_position = target.translation.xy() + *current_bias;
    camera.translation = next_position.extend(camera.translation.z);
}
