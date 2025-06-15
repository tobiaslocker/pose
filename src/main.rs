use bevy::prelude::*;
use pose::network::forward::forward;
use pose::network::ws::FramedPayloadStream;
use pose::protocol::{DetectionResult, parse};
use pose::render::skeleton::Skeleton;
use pose::resources::Detection;
use tokio::sync::mpsc;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(),));
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<DetectionResult>(32);
    let stream = FramedPayloadStream::connect("ws://127.0.0.1:9000")
        .await
        .expect("WebSocket connection failed");

    tokio::spawn(forward(stream, tx, parse));

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Detection::from_channel(rx))
        .add_systems(Startup, (setup, Skeleton::setup))
        .add_systems(Update, Detection::system_update)
        .add_systems(Update, Skeleton::position_update)
        .run();
}
