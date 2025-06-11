use crate::protocol::{DetectionResult, Landmark};

/// A pluggable interface for any source of pose/landmark detection data.
pub trait DetectionProvider: Send + Sync {
    fn poll(&mut self) -> Option<DetectionResult>;
}

pub struct MockDetectionProvider {
    frame: usize,
}

impl MockDetectionProvider {
    pub fn new() -> Self {
        Self { frame: 0 }
    }
}

impl DetectionProvider for MockDetectionProvider {
    fn poll(&mut self) -> Option<DetectionResult> {
        self.frame += 1;
        let t = self.frame as f32 * 0.05;

        Some(DetectionResult {
            landmarks: vec![
                Landmark {
                    x: 0.5,
                    y: 0.2 + 0.02 * (t).sin(),
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.48,
                    y: 0.18,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.46,
                    y: 0.18,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.44,
                    y: 0.18,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.52,
                    y: 0.18,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.54,
                    y: 0.18,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.56,
                    y: 0.18,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.44,
                    y: 0.22,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.56,
                    y: 0.22,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.46,
                    y: 0.25,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.54,
                    y: 0.25,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.4 + 0.01 * (t * 0.8).cos(),
                    y: 0.35,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.6 + 0.01 * (t * 0.8).cos(),
                    y: 0.35,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.35 + 0.02 * (t * 0.6).sin(),
                    y: 0.5,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.65 + 0.02 * (t * 0.6).sin(),
                    y: 0.5,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.3 + 0.03 * (t * 0.6).sin(),
                    y: 0.65,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.7 + 0.03 * (t * 0.6).sin(),
                    y: 0.65,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.29,
                    y: 0.68,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.71,
                    y: 0.68,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.31,
                    y: 0.68,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.69,
                    y: 0.68,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.32,
                    y: 0.67,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.68,
                    y: 0.67,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.45 + 0.005 * (t * 0.5).sin(),
                    y: 0.6,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.55 + 0.005 * (t * 0.5).sin(),
                    y: 0.6,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.45,
                    y: 0.8,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.55,
                    y: 0.8,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.45,
                    y: 0.95,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.55,
                    y: 0.95,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.44,
                    y: 0.97,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.56,
                    y: 0.97,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.43,
                    y: 0.98,
                    z: 0.0,
                    availability: None,
                },
                Landmark {
                    x: 0.57,
                    y: 0.98,
                    z: 0.0,
                    availability: None,
                },
            ],
        })
    }
}

pub use crate::network::tcp::TcpDetectionProvider;
