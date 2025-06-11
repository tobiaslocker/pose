import flatbuffers
import Detection.DetectionMessage as DetectionMessage
import Detection.DetectionPayload as DetectionPayload
import Detection.PoseDetectionResult as PoseDetectionResult
import Detection.Landmark as Landmark
import Detection.Availability as Availability

def parse(pose_landmarker_result) -> bytes:
    """Serialize PoseLandmarkerResult to FlatBuffers DetectionMessage (as bytes)."""

    builder = flatbuffers.Builder(1024)
    landmark_offsets = []

    if len(pose_landmarker_result.pose_landmarks) == 0:
        DetectionMessage.Start(builder)
        DetectionMessage.AddPayloadType(builder, DetectionPayload.DetectionPayload().Empty)
        detection_message_offset = DetectionMessage.End(builder)
        builder.Finish(detection_message_offset)
        return bytes(builder.Output())

    for landmark in pose_landmarker_result.pose_landmarks[0]:
        Availability.Start(builder)
        Availability.AddVisibility(builder, landmark.visibility)
        Availability.AddPresence(builder, landmark.presence)
        availability_offset = Availability.End(builder)
        Landmark.Start(builder)
        Landmark.AddX(builder, landmark.x)
        Landmark.AddY(builder, landmark.y)
        Landmark.AddZ(builder, landmark.z)
        Landmark.AddAvailability(builder, availability_offset)
        landmark_offset = Landmark.End(builder)
        landmark_offsets.append(landmark_offset)

    PoseDetectionResult.StartLandmarksVector(builder, len(landmark_offsets))

    for lm_offset in reversed(landmark_offsets):
        builder.PrependUOffsetTRelative(lm_offset)

    landmarks_vector = builder.EndVector()
    PoseDetectionResult.Start(builder)
    PoseDetectionResult.AddLandmarks(builder, landmarks_vector)
    pose_detection_result_offset = PoseDetectionResult.End(builder)
    DetectionMessage.Start(builder)
    DetectionMessage.AddPayloadType(builder, DetectionPayload.DetectionPayload().PoseDetectionResult)
    DetectionMessage.AddPayload(builder, pose_detection_result_offset)
    detection_message_offset = DetectionMessage.End(builder)
    builder.Finish(detection_message_offset)

    return bytes(builder.Output())

