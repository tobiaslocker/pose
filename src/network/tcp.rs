use std::net::SocketAddr;
use tokio::io;
use tokio::net::TcpStream;

use crate::detection::provider::DetectionProvider;
use crate::protocol::DetectionResult;

use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum ConnectionError {
    Io(io::Error),
    AddrParse(std::net::AddrParseError),
}

pub struct Client {
    stream: Option<TcpStream>,
}

impl Client {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub async fn connect(&mut self, host: &str, port: u16) -> Result<(), ConnectionError> {
        let addr = format!("{}:{}", host, port)
            .parse::<SocketAddr>()
            .map_err(ConnectionError::AddrParse)?;

        let stream = TcpStream::connect(addr)
            .await
            .map_err(ConnectionError::Io)?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn into_stream(mut self) -> TcpStream {
        self.stream
            .take()
            .expect("Client was not connected before calling into_stream()")
    }
}

pub struct TcpDetectionProvider {
    receiver: Receiver<DetectionResult>,
}

impl TcpDetectionProvider {
    pub fn new(receiver: Receiver<DetectionResult>) -> Self {
        Self { receiver }
    }
}

impl DetectionProvider for TcpDetectionProvider {
    fn poll(&mut self) -> Option<DetectionResult> {
        self.receiver.try_recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let mut provider = TcpDetectionProvider::new(rx);

        let result = provider.poll();
        assert!(result.is_some(), "Expected Some(DetectionResult)");
        assert_eq!(result.unwrap(), dummy);
    }
}
