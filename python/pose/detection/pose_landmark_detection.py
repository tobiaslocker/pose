import time
import cv2
import mediapipe as mp
import asyncio
from mediapipe.tasks import python
from mediapipe.tasks.python import vision
from mediapipe.tasks.python.vision.pose_landmarker import PoseLandmarkerResult
from dataclasses import dataclass

@dataclass
class Result:
    timestamp: float
    result: PoseLandmarkerResult


class PoseLandmarkDetection:
    def __init__(self, video_capture, model_asset_path):
        base_options = python.BaseOptions(model_asset_path=model_asset_path)

        def save_result(result, output_image, timestamp_ms):
            _ = output_image
            _ = timestamp_ms
            self.latest_result = result

        options = vision.PoseLandmarkerOptions(
            base_options=base_options,
            running_mode=vision.RunningMode.LIVE_STREAM,
            num_poses=2,
            min_pose_detection_confidence=0.5,
            min_pose_presence_confidence=0.5,
            min_tracking_confidence=0.5,
            result_callback=save_result
        )

        self.cap = video_capture
        self.detector = vision.PoseLandmarker.create_from_options(options)
        self.latest_result = None

    @classmethod
    def fromfilename(cls, filename, model_asset_path):
        cap = cv2.VideoCapture(filename)
        if not cap.isOpened():
            raise RuntimeError(f"Failed to open video file: {filename}")
        return cls(cap, model_asset_path)

    @classmethod
    def fromindex(cls, index,  model_asset_path, width=1280, height=960):
        cap = cv2.VideoCapture(index)
        cap.set(cv2.CAP_PROP_FRAME_WIDTH, width)
        cap.set(cv2.CAP_PROP_FRAME_HEIGHT, height)
        return cls(cap, model_asset_path)

    async def next(self):
        success, frame = self.cap.read()
        if not success:
            return None
        frame = cv2.flip(frame, 1)

        rgb_image = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb_image)

        self.detector.detect_async(mp_image, time.time_ns() // 1_000_000)

        while self.latest_result is None:
            await asyncio.sleep(0.005)

        result = self.latest_result
        self.latest_result = None
        return Result(self.cap.get(cv2.CAP_PROP_POS_MSEC), result)

    def to_generator(self):
        async def generator():
            while True:
                result = await self.next()
                if result is None:
                    break
                yield result

        return generator()

