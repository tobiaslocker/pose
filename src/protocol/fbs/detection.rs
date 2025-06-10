use crate::generated::detection;
use flatbuffers::root;

#[derive(Debug, Clone, PartialEq)]
pub struct Availability {
    pub visibility: f32,
    pub presence: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Landmark {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub availability: Option<Availability>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DetectionResult {
    pub landmarks: Vec<Landmark>,
}

pub type PoseDetectionResult = DetectionResult;
pub type HandDetectionResult = DetectionResult;

pub fn parse(bytes: &[u8]) -> Option<PoseDetectionResult> {
    let msg = root::<detection::DetectionMessage>(bytes).ok()?;
    match msg.payload_type() {
        detection::DetectionPayload::PoseDetectionResult => {
            let pose = msg.payload_as_pose_detection_result()?;
            let landmarks = pose
                .landmarks()?
                .iter()
                .map(|lm| {
                    let availability = lm.availability().map(|a| Availability {
                        visibility: a.visibility(),
                        presence: a.presence(),
                    });

                    Landmark {
                        x: lm.x(),
                        y: lm.y(),
                        z: lm.z(),
                        availability,
                    }
                })
                .collect();

            Some(DetectionResult { landmarks })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::detection::{
        Availability, AvailabilityArgs, DetectionMessage, DetectionMessageArgs, DetectionPayload,
        Landmark, LandmarkArgs, PoseDetectionResult, PoseDetectionResultArgs,
    };
    use flatbuffers::FlatBufferBuilder;

    #[test]
    fn test_parse_pose_detection_result_from_flatbuffer_bytes() {
        let mut builder = FlatBufferBuilder::new();

        let availability1 = Availability::create(
            &mut builder,
            &AvailabilityArgs {
                visibility: 0.99,
                presence: 0.95,
            },
        );
        let availability2 = Availability::create(
            &mut builder,
            &AvailabilityArgs {
                visibility: 0.89,
                presence: 0.85,
            },
        );

        let landmarks = vec![
            Landmark::create(
                &mut builder,
                &LandmarkArgs {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                    availability: Some(availability1),
                },
            ),
            Landmark::create(
                &mut builder,
                &LandmarkArgs {
                    x: 4.0,
                    y: 5.0,
                    z: 6.0,
                    availability: Some(availability2),
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
        let bytes = builder.finished_data();

        let parsed = parse(bytes).expect("failed to parse");

        assert_eq!(parsed.landmarks.len(), 2);
        assert_eq!(parsed.landmarks[0].x, 1.0);
        assert_eq!(parsed.landmarks[1].z, 6.0);
        assert!(parsed.landmarks[0].availability.is_some());
    }
}
