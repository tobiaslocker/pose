use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Index(pub usize);

impl From<usize> for Index {
    fn from(index: usize) -> Self {
        Index(index)
    }
}
