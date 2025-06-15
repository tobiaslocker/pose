from mediapipe.framework.formats import landmark_pb2
import cv2
import mediapipe as mp
from mediapipe.framework.formats import landmark_pb2

mp_drawing = mp.solutions.drawing_utils
mp_pose = mp.solutions.pose

def draw_result(result, bgr_image):
    if not result.pose_landmarks:
        return

    landmark_proto = landmark_pb2.NormalizedLandmarkList()
    for lm in result.pose_landmarks[0]:
        landmark_proto.landmark.add(
            x=lm.x,
            y=lm.y,
            z=lm.z,
            visibility=lm.visibility
        )

    mp_drawing.draw_landmarks(
        bgr_image,
        landmark_proto,
        mp_pose.POSE_CONNECTIONS,
        mp_drawing.DrawingSpec(color=(0, 255, 0), thickness=2, circle_radius=2),
        mp_drawing.DrawingSpec(color=(255, 0, 0), thickness=2)
    )

    cv2.imshow("MediaPipe Drawing", bgr_image)
    cv2.waitKey(1)

