use crate::detection::provider::DetectionProvider;
use crate::protocol::DetectionResult;
use tokio::io;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum ConnectionError {
    Io(io::Error),
    AddrParse(std::net::AddrParseError),
}

pub struct ChannelDetectionProvider {
    receiver: Receiver<DetectionResult>,
}

impl ChannelDetectionProvider {
    pub fn new(receiver: Receiver<DetectionResult>) -> Self {
        Self { receiver }
    }
}

impl DetectionProvider for ChannelDetectionProvider {
    fn poll(&mut self) -> Option<DetectionResult> {
        self.receiver.try_recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detection::provider::DetectionProvider;
    use crate::protocol::DetectionResult;
    use crate::protocol::fbs::detection::{Availability, Landmark};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_tcp_detection_provider_poll() {
        let (tx, rx) = mpsc::channel(1);

        let dummy = DetectionResult {
            landmarks: vec![Landmark {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                availability: Some(Availability {
                    visibility: 0.9,
                    presence: 0.8,
                }),
            }],
        };

        tx.send(dummy.clone()).await.expect("send failed");

        let mut provider = ChannelDetectionProvider::new(rx);

        let result = provider.poll();
        assert!(result.is_some(), "Expected Some(DetectionResult)");
        assert_eq!(result.unwrap(), dummy);
    }
}
