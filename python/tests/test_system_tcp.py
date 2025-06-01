import asyncio
import struct
import pytest
import flatbuffers
from Detection import DetectionMessage, DetectionPayload, PoseDetectionResult, Landmark, Availability
from pose.network.tcp.server import Server


@pytest.mark.asyncio
async def test_server_sends_flatbuffers_message():
    queue = asyncio.Queue()
    server = Server(host="127.0.0.1", port=9010, queue=queue)
    start_task = asyncio.create_task(server.run())
    await asyncio.sleep(0.1) 
    reader, writer = await asyncio.open_connection("127.0.0.1", 9010)

    builder = flatbuffers.Builder(1024)
    Availability.AvailabilityStart(builder)
    Availability.AvailabilityAddVisibility(builder, 0.9)
    Availability.AvailabilityAddPresence(builder, 0.8)
    avail = Availability.AvailabilityEnd(builder)
    Landmark.LandmarkStart(builder)
    Landmark.LandmarkAddX(builder, 0.5)
    Landmark.LandmarkAddY(builder, 0.6)
    Landmark.LandmarkAddZ(builder, -0.1)
    Landmark.LandmarkAddAvailability(builder, avail)
    landmark = Landmark.LandmarkEnd(builder)
    PoseDetectionResult.PoseDetectionResultStartLandmarksVector(builder, 1)
    builder.PrependUOffsetTRelative(landmark)
    landmarks = builder.EndVector()
    PoseDetectionResult.PoseDetectionResultStart(builder)
    PoseDetectionResult.PoseDetectionResultAddLandmarks(builder, landmarks)
    result = PoseDetectionResult.PoseDetectionResultEnd(builder)
    DetectionMessage.DetectionMessageStart(builder)
    DetectionMessage.DetectionMessageAddPayloadType(builder, DetectionPayload.DetectionPayload().PoseDetectionResult)
    DetectionMessage.DetectionMessageAddPayload(builder, result)
    message = DetectionMessage.DetectionMessageEnd(builder)

    builder.Finish(message)
    test_data = bytes(builder.Output())
    await queue.put(test_data)

    data = await reader.read(len(test_data))
    assert data == test_data
    await queue.put(None)
    await asyncio.sleep(0.1)

    writer.close()
    await writer.wait_closed()

    start_task.cancel()
    try:
        await start_task
    except asyncio.CancelledError:
        pass

    msg = DetectionMessage.DetectionMessage.GetRootAsDetectionMessage(data, 0)
    assert msg.PayloadType() == DetectionPayload.DetectionPayload().PoseDetectionResult

    payload = msg.Payload()
    assert payload is not None, "Payload is missing"

    pose = PoseDetectionResult.PoseDetectionResult()
    pose.Init(payload.Bytes, payload.Pos)

    assert pose.LandmarksLength() == 1
    lm = pose.Landmarks(0)
    assert lm is not None, "Landmark 0 missing"
    assert abs(lm.X() - 0.5) < 1e-6
    assert abs(lm.Y() - 0.6) < 1e-6
    assert abs(lm.Z() + 0.1) < 1e-6
    availability = lm.Availability()
    assert availability is not None
    assert abs(availability.Visibility() - 0.9) < 1e-6
    assert abs(availability.Presence() - 0.8) < 1e-6


@pytest.mark.asyncio
async def test_server_runs_and_sends_data():
    queue = asyncio.Queue()
    server = Server(host="127.0.0.1", port=9010, queue=queue)
    start_task = asyncio.create_task(server.run())
    await asyncio.sleep(0.1)  # Give server time to start
    reader, writer = await asyncio.open_connection("127.0.0.1", 9010)

    test_data = b"Hello, client!"
    await queue.put(test_data)

    data = await reader.read(len(test_data))
    assert data == test_data
    await queue.put(None)
    await asyncio.sleep(0.1)

    writer.close()
    await writer.wait_closed()

    start_task.cancel()
    try:
        await start_task
    except asyncio.CancelledError:
        pass

