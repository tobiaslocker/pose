use crate::network::stream::PayloadStream;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

pub fn forward<T, P>(
    mut stream: impl PayloadStream + Send + 'static,
    tx: Sender<T>,
    parser: P,
) -> JoinHandle<()>
where
    T: Send + 'static,
    P: Fn(&[u8]) -> Option<T> + Send + Sync + 'static,
{
    tokio::spawn(async move {
        while let Some(payload) = stream.next_payload().await {
            if let Some(parsed) = parser(&payload) {
                if tx.send(parsed).await.is_err() {
                    eprintln!("Receiver dropped, stopping forwarder");
                    break;
                }
            } else {
                eprintln!(
                    "Failed to parse message, skipping ({} bytes)",
                    payload.len()
                );
            }
        }

        eprintln!("PayloadStream ended, forwarder exiting");
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::forward;
    use std::pin::Pin;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::mpsc;
    use tokio::time::{Duration, timeout};

    #[derive(Debug, PartialEq)]
    struct TestPacket(u32);

    fn encode_packet(packet: &TestPacket) -> Vec<u8> {
        let mut buf = Vec::new();
        let data = packet.0.to_le_bytes();
        buf.extend(&(data.len() as u32).to_le_bytes());
        buf.extend(&data);
        buf
    }

    fn parse_packet(bytes: &[u8]) -> Option<TestPacket> {
        if bytes.len() != 4 {
            return None;
        }
        Some(TestPacket(u32::from_le_bytes(bytes.try_into().ok()?)))
    }

    #[tokio::test(flavor = "current_thread")]
    async fn test_forward_sends_parsed_packets() {
        let (mut client_side, server_side) = tokio::io::duplex(64);
        let (tx, mut rx) = mpsc::channel(10);

        struct DummyStream {
            inner: tokio::io::DuplexStream,
        }

        impl PayloadStream for DummyStream {
            fn next_payload<'a>(
                &'a mut self,
            ) -> Pin<Box<dyn std::future::Future<Output = Option<Vec<u8>>> + Send + 'a>>
            {
                Box::pin(async move {
                    let mut length_buf = [0u8; 4];
                    if self.inner.read_exact(&mut length_buf).await.is_err() {
                        return None;
                    }

                    let length = u32::from_le_bytes(length_buf) as usize;
                    let mut buffer = vec![0; length];
                    if self.inner.read_exact(&mut buffer).await.is_err() {
                        return None;
                    }

                    Some(buffer)
                })
            }
        }

        let stream = DummyStream { inner: server_side };

        forward::forward(stream, tx, parse_packet);

        let packets = vec![TestPacket(42), TestPacket(7)];
        for pkt in &packets {
            let raw = encode_packet(pkt);
            client_side.write_all(&raw).await.unwrap();
        }

        for expected in packets {
            let actual = timeout(Duration::from_secs(1), rx.recv())
                .await
                .expect("timeout")
                .expect("channel closed");

            assert_eq!(actual, expected);
        }
    }
}
