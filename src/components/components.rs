// src/components/landmark_index.rs

use bevy::prelude::*;

/// Identifies a sprite as representing a specific landmark by index.
#[derive(Component, Debug, Clone, Copy)]
pub struct LandmarkIndex(pub usize);

impl From<usize> for LandmarkIndex {
    fn from(index: usize) -> Self {
        LandmarkIndex(index)
    }
}
