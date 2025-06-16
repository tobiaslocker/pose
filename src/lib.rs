pub mod character;
pub mod landmark;
pub mod network;
pub mod protocol;
pub mod realm;

#[allow(warnings)]
#[path = "../generated/rust/pose_generated.rs"]
mod pose_generated;

pub mod generated {
    pub use crate::pose_generated::detection;
}
