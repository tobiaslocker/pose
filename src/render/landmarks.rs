use crate::components::LandmarkIndex;
use crate::protocol::fbs::detection::Landmark;

use bevy::prelude::*;

pub fn to_bevy_coords(lm: &Landmark) -> Vec3 {
    Vec3::new((lm.x - 0.5) * 1920.0, -(lm.y - 0.5) * 1080.0, lm.z * 1000.0)
}

pub fn update_landmarks(
    landmarks: &[Landmark],
    mut query: Query<(&mut Transform, &LandmarkIndex)>,
) {
    for (mut transform, LandmarkIndex(i)) in query.iter_mut() {
        if let Some(lm) = landmarks.get(*i) {
            transform.translation = to_bevy_coords(lm);
        }
    }
}

pub fn draw_connections(
    landmarks: &[Landmark],
    connections: &[(usize, usize)],
    gizmos: &mut Gizmos,
    color: Color,
) {
    for &(start_idx, end_idx) in connections {
        if let (Some(a), Some(b)) = (landmarks.get(start_idx), landmarks.get(end_idx)) {
            let a = to_bevy_coords(a);
            let b = to_bevy_coords(b);
            gizmos.line(a, b, color);
        }
    }
}
