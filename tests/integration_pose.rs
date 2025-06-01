use flatbuffers::FlatBufferBuilder;
use pose::generated::detection::{
    Availability, AvailabilityArgs, DetectionMessage, DetectionMessageArgs, DetectionPayload,
    Landmark, LandmarkArgs, PoseDetectionResult, PoseDetectionResultArgs,
    finish_detection_message_buffer,
};
use pose::protocol::fbs::detection::parse;

#[test]
fn test_parse_pose_integration() {
    let mut builder = FlatBufferBuilder::new();

    let availability = Availability::create(
        &mut builder,
        &AvailabilityArgs {
            visibility: 0.9,
            presence: 0.95,
        },
    );

    let l1 = Landmark::create(
        &mut builder,
        &LandmarkArgs {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            availability: Some(availability),
        },
    );

    let l2 = Landmark::create(
        &mut builder,
        &LandmarkArgs {
            x: 4.0,
            y: 5.0,
            z: 6.0,
            availability: None,
        },
    );

    let landmarks_vec = builder.create_vector(&[l1, l2]);

    let pose_result = PoseDetectionResult::create(
        &mut builder,
        &PoseDetectionResultArgs {
            landmarks: Some(landmarks_vec),
        },
    );

    let detection_msg = DetectionMessage::create(
        &mut builder,
        &DetectionMessageArgs {
            payload_type: DetectionPayload::PoseDetectionResult,
            payload: Some(pose_result.as_union_value()),
        },
    );

    finish_detection_message_buffer(&mut builder, detection_msg);
    let bytes = builder.finished_data();

    let parsed = parse(bytes).expect("Should successfully parse");

    assert_eq!(parsed.landmarks.len(), 2);
    assert_eq!(parsed.landmarks[0].x, 1.0);
    assert_eq!(parsed.landmarks[0].y, 2.0);
    assert_eq!(parsed.landmarks[1].z, 6.0);
}
