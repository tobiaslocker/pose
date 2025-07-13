from mediapipe.framework.formats import landmark_pb2
import cv2
import mediapipe as mp
from mediapipe.framework.formats import landmark_pb2

mp_drawing = mp.solutions.drawing_utils
mp_pose = mp.solutions.pose

def draw_result(result):
    if not result.result.pose_landmarks:
        return

    # Define a few distinct colors
    colors = [
        (0, 255, 0),    # Green
        (255, 0, 0),    # Blue
        (0, 0, 255),    # Red
    ]

    for idx, pose_landmarks in enumerate(result.result.pose_landmarks):
        # Choose color based on index, cycle if more than len(colors)
        color = colors[idx % len(colors)]

        landmark_proto = landmark_pb2.NormalizedLandmarkList()
        for lm in pose_landmarks:
            landmark_proto.landmark.add(
                x=lm.x,
                y=lm.y,
                z=lm.z,
                visibility=lm.visibility
            )

        mp_drawing.draw_landmarks(
            result.frame,
            landmark_proto,
            mp_pose.POSE_CONNECTIONS,
            mp_drawing.DrawingSpec(color=color, thickness=2, circle_radius=2),
            mp_drawing.DrawingSpec(color=color, thickness=2)
        )

        # Label person with ID near the nose (landmark 0)
        nose = pose_landmarks[0]
        h, w, _ = result.frame.shape
        x = int(nose.x * w)
        y = int(nose.y * h)
        cv2.putText(result.frame, f"#{idx+1}", (x, y - 10),
                    cv2.FONT_HERSHEY_SIMPLEX, 1.0, color, 2, cv2.LINE_AA)

    cv2.imshow("MediaPipe Drawing", result.frame)
    cv2.waitKey(1)

#def draw_result(result):
#    if not result.result.pose_landmarks:
#        return
#
#    for pose_landmarks in result.result.pose_landmarks:
#        landmark_proto = landmark_pb2.NormalizedLandmarkList()
#        for lm in pose_landmarks:
#            landmark_proto.landmark.add(
#                x=lm.x,
#                y=lm.y,
#                z=lm.z,
#                visibility=lm.visibility
#            )
#
#        mp_drawing.draw_landmarks(
#            result.frame,
#            landmark_proto,
#            mp_pose.POSE_CONNECTIONS,
#            mp_drawing.DrawingSpec(color=(0, 255, 0), thickness=2, circle_radius=2),
#            mp_drawing.DrawingSpec(color=(255, 0, 0), thickness=2)
#        )
#
#    cv2.imshow("MediaPipe Drawing", result.frame)
#    cv2.waitKey(1)

#def draw_result(result):
#    if not result.result.pose_landmarks:
#        return
#
#    landmark_proto = landmark_pb2.NormalizedLandmarkList()
#    for lm in result.result.pose_landmarks[0]:
#        landmark_proto.landmark.add(
#            x=lm.x,
#            y=lm.y,
#            z=lm.z,
#            visibility=lm.visibility
#        )
#
#    mp_drawing.draw_landmarks(
#        result.frame,
#        landmark_proto,
#        mp_pose.POSE_CONNECTIONS,
#        mp_drawing.DrawingSpec(color=(0, 255, 0), thickness=2, circle_radius=2),
#        mp_drawing.DrawingSpec(color=(255, 0, 0), thickness=2)
#    )
#
#    cv2.imshow("MediaPipe Drawing", result.frame)
#    cv2.waitKey(1)

