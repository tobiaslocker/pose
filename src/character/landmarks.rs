use crate::landmark::Drawing;
use crate::landmark::Index;
use crate::protocol::Landmark;
use bevy::prelude::*;

pub struct Landmarks;

impl Landmarks {
    pub fn new() -> Self {
        Self
    }

    pub fn spawn(&self, commands: &mut Commands) {
        Landmark::spawn(commands);
    }

    pub fn draw(
        &self,
        landmarks: &[Landmark],
        query: &mut Query<(&mut Transform, &Index, &mut Visibility)>,
        windows: Query<&Window>,
    ) {
        Landmark::update(landmarks, query, windows);
    }
}

impl Drawing for Landmark {
    fn num_landmarks() -> usize {
        33
    }
}
