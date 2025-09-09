use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera);
    }
}

fn move_camera(camera: Single<&mut Transform, With<Camera>>, input: Res<AccumulatedMouseScroll>) {
    let mut camera = camera.into_inner();
    let x = input.delta.x;
    let y = input.delta.y;
    let translation = camera.translation + Vec3::new(y, 0.0, x);
    *camera = Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y);
}
