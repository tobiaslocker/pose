// tests/test_system_tcp.rs

use std::io::Write;
use std::net::TcpListener;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::task;
use tokio::time::sleep;

use pose::detection::provider::DetectionProvider;
use pose::network::forward::forward;
use pose::network::tcp::TcpDetectionProvider;
use pose::protocol::{DetectionResult, parse};

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use pose::generated::detection::{
    Availability, AvailabilityArgs, DetectionMessage, DetectionMessageArgs, DetectionPayload,
    Landmark, LandmarkArgs, PoseDetectionResult, PoseDetectionResultArgs,
};

fn create_dummy_flatbuffer() -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let avail1 = Availability::create(
        &mut builder,
        &AvailabilityArgs {
            visibility: 0.9,
            presence: 0.8,
        },
    );

    let avail2 = Availability::create(
        &mut builder,
        &AvailabilityArgs {
            visibility: 0.7,
            presence: 0.6,
        },
    );

    let landmarks: Vec<WIPOffset<Landmark>> = vec![
        Landmark::create(
            &mut builder,
            &LandmarkArgs {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                availability: Some(avail1),
            },
        ),
        Landmark::create(
            &mut builder,
            &LandmarkArgs {
                x: 4.0,
                y: 5.0,
                z: 6.0,
                availability: Some(avail2),
            },
        ),
    ];

    let landmarks_vec = builder.create_vector(&landmarks);

    let pose_result = PoseDetectionResult::create(
        &mut builder,
        &PoseDetectionResultArgs {
            landmarks: Some(landmarks_vec),
        },
    );

    let message = DetectionMessage::create(
        &mut builder,
        &DetectionMessageArgs {
            payload_type: DetectionPayload::PoseDetectionResult,
            payload: Some(pose_result.as_union_value()),
        },
    );

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

#[tokio::test]
async fn test_tcp_provider_end_to_end() {
    let port = 9100;
    let addr = format!("127.0.0.1:{}", port);

    TcpListener::bind(&addr).expect("Port unavailable");

    let addr_clone = addr.clone();
    let (tx, rx) = mpsc::channel::<DetectionResult>(8);
    task::spawn(async move {
        let listener = tokio::net::TcpListener::bind(&addr_clone).await.unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        forward(stream, tx, parse);
    });

    sleep(Duration::from_millis(100)).await;

    task::spawn_blocking(move || {
        let dummy_data = create_dummy_flatbuffer();
        let mut stream = std::net::TcpStream::connect(&addr).expect("Connect failed");
        let len = (dummy_data.len() as u32).to_le_bytes(); // LE because forward uses LE
        stream.write_all(&len).unwrap();
        stream.write_all(&dummy_data).unwrap();
    });

    let mut provider = TcpDetectionProvider::new(rx);
    let mut retries = 0;
    let mut result = None;

    while result.is_none() && retries < 10 {
        result = provider.poll();
        retries += 1;
        sleep(Duration::from_millis(100)).await;
    }

    assert!(result.is_some(), "Expected detection result, got none");
    let detection = result.unwrap();
    assert_eq!(detection.landmarks.len(), 2);
    assert_eq!(detection.landmarks[0].x, 1.0);
}
