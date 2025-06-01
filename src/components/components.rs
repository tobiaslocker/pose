use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct LandmarkIndex(pub usize);

impl From<usize> for LandmarkIndex {
    fn from(index: usize) -> Self {
        LandmarkIndex(index)
    }
}
