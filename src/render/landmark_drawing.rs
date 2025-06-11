use crate::components::LandmarkIndex;
use crate::protocol::Landmark;
use bevy::prelude::*;
use bevy::render::view::Visibility;

pub trait LandmarkDrawing {
    const PRESENCE_THRESHOLD: f32 = 0.5;
    const VISIBILITY_THRESHOLD: f32 = 0.5;

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
        query: &mut Query<(&mut Transform, &LandmarkIndex, &mut Visibility)>,
        windows: Query<&Window>,
    ) {
        if let Some(window) = windows.iter().next() {
            let width = window.width();
            let height = window.height();
            for (mut transform, LandmarkIndex(i), mut visibility) in query.iter_mut() {
                if let Some(lm) = landmarks.get(*i).filter(|lm| {
                    lm.availability.as_ref().map_or(false, |av| {
                        av.presence >= Self::PRESENCE_THRESHOLD
                            && av.visibility >= Self::VISIBILITY_THRESHOLD
                    })
                }) {
                    transform.translation = Self::to_bevy_coords(lm, width, height);
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
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
                    if let (Some(pa), Some(pb)) = (&a.availability, &b.availability) {
                        if pa.presence < Self::PRESENCE_THRESHOLD
                            || pb.presence < Self::PRESENCE_THRESHOLD
                        {
                            continue;
                        }
                        if pa.visibility < Self::VISIBILITY_THRESHOLD
                            || pb.visibility < Self::VISIBILITY_THRESHOLD
                        {
                            continue;
                        }
                    }

                    let a_pos = Self::to_bevy_coords(a, width, height);
                    let b_pos = Self::to_bevy_coords(b, width, height);
                    gizmos.line(a_pos, b_pos, Self::color());
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
                Visibility::Inherited,
            ));
        }
    }
}
