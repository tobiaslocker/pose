use crate::components::LandmarkIndex;
use crate::protocol::Landmark;
use bevy::prelude::*;

pub trait LandmarkDrawing {
    fn connections() -> &'static [(usize, usize)];

    fn num_landmarks() -> usize;

    fn color() -> Color {
        Color::srgb(0.0, 1.0, 0.0)
    }

    fn to_bevy_coords(lm: &Landmark, width: f32, height: f32) -> Vec3 {
        Vec3::new((lm.x - 0.5) * width, -(lm.y - 0.5) * height, lm.z * 1000.0)
    }

    fn update_landmarks(
        landmarks: &[Landmark],
        query: &mut Query<(&mut Transform, &LandmarkIndex)>,
        windows: Query<&Window>,
    ) {
        if let Some(window) = windows.iter().next() {
            let width = window.width();
            let height = window.height();
            for (mut transform, LandmarkIndex(i)) in query.iter_mut() {
                if let Some(lm) = landmarks.get(*i) {
                    transform.translation = Self::to_bevy_coords(lm, width, height);
                }
            }
        }
    }

    fn draw_connections(landmarks: &[Landmark], gizmos: &mut Gizmos, windows: Query<&Window>) {
        if let Some(window) = windows.iter().next() {
            let width = window.width();
            let height = window.height();
            for &(start_idx, end_idx) in Self::connections() {
                if let (Some(a), Some(b)) = (landmarks.get(start_idx), landmarks.get(end_idx)) {
                    let a = Self::to_bevy_coords(a, width, height);
                    let b = Self::to_bevy_coords(b, width, height);
                    gizmos.line(a, b, Self::color());
                }
            }
        }
    }

    fn spawn_indexed_sprites(commands: &mut Commands) {
        for i in 0..Self::num_landmarks() {
            commands.spawn((
                Sprite {
                    color: Self::color(),
                    custom_size: Some(Vec2::splat(16.0)),
                    ..default()
                },
                Transform::default(),
                GlobalTransform::default(),
                LandmarkIndex(i),
            ));
        }
    }
}
