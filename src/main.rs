use bevy::prelude::*;
use pose::network::forward::forward;
use pose::network::tcp::Client;
use pose::protocol::{DetectionResult, parse};
use pose::render::skeleton::Skeleton;
use pose::resources::Detection;
use tokio::sync::mpsc;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(),));
}

#[tokio::main]
async fn main() {
    // Connect to the Python TCP server
    let mut client = Client::new();
    client
        .connect("127.0.0.1", 9000)
        .await
        .expect("Failed to connect");

    // Set up the channel for parsed messages
    let (tx, rx) = mpsc::channel::<DetectionResult>(32);

    // Spawn the forwarding task
    let stream = client.into_stream();
    tokio::spawn(forward(stream, tx, parse));

    // Build and run the Bevy app
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Detection::from_tcp(rx))
        //.insert_resource(Detection::from_mock())
        .add_systems(Startup, (setup, Skeleton::setup))
        .add_systems(Update, Detection::system_update)
        .add_systems(Update, Skeleton::position_update)
        .run();
}
