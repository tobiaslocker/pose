use flatbuffers::{FlatBufferBuilder, WIPOffset};
use pose::detection::provider::DetectionProvider;
use pose::generated::detection::{
    Availability, AvailabilityArgs, DetectionMessage, DetectionMessageArgs, DetectionPayload,
    Landmark, LandmarkArgs, PoseDetectionResult, PoseDetectionResultArgs,
};
use pose::network::forward::forward;
use pose::network::provider::ChannelDetectionProvider;
use pose::network::tcp::FramedPayloadStream;
use pose::protocol::parse;
use std::io::Write;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

fn create_dummy_flatbuffer() -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let avail = Availability::create(
        &mut builder,
        &AvailabilityArgs {
            visibility: 0.9,
            presence: 0.8,
        },
    );

    let landmarks: Vec<WIPOffset<Landmark>> = vec![Landmark::create(
        &mut builder,
        &LandmarkArgs {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            availability: Some(avail),
        },
    )];

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
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind failed");
    let addr = listener.local_addr().unwrap();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept failed");
        let dummy_data = create_dummy_flatbuffer();
        let len = (dummy_data.len() as u32).to_le_bytes();
        stream.write_all(&len).unwrap();
        stream.write_all(&dummy_data).unwrap();
    });

    let (tx, rx) = mpsc::channel(8);
    let stream = FramedPayloadStream::connect(&addr.to_string())
        .await
        .expect("TCP connect failed");

    forward(stream, tx, parse);

    let mut provider = ChannelDetectionProvider::new(rx);
    let mut retries = 0;
    let mut result = None;

    while result.is_none() && retries < 10 {
        result = provider.poll();
        retries += 1;
        sleep(Duration::from_millis(50)).await;
    }

    assert!(result.is_some(), "Expected detection result, got none");
    let detection = result.unwrap();
    assert_eq!(detection.landmarks.len(), 1);
    assert_eq!(detection.landmarks[0].x, 1.0);
}
