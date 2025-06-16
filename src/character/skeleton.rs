use crate::landmark::denormalize;
use crate::protocol::Landmark;
use bevy::prelude::*;

pub struct Skeleton;

impl Skeleton {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, landmarks: &[Landmark], gizmos: &mut Gizmos, windows: Query<&Window>) {
        const CONNECTIONS: &[(usize, usize)] = &[
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 7),
            (0, 4),
            (4, 5),
            (5, 6),
            (6, 8),
            (9, 10),
            (11, 12),
            (11, 13),
            (13, 15),
            (15, 17),
            (15, 19),
            (15, 21),
            (17, 19),
            (19, 21),
            (12, 14),
            (14, 16),
            (16, 18),
            (16, 20),
            (16, 22),
            (18, 20),
            (20, 22),
            (11, 23),
            (12, 24),
            (23, 24),
            (23, 25),
            (25, 27),
            (27, 29),
            (29, 31),
            (24, 26),
            (26, 28),
            (28, 30),
            (30, 32),
        ];

        if let Some(window) = windows.iter().next() {
            for &(start, end) in CONNECTIONS {
                if let (Some(a), Some(b)) = (landmarks.get(start), landmarks.get(end)) {
                    if let (Some(pa), Some(pb)) = (&a.availability, &b.availability) {
                        if pa.presence >= 0.5
                            && pb.presence >= 0.5
                            && pa.visibility >= 0.5
                            && pb.visibility >= 0.5
                        {
                            gizmos.line(
                                denormalize(a, window),
                                denormalize(b, window),
                                Color::srgb(0.0, 1.0, 0.0),
                            );
                        }
                    }
                }
            }
        }
    }
}
