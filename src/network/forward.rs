use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

/// Spawns an async task to forward parsed messages from the stream.
///
/// `T`: type of parsed message (e.g., Pose)
/// `P`: parser function: `&[u8] -> Option<T>`
///
pub fn forward<T, P, S>(stream: S, tx: Sender<T>, parser: P) -> JoinHandle<()>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    T: Send + 'static,
    P: Fn(&[u8]) -> Option<T> + Send + Sync + 'static,
{
    tokio::spawn(async move {
        let mut reader = BufReader::new(stream);
        let mut buffer = Vec::new();

        loop {
            let mut length_buf = [0u8; 4];

            if reader.read_exact(&mut length_buf).await.is_err() {
                eprintln!("Connection closed or failed");
                break;
            }

            let length = u32::from_le_bytes(length_buf) as usize;
            buffer.resize(length, 0);

            if reader.read_exact(&mut buffer).await.is_err() {
                eprintln!("Failed to read full message");
                break;
            }

            if let Some(parsed) = parser(&buffer) {
                if tx.send(parsed).await.is_err() {
                    eprintln!("Receiver dropped, stopping forwarder");
                    break;
                }
            } else {
                eprintln!(
                    "Failed to parse message, skipping ({} bytes read)",
                    buffer.len()
                );
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;
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

    #[tokio::test]
    async fn test_forward_sends_parsed_packets() {
        let (mut client_side, server_side) = tokio::io::duplex(64);
        let (tx, mut rx) = mpsc::channel(10);

        forward(server_side, tx, parse_packet);

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
