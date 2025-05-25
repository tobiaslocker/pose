extern crate flatbuffers;

use bevy::prelude::*;
use std::thread;
use tokio::sync::mpsc;

mod generated;
use generated::pose_generated::pose::PoseFrame;

#[derive(Clone, Copy, Debug)]
pub struct Keypoint {
    pub position: Vec3,
    pub confidence: f32,
}

#[derive(Resource, Default, Clone)]
pub struct Keypoints(Vec<Keypoint>);

#[derive(Component)]
pub struct KeypointMarker(usize);

#[derive(Resource)]
pub struct PoseReceiver(pub mpsc::Receiver<Vec<Keypoint>>);

pub const SKELETON: &[(usize, usize)] = &[
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 7), // Face - right
    (0, 4),
    (4, 5),
    (5, 6),
    (6, 8),   // Face - left
    (9, 10),  // Mouth
    (11, 12), // Shoulders
    (11, 13),
    (13, 15), // Left Arm
    (15, 17),
    (15, 19),
    (15, 21), // Left Hand
    (17, 19),
    (19, 21), // Left Hand continued
    (12, 14),
    (14, 16), // Right Arm
    (16, 18),
    (16, 20),
    (16, 22), // Right Hand
    (18, 20),
    (20, 22), // Right Hand continued
    (11, 23),
    (12, 24), // Torso sides
    (23, 24), // Hips
    (23, 25),
    (25, 27), // Left Leg
    (27, 29),
    (29, 31), // Left Foot
    (24, 26),
    (26, 28), // Right Leg
    (28, 30),
    (30, 32), // Right Foot
];

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(),));
    for i in 0..33 {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(16.0)),
                ..default()
            },
            Transform::default(),
            GlobalTransform::default(),
            KeypointMarker(i),
        ));
    }
}

/// Only updates Keypoints resource from the receiver
pub fn receive_keypoints(mut receiver: ResMut<PoseReceiver>, mut keypoints: ResMut<Keypoints>) {
    while let Ok(new_kps) = receiver.0.try_recv() {
        let width = 1920.0;
        let height = 1080.0;

        println!("{:?}", keypoints.0);

        keypoints.0 = new_kps
            .iter()
            .map(|kp| Keypoint {
                position: Vec3::new(
                    (kp.position.x - 0.5) * width,
                    -(kp.position.y - 0.5) * height,
                    kp.position.z * 1000.0,
                ),
                confidence: kp.confidence,
            })
            .collect();
    }
}

/// Always runs, even if keypoints weren't updated this frame
pub fn update_keypoint_transforms(
    keypoints: Res<Keypoints>,
    mut query: Query<(&mut Transform, &KeypointMarker)>,
) {
    for (mut transform, KeypointMarker(i)) in query.iter_mut() {
        if let Some(kp) = keypoints.0.get(*i) {
            transform.translation = kp.position;
        }
    }
}

/// Receives Keypoints from the given Receiver and updates resources so they can be rendered.
//pub fn receive_and_update_keypoints(
//    mut receiver: ResMut<PoseReceiver>,
//    mut keypoints: ResMut<Keypoints>,
//    mut query: Query<(&mut Transform, &KeypointMarker)>,
//) {
//    while let Ok(new_kps) = receiver.0.try_recv() {
//        let width = 1920.0;
//        let height = 1080.0;
//
//        keypoints.0 = new_kps
//            .iter()
//            .map(|kp| Keypoint {
//                position: Vec3::new(
//                    (kp.position.x - 0.5) * width,
//                    -(kp.position.y - 0.5) * height,
//                    kp.position.z * 1000.0,
//                ),
//                confidence: kp.confidence,
//            })
//            .collect();
//
//        for (mut transform, KeypointMarker(i)) in query.iter_mut() {
//            if let Some(kp) = keypoints.0.get(*i) {
//                transform.translation = kp.position;
//            }
//        }
//    }
//}

pub fn draw_skeleton(keypoints: Res<Keypoints>, mut gizmos: Gizmos) {
    for &(a, b) in SKELETON {
        if let (Some(p1), Some(p2)) = (keypoints.0.get(a), keypoints.0.get(b)) {
            let min_conf = p1.confidence.min(p2.confidence);
            let color = Color::srgba(0.0, 1.0, 0.0, min_conf.clamp(0.1, 1.0));
            gizmos.line(p1.position, p2.position, color);
        }
    }
}

pub fn spawn_pose_receiver(tx: mpsc::Sender<Vec<Keypoint>>) {
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            use tokio::io::AsyncReadExt;
            use tokio::net::TcpListener;

            let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
            println!("Listening on 127.0.0.1:9000");

            while let Ok((mut socket, _)) = listener.accept().await {
                println!("Client connected");
                let tx = tx.clone();

                tokio::spawn(async move {
                    loop {
                        let mut size_buf = [0u8; 4];
                        if socket.read_exact(&mut size_buf).await.is_err() {
                            break;
                        }

                        let size = u32::from_be_bytes(size_buf) as usize;
                        let mut buffer = vec![0u8; size];
                        if socket.read_exact(&mut buffer).await.is_err() {
                            break;
                        }

                        if let Ok(pose) = flatbuffers::root::<PoseFrame>(&buffer) {
                            if let Some(kps) = pose.keypoints() {
                                let vec3s = kps
                                    .iter()
                                    .map(|kp| Keypoint {
                                        position: Vec3::new(kp.x(), kp.y(), kp.z()),
                                        confidence: kp.confidence(),
                                    })
                                    .collect::<Vec<_>>();
                                let _ = tx.send(vec3s).await;
                            }
                        }
                    }
                });
            }
        });
    });
}
