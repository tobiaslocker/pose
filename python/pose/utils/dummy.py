import asyncio
import random

async def dummy_pose_generator():
    while True:
        await asyncio.sleep(0.033)  # ~30 FPS
        dummy_result = {"landmarks": [random.random() for _ in range(33 * 3)]}
        yield dummy_result

