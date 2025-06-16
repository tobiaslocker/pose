use crate::protocol::LandmarkFrame;

pub trait StreamProvider: Send + Sync {
    fn poll(&mut self) -> Option<LandmarkFrame>;
}

pub use crate::network::provider::ChannelStreamProvider;
