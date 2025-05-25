// import the flatbuffers runtime library
extern crate flatbuffers;

// import the generated code
#[allow(dead_code, unused_imports)]
//#[path = "../generated/pose_generated.rs"]
mod generated;

use generated::pose_generated::pose::PoseFrame;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;
    println!("Listening on port 9000...");

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Client connected");

        tokio::spawn(async move {
            loop {
                // Read 4-byte length prefix
                let mut size_buf = [0u8; 4];
                if let Err(e) = socket.read_exact(&mut size_buf).await {
                    eprintln!("Failed to read length: {}", e);
                    break; // connection probably closed, exit loop
                }

                let size = u32::from_be_bytes(size_buf) as usize;
                let mut buffer = vec![0u8; size];

                // Read the actual payload of `size` bytes
                if let Err(e) = socket.read_exact(&mut buffer).await {
                    eprintln!("Failed to read data: {}", e);
                    break;
                }

                // Deserialize the pose frame
                match flatbuffers::root::<PoseFrame>(&buffer) {
                    Ok(pose) => {
                        if let Some(keypoints) = pose.keypoints() {
                            for (i, kp) in keypoints.iter().enumerate() {
                                println!(
                                    "Keypoint {}: x = {:.2}, y = {:.2}, conf = {:.2}",
                                    i,
                                    kp.x(),
                                    kp.y(),
                                    kp.confidence()
                                );
                            }
                        } else {
                            eprintln!("No keypoints found");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse pose frame: {:?}", e);
                    }
                }
            }

            println!("Client disconnected");
        });
    }
}
