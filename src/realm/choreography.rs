use crate::character::Character;
use crate::landmark::{Index, Stream};
use crate::protocol::LandmarkFrame;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};
use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Sender, channel};
use tokio::time::{Duration, Instant, sleep_until};

#[derive(Serialize, Deserialize, Clone)]
pub struct Recording {
    pub frames: Vec<LandmarkFrame>,
}

impl Recording {
    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let recording = serde_json::from_reader(reader)?;
        Ok(recording)
    }
}

#[derive(Resource)]
pub struct Choreography {
    pub name: String,
    pub audio: Handle<AudioSource>,
    pub recording: Recording,
    pub character: Character,
    pub state: PlaybackState,
}

pub enum PlaybackState {
    Loading,
    Ready(Sender<LandmarkFrame>),
    Playing,
}

async fn playback(recording: Recording, sender: Sender<LandmarkFrame>) {
    let start_time = Instant::now();

    for frame in recording.frames {
        let target_time = start_time + Duration::from_micros((frame.timestamp * 1000.0) as u64);
        sleep_until(target_time.into()).await;

        if sender.send(frame).await.is_err() {
            break;
        }
    }

    info!("[Choreography] Playback finished.");
}

impl Choreography {
    pub fn start(mut commands: Commands, asset_server: Res<AssetServer>) {
        let audio = asset_server.load("choreography/energia-de-gostosa.mp3");
        let recording = Recording::load_from_file("assets/choreography/energia-de-gostosa.json")
            .expect("Failed to load recording");
        let character = Character::new();
        character.spawn(&mut commands);

        commands.insert_resource(Choreography {
            name: "energia-de-gostosa".to_string(),
            audio,
            recording,
            character,
            state: PlaybackState::Loading,
        });
    }

    pub fn update(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
        mut choreography: ResMut<Choreography>,
        runtime: Res<TokioTasksRuntime>,
        stream: Option<ResMut<Stream>>,
        mut query: Query<(&mut Transform, &Index, &mut Visibility)>,
        mut gizmos: Gizmos,
        windows: Query<&Window>,
    ) {
        let Choreography {
            audio: audio_handle,
            recording,
            state,
            ..
        } = &mut *choreography;

        match state {
            PlaybackState::Loading => {
                if asset_server.is_loaded_with_dependencies(audio_handle) {
                    let (tx, rx) = channel::<LandmarkFrame>(32);
                    commands.insert_resource(Stream::from_channel(rx));
                    *state = PlaybackState::Ready(tx);
                }
            }

            PlaybackState::Ready(tx) => {
                let audio_handle = audio_handle.clone();
                let tx = tx.clone();
                let recording = recording.clone();
                let start_time = Instant::now();

                audio.play(audio_handle);

                runtime.spawn_background_task(move |_ctx: TaskContext| {
                    Box::pin(async move {
                        let start_delay_ms = 100;
                        let elapsed = start_time.elapsed().as_millis() as u64;
                        if elapsed < start_delay_ms {
                            tokio::time::sleep(Duration::from_millis(start_delay_ms - elapsed))
                                .await;
                        }

                        playback(recording, tx).await;
                    })
                });

                *state = PlaybackState::Playing;
            }

            PlaybackState::Playing => {
                if let Some(mut stream) = stream {
                    stream.update();

                    if let Some(frame) = stream.latest() {
                        choreography.character.draw(
                            &frame.landmarks,
                            &mut query,
                            &mut gizmos,
                            windows,
                        );
                    }
                }
            }
        }
    }
}
