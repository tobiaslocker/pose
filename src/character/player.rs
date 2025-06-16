use crate::character::Character;
use crate::landmark::{Index, Stream};
use crate::network::forward::forward;
use crate::network::ws::FramedPayloadStream;
use crate::protocol::LandmarkFrame;
use crate::protocol::parse;

use bevy::prelude::*;
use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use tokio::sync::mpsc;

#[derive(Resource)]
pub struct Player {
    pub character: Character,
    pub stream: Stream,
}

impl Player {
    pub fn new(commands: &mut Commands, runtime: &TokioTasksRuntime) -> Self {
        let character = Character::new();
        character.spawn(commands);

        let (tx, rx) = mpsc::channel::<LandmarkFrame>(32);
        let stream = Stream::from_channel(rx);

        runtime.spawn_background_task(|_ctx: TaskContext| {
            Box::pin(async move {
                if let Ok(ws) = FramedPayloadStream::connect("ws://127.0.0.1:9000").await {
                    forward(ws, tx, parse).await.unwrap();
                } else {
                    error!("[Player] Failed to connect to inference WebSocket.");
                }
            })
        });

        Self { character, stream }
    }

    pub fn update(
        mut player: ResMut<Player>,
        mut query: Query<(&mut Transform, &Index, &mut Visibility)>,
        mut gizmos: Gizmos,
        windows: Query<&Window>,
    ) {
        player.stream.update();

        if let Some(frame) = player.stream.latest() {
            player
                .character
                .draw(&frame.landmarks, &mut query, &mut gizmos, windows);
        }
    }
}
