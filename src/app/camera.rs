use bevy::{input::mouse::MouseWheel, prelude::*};

const CAMERA_SPEED: f32 = 520.0;
const ZOOM_STEP: f32 = 0.12;
const MIN_ZOOM: f32 = 0.55;
const MAX_ZOOM: f32 = 2.2;

pub(super) fn move_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<&mut Transform, With<Camera2d>>,
) {
    let Ok(mut transform) = cameras.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * CAMERA_SPEED * time.delta_seconds();
    }
}

pub(super) fn zoom_camera(
    mut mouse_wheel: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut projections: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    let Ok(mut projection) = projections.get_single_mut() else {
        return;
    };

    let mut zoom_delta = 0.0;
    for event in mouse_wheel.read() {
        zoom_delta -= event.y * ZOOM_STEP;
    }
    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        zoom_delta -= ZOOM_STEP;
    }
    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
        zoom_delta += ZOOM_STEP;
    }

    if zoom_delta != 0.0 {
        projection.scale = (projection.scale + zoom_delta).clamp(MIN_ZOOM, MAX_ZOOM);
    }
}
