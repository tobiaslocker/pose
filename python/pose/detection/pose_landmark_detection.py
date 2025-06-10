import cv2
import time
from mediapipe.tasks.python import vision
from mediapipe.tasks import python
import mediapipe as mp
import asyncio

class PoseLandmarkDetection:
    def __init__(self, model_path='tasks/pose_landmarker_lite.task', camera_id=0, width=1280, height=960):
        base_options = python.BaseOptions(model_asset_path=model_path)
        def save_result(result, output_image, timestamp_ms):
            _ = output_image
            _ = timestamp_ms
            self.latest_result = result
        options = vision.PoseLandmarkerOptions(
            base_options=base_options,
            running_mode=vision.RunningMode.LIVE_STREAM,
            num_poses=1,
            min_pose_detection_confidence=0.5,
            min_pose_presence_confidence=0.5,
            min_tracking_confidence=0.5,
            result_callback=save_result
        )
        self.detector = vision.PoseLandmarker.create_from_options(options)
        self.cap = cv2.VideoCapture(camera_id)
        self.cap.set(cv2.CAP_PROP_FRAME_WIDTH, width)
        self.cap.set(cv2.CAP_PROP_FRAME_HEIGHT, height)

        self.latest_result = None


    async def next(self):
        success, image = self.cap.read()
        if not success:
            raise RuntimeError("Unable to read from camera")
        rgb_image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
        mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb_image)
        self.detector.detect_async(mp_image, time.time_ns() // 1_000_000)
        while self.latest_result is None:
            await asyncio.sleep(0.005)
        result = self.latest_result
        self.latest_result = None
        return result

    def to_generator(self):
        async def generator():
            while True:
                result = await self.next()
                yield result
        return generator()

