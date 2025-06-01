pub mod components;
pub mod detection;
pub mod network;
pub mod protocol;
pub mod render;
pub mod resources;

#[allow(warnings)]
#[path = "../generated/rust/pose_generated.rs"]
mod pose_generated;

pub mod generated {
    pub use crate::pose_generated::detection;
}
