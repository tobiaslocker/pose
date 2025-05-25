use bevy::prelude::*;
use pose_server::*;

fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel(10);
    spawn_pose_receiver(tx);

    App::new()
        .insert_resource(Keypoints::default())
        .insert_resource(PoseReceiver(rx))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Live Pose Viewer".into(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        //.add_systems(Update, receive_and_update_keypoints)
        .add_systems(
            Update,
            (receive_keypoints, update_keypoint_transforms, draw_skeleton),
        )
        .run();
}
