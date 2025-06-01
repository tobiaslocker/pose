import pytest

from pose.protocol.fbs.detection import parse

import Detection.DetectionMessage as DetectionMessage
import Detection.DetectionPayload as DetectionPayload
import Detection.PoseDetectionResult as PoseDetectionResult

class DummyLandmark:
    def __init__(self, x, y, z, visibility, presence):
        self.x = x
        self.y = y
        self.z = z
        self.visibility = visibility
        self.presence = presence

class DummyPoseLandmarkerResult:
    def __init__(self, landmarks):
        self.pose_landmarks = [landmarks]

@pytest.mark.parametrize("num_landmarks", [0, 1, 5])
def test_parser_generates_valid_flatbuffer(num_landmarks):
    dummy_landmarks = [
        DummyLandmark(x=i, y=i+1, z=i+2, visibility=0.9, presence=0.8)
        for i in range(num_landmarks)
    ]
    dummy_result = DummyPoseLandmarkerResult(dummy_landmarks)
    buf = parse(dummy_result)

    msg = DetectionMessage.DetectionMessage.GetRootAsDetectionMessage(buf, 0)
    assert msg.PayloadType() == DetectionPayload.DetectionPayload().PoseDetectionResult

    payload = msg.Payload()
    assert payload is not None, "Payload is None"

    pose_result = PoseDetectionResult.PoseDetectionResult()
    pose_result.Init(payload.Bytes, payload.Pos)

    assert pose_result.LandmarksLength() == num_landmarks

    for i in range(num_landmarks):
        lm = pose_result.Landmarks(i)
        assert lm is not None, f"Landmark {i} is None"

        assert lm.X() == i
        assert lm.Y() == i + 1
        assert lm.Z() == i + 2

        availability = lm.Availability()
        assert availability is not None, f"Landmark {i} availability is None"
        assert availability.Visibility() == pytest.approx(0.9)
        assert availability.Presence() == pytest.approx(0.8)

