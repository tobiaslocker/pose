use pose::network::{forward::forward, tcp::Client};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
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
async fn test_forward_with_real_tcp() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();

        let packets = vec![TestPacket(42), TestPacket(7), TestPacket(999_999)];

        for packet in &packets {
            let raw = encode_packet(packet);
            socket.write_all(&raw).await.unwrap();
        }
    });

    let mut client = Client::new();
    client
        .connect(&addr.ip().to_string(), addr.port())
        .await
        .unwrap();
    let stream = client.into_stream();

    let (tx, mut rx) = mpsc::channel(10);
    forward(stream, tx, parse_packet);

    let expected_packets = vec![TestPacket(42), TestPacket(7), TestPacket(999_999)];

    for expected in expected_packets {
        let actual = timeout(Duration::from_secs(1), rx.recv())
            .await
            .expect("timeout")
            .expect("channel closed");
        assert_eq!(actual, expected);
    }

    server_task.await.unwrap();
}
