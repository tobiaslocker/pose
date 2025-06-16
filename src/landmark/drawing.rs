use crate::landmark::Index;
use crate::protocol::Landmark;
use bevy::prelude::*;
use bevy::render::view::Visibility;

pub fn denormalize(landmark: &Landmark, window: &Window) -> Vec3 {
    Vec3::new(
        (landmark.x - 0.5) * window.width(),
        -(landmark.y - 0.5) * window.height(),
        landmark.z * 1000.0,
    )
}

pub trait Drawing {
    const PRESENCE_THRESHOLD: f32 = 0.5;
    const VISIBILITY_THRESHOLD: f32 = 0.5;

    fn num_landmarks() -> usize;
    fn color() -> Color {
        Color::srgb(0.0, 1.0, 0.0)
    }

    fn update(
        landmarks: &[Landmark],
        query: &mut Query<(&mut Transform, &Index, &mut Visibility)>,
        windows: Query<&Window>,
    ) {
        if let Some(window) = windows.iter().next() {
            for (mut transform, Index(i), mut visibility) in query.iter_mut() {
                if let Some(lm) = landmarks.get(*i).filter(|lm| {
                    lm.availability.as_ref().map_or(false, |av| {
                        av.presence >= Self::PRESENCE_THRESHOLD
                            && av.visibility >= Self::VISIBILITY_THRESHOLD
                    })
                }) {
                    transform.translation = denormalize(lm, window);
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }

    fn spawn(commands: &mut Commands) {
        for i in 0..Self::num_landmarks() {
            commands.spawn((
                Sprite {
                    color: Self::color(),
                    custom_size: Some(Vec2::splat(16.0)),
                    ..default()
                },
                Transform::default(),
                GlobalTransform::default(),
                Index(i),
                Visibility::Inherited,
            ));
        }
    }
}
