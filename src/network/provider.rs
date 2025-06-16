use crate::protocol::LandmarkFrame;
use tokio::io;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum ConnectionError {
    Io(io::Error),
    AddrParse(std::net::AddrParseError),
}

pub struct ChannelStreamProvider {
    receiver: Receiver<LandmarkFrame>,
}

impl ChannelStreamProvider {
    pub fn new(receiver: Receiver<LandmarkFrame>) -> Self {
        Self { receiver }
    }
}

impl crate::landmark::StreamProvider for ChannelStreamProvider {
    fn poll(&mut self) -> Option<LandmarkFrame> {
        self.receiver.try_recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::landmark::StreamProvider;
    use crate::protocol::LandmarkFrame;
    use crate::protocol::fbs::landmark::{Availability, Landmark};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_tcp_detection_provider_poll() {
        let (tx, rx) = mpsc::channel(1);

        let dummy = LandmarkFrame {
            landmarks: vec![Landmark {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                availability: Some(Availability {
                    visibility: 0.9,
                    presence: 0.8,
                }),
            }],
            timestamp: 1234.5678,
        };

        tx.send(dummy.clone()).await.expect("send failed");

        let mut provider = ChannelStreamProvider::new(rx);

        let result = provider.poll();
        assert!(result.is_some(), "Expected Some(LandmarkFrame)");
        assert_eq!(result.unwrap(), dummy);
    }
}
