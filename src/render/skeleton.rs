use crate::components::LandmarkIndex;
use crate::render::landmark_drawing::LandmarkDrawing;

//use crate::render::landmarks::{draw_connections, update_landmarks};
use crate::resources::detection::Detection;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Skeleton {
    pub connections: Vec<(usize, usize)>,
}

impl Skeleton {
    pub const CONNECTIONS: &'static [(usize, usize)] = &[
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 7), // Face - right
        (0, 4),
        (4, 5),
        (5, 6),
        (6, 8),   // Face - left
        (9, 10),  // Mouth
        (11, 12), // Shoulders
        (11, 13),
        (13, 15), // Left Arm
        (15, 17),
        (15, 19),
        (15, 21), // Left Hand
        (17, 19),
        (19, 21), // Left Hand continued
        (12, 14),
        (14, 16), // Right Arm
        (16, 18),
        (16, 20),
        (16, 22), // Right Hand
        (18, 20),
        (20, 22), // Right Hand continued
        (11, 23),
        (12, 24), // Torso sides
        (23, 24), // Hips
        (23, 25),
        (25, 27), // Left Leg
        (27, 29),
        (29, 31), // Left Foot
        (24, 26),
        (26, 28), // Right Leg
        (28, 30),
        (30, 32), // Right Foot
    ];

    pub const LANDMARK_COUNT: usize = 33;

    pub fn setup(mut commands: Commands) {
        Self::spawn_indexed_sprites(&mut commands);
    }

    pub fn position_update(
        detection: Res<Detection>,
        mut gizmos: Gizmos,
        mut query: Query<(&mut Transform, &LandmarkIndex)>,
        windows: Query<&Window>,
    ) {
        if let Some(result) = detection.latest() {
            Self::update_landmarks(&result.landmarks, &mut query, windows);
            Self::draw_connections(&result.landmarks, &mut gizmos, windows);
        }
    }
}

impl LandmarkDrawing for Skeleton {
    fn connections() -> &'static [(usize, usize)] {
        Self::CONNECTIONS
    }

    fn num_landmarks() -> usize {
        Self::LANDMARK_COUNT
    }

    fn color() -> Color {
        Color::srgb(0.0, 1.0, 0.0) // skeleton green
    }
}
