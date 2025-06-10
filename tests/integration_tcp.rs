use flatbuffers::FlatBufferBuilder;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use pose::generated::detection::{
    Availability, AvailabilityArgs, DetectionMessage, DetectionMessageArgs, DetectionPayload,
    Landmark, LandmarkArgs, PoseDetectionResult, PoseDetectionResultArgs,
};
use pose::protocol::fbs::detection::{
    Availability as ParsedAvailability, Landmark as ParsedLandmark, parse,
};

#[tokio::test]
async fn test_tcp_flatbuffer_parse_end_to_end() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut builder = FlatBufferBuilder::new();

        let availability = Availability::create(
            &mut builder,
            &AvailabilityArgs {
                visibility: 0.9,
                presence: 0.95,
            },
        );

        let landmarks = vec![
            Landmark::create(
                &mut builder,
                &LandmarkArgs {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                    availability: Some(availability),
                },
            ),
            Landmark::create(
                &mut builder,
                &LandmarkArgs {
                    x: 4.0,
                    y: 5.0,
                    z: 6.0,
                    availability: None,
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

        let payload_type = DetectionPayload::PoseDetectionResult;
        let payload = Some(pose_result.as_union_value());

        let detection_msg = DetectionMessage::create(
            &mut builder,
            &DetectionMessageArgs {
                payload_type,
                payload,
            },
        );

        pose::generated::detection::finish_detection_message_buffer(&mut builder, detection_msg);

        let data = builder.finished_data();
        let mut packet = Vec::new();
        packet.extend((data.len() as u32).to_le_bytes());
        packet.extend_from_slice(data);

        socket.write_all(&packet).await.unwrap();
    });

    let mut stream = TcpStream::connect(addr).await.unwrap();

    let mut length_buf = [0u8; 4];
    stream.read_exact(&mut length_buf).await.unwrap();
    let length = u32::from_le_bytes(length_buf);

    let mut buffer = vec![0u8; length as usize];
    stream.read_exact(&mut buffer).await.unwrap();

    let parsed = parse(&buffer).expect("Failed to parse detection");

    assert_eq!(parsed.landmarks.len(), 2);

    assert_eq!(
        parsed.landmarks[0],
        ParsedLandmark {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            availability: Some(ParsedAvailability {
                visibility: 0.9,
                presence: 0.95,
            }),
        }
    );

    assert_eq!(parsed.landmarks[1].z, 6.0);
}
