#[allow(warnings)]
#[path = "../generated/rust/pose_generated.rs"]
mod pose_generated;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioPlugin};
use futures_util::SinkExt;
use futures_util::StreamExt;
use pose_generated::detection;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::process::{Child, Command, Stdio};
use std::{net::TcpStream, thread, time::Duration};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

const SKELETON_CONNECTIONS: &[(usize, usize)] = &[
    // Arms
    (11, 13), // left shoulder → elbow
    (13, 15), // left elbow → wrist
    (12, 14), // right shoulder → elbow
    (14, 16), // right elbow → wrist
    // Legs
    (23, 25), // left hip → knee
    (25, 27), // left knee → ankle
    (24, 26), // right hip → knee
    (26, 28), // right knee → ankle
    // Torso
    (11, 12), // left shoulder ↔ right shoulder
    (23, 24), // left hip ↔ right hip
    (11, 23), // left shoulder ↔ left hip
    (12, 24), // right shoulder ↔ right hip
];

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("assets/energia-de-gostosa.json")?;
    let reader = std::io::BufReader::new(file);
    let sequence: Sequence = serde_json::from_reader(reader)?;
    let (tx, rx) = mpsc::channel::<LandmarkFrame>(32);

    let mut controller_process = start_inference()?;

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(websocket_task(tx));
    });
    let frames = VecDeque::new();
    let recorded_sequence = RecordedSequence { frames };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin)
        .insert_resource(sequence)
        .insert_resource(recorded_sequence)
        .insert_resource(LandmarkFrameReceiver(rx))
        .add_systems(Startup, setup)
        .add_systems(Update, file_stream)
        .add_systems(Update, ws_stream)
        .add_systems(Update, draw_character)
        .add_systems(Update, record_live_frames)
        //.add_systems(Update, score_pose_similarity)
        //.add_systems(Update, update_score_text)
        .run();
    let _ = controller_process.kill();
    Ok(())
}

fn record_live_frames(
    mut buffer: ResMut<RecordedSequence>,
    audio_instances: Res<Assets<AudioInstance>>,
    audio_handle: Res<AudioHandle>,
    query: Query<&LatestLandmarkFrame, With<Playable>>,
    ref_query: Query<&LatestLandmarkFrame, (With<NonPlayable>, Without<Playable>)>,
    mut has_saved: Local<bool>,
) {
    let Some(instance) = audio_instances.get(&audio_handle.0) else {
        return;
    };

    match instance.state() {
        bevy_kira_audio::PlaybackState::Playing { .. } => {
            if let (Ok(frame), Ok(ref_frame)) = (query.single(), ref_query.single()) {
                if let (Some(live), Some(recorded)) = (&frame.0, &ref_frame.0) {
                    if buffer
                        .frames
                        .back()
                        .map_or(true, |last| last.timestamp != recorded.timestamp)
                    {
                        let mut live_clone = live.clone();
                        live_clone.timestamp = recorded.timestamp;
                        buffer.frames.push_back(live_clone);
                        println!("recorded {} frames", buffer.frames.len());
                    }
                }
            }
        }

        bevy_kira_audio::PlaybackState::Stopped => {
            if !*has_saved && !buffer.frames.is_empty() {
                *has_saved = true;
                println!("Audio finished. Saving {} frames...", buffer.frames.len());

                let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
                let filename = format!("recorded-choreo-{}.json", timestamp);
                let path = std::path::Path::new("recordings").join(filename);

                if let Err(e) = std::fs::create_dir_all("recordings") {
                    eprintln!("❌ Failed to create directory: {e}");
                } else {
                    match std::fs::File::create(&path) {
                        Ok(f) => {
                            if let Err(e) = serde_json::to_writer_pretty(f, &*buffer) {
                                eprintln!("❌ Failed to write JSON: {e}");
                            } else {
                                println!("✅ Recording saved to {}", path.display());
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to create file: {e}"),
                    }
                }
            }
        }
        _ => {}
    }
}

#[derive(Component)]
struct ScoreText;

fn score_pose_similarity(
    mut live_query: Query<(&LatestLandmarkFrame, &mut PoseScore), With<Playable>>,
    ref_query: Query<&LatestLandmarkFrame, (With<NonPlayable>, Without<Playable>)>,
) {
}

#[derive(Component, Debug)]
pub struct PoseScore(pub f32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Availability {
    pub visibility: f32,
    pub presence: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Landmark {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub availability: Option<Availability>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LandmarkFrame {
    pub landmarks: Vec<Landmark>,
    pub timestamp: f64,
}

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct RecordedSequence {
    pub frames: VecDeque<LandmarkFrame>,
}

#[derive(Resource, Debug, Deserialize)]
pub struct Sequence {
    pub frames: VecDeque<LandmarkFrame>,
}

#[derive(Resource)]
struct AudioHandle(Handle<AudioInstance>);

#[derive(Component, Debug)]
pub struct LatestLandmarkFrame(pub Option<LandmarkFrame>);

#[derive(Component)]
struct Playable;

#[derive(Component)]
struct NonPlayable;

#[derive(Resource)]
struct LandmarkFrameReceiver(mpsc::Receiver<LandmarkFrame>);

fn start_inference() -> std::io::Result<Child> {
    let controller_path = "/Users/tobiaslocker/dev/github.com/tobiaslocker/pose";
    let child = Command::new("poetry")
        .arg("--directory")
        .arg(format!("{}/python", controller_path))
        .arg("run")
        .arg("python")
        .arg("pose/run_server.py")
        .arg("--model")
        .arg(format!(
            "{}/models/pose_landmarker_lite.task",
            controller_path
        ))
        .arg("--show-preview")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();
    let addr = format!("localhost:9000");
    let start = std::time::Instant::now();

    while start.elapsed().as_secs() < 5 {
        if TcpStream::connect(&addr).is_ok() {
            return child;
        }
        thread::sleep(Duration::from_millis(200));
    }
    let _ = child.unwrap().kill();

    Err(std::io::Error::new(
        std::io::ErrorKind::TimedOut,
        "Inference server did not start in time",
    ))
}

fn ws_stream(
    mut receiver: ResMut<LandmarkFrameReceiver>,
    mut query: Query<&mut LatestLandmarkFrame, With<Playable>>,
) {
    if let Ok(frame) = receiver.0.try_recv() {
        if let Ok(mut current) = query.single_mut() {
            current.0 = Some(frame);
        }
    }
}

async fn websocket_task(tx: tokio::sync::mpsc::Sender<LandmarkFrame>) {
    let (ws_stream, _) = connect_async("ws://localhost:9000").await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    loop {
        let data = match read.next().await {
            Some(Ok(Message::Binary(data))) => Some(data),
            Some(Ok(Message::Ping(payload))) => {
                if let Err(e) = write.send(Message::Pong(payload)).await {
                    eprintln!("Failed to send Pong: {}", e);
                    break;
                }
                continue;
            }
            Some(Ok(Message::Pong(_))) => {
                eprintln!("Received Pong");
                continue;
            }
            Some(Ok(_)) => continue,
            Some(Err(e)) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
            None => {
                eprintln!("WebSocket stream closed");
                break;
            }
        };

        if let Some(data) = data {
            if let Some(frame) = parse(&data) {
                let _ = tx.send(frame).await;
            }
        }
    }
}

fn parse(buf: &[u8]) -> Option<LandmarkFrame> {
    let msg = flatbuffers::root::<detection::DetectionMessage>(buf).ok()?;

    match msg.payload_type() {
        detection::DetectionPayload::PoseDetectionResult => {
            let pose_result = msg.payload_as_pose_detection_result()?;
            let landmarks_fb = pose_result.landmarks().unwrap_or_default();

            let landmarks: Vec<Landmark> = landmarks_fb
                .iter()
                .map(|lm| Landmark {
                    x: lm.x(),
                    y: lm.y(),
                    z: lm.z(),
                    availability: lm.availability().map(|avail| Availability {
                        visibility: avail.visibility(),
                        presence: avail.presence(),
                    }),
                })
                .collect();

            let timestamp = pose_result.timestamp();
            Some(LandmarkFrame {
                landmarks,
                timestamp,
            })
        }

        detection::DetectionPayload::Empty => {
            eprintln!("Empty payload received — skipping frame.");
            None
        }

        other => {
            eprintln!("Unknown payload type {:?} — skipping frame.", other);
            None
        }
    }
}

fn setup(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    let sound = asset_server.load("energia.ogg");
    let handle = audio.play(sound).handle();
    commands.insert_resource(AudioHandle(handle));
    commands.spawn(Camera2d::default());
    commands.spawn((
        Name::new("Live"),
        Playable,
        LatestLandmarkFrame(None),
        PoseScore(0.0),
    ));
    commands.spawn((
        Name::new("Sequence"),
        NonPlayable,
        LatestLandmarkFrame(None),
    ));
    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        ScoreText,
    ));
}

fn update_score_text(
    query_score: Query<&PoseScore, With<Playable>>,
    mut query_text: Query<&mut Text, With<ScoreText>>,
) {
    if let (Ok(score), Ok(mut text)) = (query_score.single(), query_text.single_mut()) {
        text.0 = format!("Score: {:.1}", score.0);
    }
}

fn draw_character(
    query: Query<(
        &LatestLandmarkFrame,
        Option<&Playable>,
        Option<&NonPlayable>,
    )>,
    mut gizmos: Gizmos,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let window_size = Vec2::new(window.width(), window.height());

    for (frame_wrapper, is_playable, is_non_playable) in query.iter() {
        let Some(frame) = &frame_wrapper.0 else {
            continue;
        };

        let color = if is_playable.is_some() {
            Color::srgb(1.0, 0.0, 0.0)
        } else if is_non_playable.is_some() {
            Color::srgb(0.0, 1.0, 0.0)
        } else {
            Color::srgb(0.0, 0.0, 1.0)
        };

        for lm in &frame.landmarks {
            let x = (lm.x - 0.5) * window_size.x;
            let y = (0.5 - lm.y) * window_size.y;
            let position = Vec3::new(x, y, 0.0);
            gizmos.circle_2d(position.truncate(), 5.0, color);
        }
        for &(a_idx, b_idx) in SKELETON_CONNECTIONS {
            if let (Some(a), Some(b)) = (frame.landmarks.get(a_idx), frame.landmarks.get(b_idx)) {
                let ax = (a.x - 0.5) * window_size.x;
                let ay = (0.5 - a.y) * window_size.y;
                let bx = (b.x - 0.5) * window_size.x;
                let by = (0.5 - b.y) * window_size.y;

                let a_pos = Vec3::new(ax, ay, 0.0);
                let b_pos = Vec3::new(bx, by, 0.0);

                gizmos.line(a_pos, b_pos, Color::WHITE);
            }
        }
    }
}

fn file_stream(
    audio_instances: Res<Assets<AudioInstance>>,
    audio_handle: Res<AudioHandle>,
    mut sequence: ResMut<Sequence>,
    mut query: Query<&mut LatestLandmarkFrame, With<NonPlayable>>,
) {
    let Some(instance) = audio_instances.get(&audio_handle.0) else {
        return;
    };

    let Ok(mut current_frame) = query.single_mut() else {
        println!("No character entity with CurrentLandmarkFrame");
        return;
    };

    match instance.state() {
        bevy_kira_audio::PlaybackState::Playing { position } => {
            while let Some(frame) = sequence.frames.front() {
                if frame.timestamp / 1000.0 <= position {
                    let frame = sequence.frames.pop_front().unwrap();
                    current_frame.0 = Some(frame);
                } else {
                    break;
                }
            }
        }
        other_state => {
            println!("Audio not playing: {:?}", other_state);
        }
    }
}
