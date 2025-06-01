# tests/test_system_tcp.py

import asyncio
import struct
import pytest

import flatbuffers
from Detection import DetectionMessage, DetectionPayload, PoseDetectionResult, Landmark, Availability
from pose.network.tcp.server import Server

def build_test_message() -> bytes:
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
    return bytes(builder.Output())

@pytest.mark.asyncio
async def test_tcp_roundtrip():
    HOST = "127.0.0.1"
    PORT = 9010
    queue = asyncio.Queue()
    received_data = []

    server = Server(host=HOST, port=PORT)

    # Start server
    start_task = asyncio.create_task(server.start())

    # Start client
    async def run_client():
        reader, writer = await asyncio.open_connection(HOST, PORT)

        len_prefix = await reader.readexactly(4)
        msg_len = struct.unpack('>I', len_prefix)[0]
        buf = await reader.readexactly(msg_len)
        received_data.append(buf)

        writer.close()
        await writer.wait_closed()

    client_task = asyncio.create_task(run_client())

    # Wait for server to accept connection
    await start_task

    # Start send_loop
    send_loop_task = asyncio.create_task(server.send_loop(queue))

    # Enqueue test message
    msg = build_test_message()
    framed = struct.pack('>I', len(msg)) + msg
    await queue.put(framed)

    # Wait for client to finish
    await client_task

    # Send poison pill to end send_loop
    await queue.put(None)
    await send_loop_task

    # Shutdown server
    await server.shutdown()

    # Assertions
    assert len(received_data) == 1
    buf = received_data[0]

    msg = DetectionMessage.DetectionMessage.GetRootAsDetectionMessage(buf, 0)
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

    print("✅ TCP system test passed.")
