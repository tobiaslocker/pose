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
    let mut sequences = Vec::new();
    let dir = std::fs::read_dir("assets/energia-de-gostosa").expect("Failed to read directory");

    for entry in dir {
        let path = entry.expect("Failed to read entry").path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file = std::fs::File::open(&path).expect("Failed to open JSON file");
            let reader = std::io::BufReader::new(file);
            let sequence: Sequence = serde_json::from_reader(reader).expect("Failed to parse JSON");
            sequences.push(sequence);
        }
    }

    let (tx, rx) = mpsc::channel::<LandmarkFrame>(32);
    let mut controller_process = start_inference()?;

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(websocket_task(tx));
    });

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AudioPlugin)
        .insert_resource(Sequences { sequences })
        .insert_resource(LandmarkFrameReceiver(rx))
        .add_systems(Startup, setup)
        .add_systems(Update, file_stream)
        .add_systems(Update, ws_stream)
        .add_systems(Update, draw_character)
        .add_systems(Update, score_pose_similarity)
        //.add_systems(Update, update_score_text)
        .run();
    let _ = controller_process.kill();
    Ok(())
}

#[derive(Component)]
struct ScoreText;

//fn update_score_text(
//    query_score: Query<&PoseScore, With<Playable>>,
//    mut query_text: Query<&mut Text, With<ScoreText>>,
//) {
//    if let (Ok(score), Ok(mut text)) = (query_score.single(), query_text.single_mut()) {
//        text.0 = format!("Score: {:.1}", score.0);
//    }
//}

fn score_pose_similarity(
    live_query: Query<(&LatestLandmarkFrame, &mut PoseScore), With<Playable>>,
    ref_query: Query<
        (&LatestLandmarkFrame, Entity),
        (With<NonPlayable>, Changed<LatestLandmarkFrame>),
    >,
    time: Res<Time>,
) {
    let Ok((live_frame, _score)) = live_query.single() else {
        return;
    };
    let Some(live) = &live_frame.0 else {
        return;
    };

    for (ref_frame, entity) in ref_query.iter() {
        if let Some(reference) = &ref_frame.0 {
            // Here you would compute a score between `live` and `reference`
            println!(
                "NPC {:?} frame updated at t={}",
                entity,
                time.elapsed_secs_f64()
            );
            // println!("Score vs {:?} = {:.2}", entity, score);
        }
    }
}
//fn score_pose_similarity(
//    mut live_query: Query<(&LatestLandmarkFrame, &mut PoseScore), With<Playable>>,
//    ref_query: Query<&LatestLandmarkFrame, (With<NonPlayable>, Changed<LatestLandmarkFrame>)>,
//    time: Res<Time>,
//    mut last_update: Local<Option<f64>>,
//) {
//    if let Ok(ref_frame) = ref_query.single() {
//        println!("Here-------");
//    }
//
//    if let (Ok((live_frame, mut _score)), Ok(ref_frame)) =
//        (live_query.single_mut(), ref_query.single())
//    {
//        println!("Here");
//        if let (Some(live), Some(reference)) = (&live_frame.0, &ref_frame.0) {
//            let now = time.elapsed_secs_f64();
//            if let Some(last) = *last_update {
//                let delta = now - last;
//                println!("⏱️ NPC frame delta: {:.4} sec", delta);
//            }
//            *last_update = Some(now);
//        }
//    }
//}

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
}

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct RecordedSequence {
    pub frames: VecDeque<LandmarkFrame>,
}

#[derive(Resource, Debug, Deserialize)]
pub struct Sequences {
    pub sequences: Vec<Sequence>,
}

#[derive(Resource, Debug, Deserialize)]
pub struct Sequence {
    pub frames: VecDeque<LandmarkFrame>,
    pub fps: f64,
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
        .arg("pose_tool.py")
        .arg("serve")
        .arg("--model")
        .arg(format!(
            "{}/models/pose_landmarker_lite.task",
            controller_path
        ))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    let addr = "localhost:9000";
    let start = std::time::Instant::now();

    while start.elapsed().as_secs() < 5 {
        if TcpStream::connect(addr).is_ok() {
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

            Some(LandmarkFrame { landmarks })
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
    let sound = asset_server.load("energia-de-gostosa/energia-de-gostosa.ogg");
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
    for i in 0..2 {
        commands.spawn((
            Name::new(format!("NPC {}", i + 1)),
            NonPlayable,
            LatestLandmarkFrame(None),
        ));
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
    sequences: ResMut<Sequences>,
    mut query: Query<&mut LatestLandmarkFrame, With<NonPlayable>>,
) {
    let Some(instance) = audio_instances.get(&audio_handle.0) else {
        return;
    };
    let mut npcs: Vec<_> = query.iter_mut().collect();
    if npcs.len() != sequences.sequences.len() {
        eprintln!(
            "Mismatch: {} sequences for {} NPCs",
            sequences.sequences.len(),
            npcs.len()
        );
        return;
    }

    match instance.state() {
        bevy_kira_audio::PlaybackState::Playing { position } => {
            for (i, sequence) in sequences.sequences.iter().enumerate() {
                let expected_frame_index = (position * sequence.fps) as usize;
                if let Some(frame) = sequence.frames.get(expected_frame_index) {
                    npcs[i].0 = Some(frame.clone());
                }
            }
        }
        _ => {}
    }
}
