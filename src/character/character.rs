use crate::character::{Landmarks, Skeleton};
use crate::protocol::Landmark;
use bevy::prelude::*;

pub struct Character {
    pub landmarks: Landmarks,
    pub skeleton: Skeleton,
}

impl Character {
    pub fn new() -> Self {
        Self {
            landmarks: Landmarks::new(),
            skeleton: Skeleton::new(),
        }
    }

    pub fn spawn(&self, commands: &mut Commands) {
        self.landmarks.spawn(commands);
    }

    pub fn draw(
        &self,
        landmarks: &[Landmark],
        query: &mut Query<(&mut Transform, &crate::landmark::Index, &mut Visibility)>,
        gizmos: &mut Gizmos,
        windows: Query<&Window>,
    ) {
        self.landmarks.draw(landmarks, query, windows);
        self.skeleton.draw(landmarks, gizmos, windows);
    }
}
