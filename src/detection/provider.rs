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
        let t = self.frame as f32 * 0.05; // animation speed

        Some(DetectionResult {
            landmarks: vec![
                // HEAD
                Landmark {
                    x: 0.5,
                    y: 0.2 + 0.02 * (t).sin(),
                    z: 0.0,
                    availability: None,
                }, // Nose
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
                // SHOULDERS
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
                // ELBOWS
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
                // WRISTS
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
                // FINGERS (static)
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
                // HIPS (small sway left/right)
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
                // KNEES (static)
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
                // ANKLES (static)
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
                // HEELS
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
                // FOOT INDEX
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

//impl DetectionProvider for MockDetectionProvider {
//    fn poll(&mut self) -> Option<DetectionResult> {
//        Some(DetectionResult {
//            landmarks: vec![
//                // HEAD
//                Landmark {
//                    x: 0.5,
//                    y: 0.2,
//                    z: 0.0,
//                    availability: None,
//                }, // Nose
//                Landmark {
//                    x: 0.48,
//                    y: 0.18,
//                    z: 0.0,
//                    availability: None,
//                }, // Left eye inner
//                Landmark {
//                    x: 0.46,
//                    y: 0.18,
//                    z: 0.0,
//                    availability: None,
//                }, // Left eye
//                Landmark {
//                    x: 0.44,
//                    y: 0.18,
//                    z: 0.0,
//                    availability: None,
//                }, // Left eye outer
//                Landmark {
//                    x: 0.52,
//                    y: 0.18,
//                    z: 0.0,
//                    availability: None,
//                }, // Right eye inner
//                Landmark {
//                    x: 0.54,
//                    y: 0.18,
//                    z: 0.0,
//                    availability: None,
//                }, // Right eye
//                Landmark {
//                    x: 0.56,
//                    y: 0.18,
//                    z: 0.0,
//                    availability: None,
//                }, // Right eye outer
//                Landmark {
//                    x: 0.44,
//                    y: 0.22,
//                    z: 0.0,
//                    availability: None,
//                }, // Left ear
//                Landmark {
//                    x: 0.56,
//                    y: 0.22,
//                    z: 0.0,
//                    availability: None,
//                }, // Right ear
//                Landmark {
//                    x: 0.46,
//                    y: 0.25,
//                    z: 0.0,
//                    availability: None,
//                }, // Mouth left
//                Landmark {
//                    x: 0.54,
//                    y: 0.25,
//                    z: 0.0,
//                    availability: None,
//                }, // Mouth right
//                // SHOULDERS
//                Landmark {
//                    x: 0.4,
//                    y: 0.35,
//                    z: 0.0,
//                    availability: None,
//                }, // Left shoulder
//                Landmark {
//                    x: 0.6,
//                    y: 0.35,
//                    z: 0.0,
//                    availability: None,
//                }, // Right shoulder
//                // ELBOWS
//                Landmark {
//                    x: 0.35,
//                    y: 0.5,
//                    z: 0.0,
//                    availability: None,
//                }, // Left elbow
//                Landmark {
//                    x: 0.65,
//                    y: 0.5,
//                    z: 0.0,
//                    availability: None,
//                }, // Right elbow
//                // WRISTS
//                Landmark {
//                    x: 0.3,
//                    y: 0.65,
//                    z: 0.0,
//                    availability: None,
//                }, // Left wrist
//                Landmark {
//                    x: 0.7,
//                    y: 0.65,
//                    z: 0.0,
//                    availability: None,
//                }, // Right wrist
//                // FINGERS (just approximate)
//                Landmark {
//                    x: 0.29,
//                    y: 0.68,
//                    z: 0.0,
//                    availability: None,
//                }, // Left pinky
//                Landmark {
//                    x: 0.71,
//                    y: 0.68,
//                    z: 0.0,
//                    availability: None,
//                }, // Right pinky
//                Landmark {
//                    x: 0.31,
//                    y: 0.68,
//                    z: 0.0,
//                    availability: None,
//                }, // Left index
//                Landmark {
//                    x: 0.69,
//                    y: 0.68,
//                    z: 0.0,
//                    availability: None,
//                }, // Right index
//                Landmark {
//                    x: 0.32,
//                    y: 0.67,
//                    z: 0.0,
//                    availability: None,
//                }, // Left thumb
//                Landmark {
//                    x: 0.68,
//                    y: 0.67,
//                    z: 0.0,
//                    availability: None,
//                }, // Right thumb
//                // HIPS
//                Landmark {
//                    x: 0.45,
//                    y: 0.6,
//                    z: 0.0,
//                    availability: None,
//                }, // Left hip
//                Landmark {
//                    x: 0.55,
//                    y: 0.6,
//                    z: 0.0,
//                    availability: None,
//                }, // Right hip
//                // KNEES
//                Landmark {
//                    x: 0.45,
//                    y: 0.8,
//                    z: 0.0,
//                    availability: None,
//                }, // Left knee
//                Landmark {
//                    x: 0.55,
//                    y: 0.8,
//                    z: 0.0,
//                    availability: None,
//                }, // Right knee
//                // ANKLES
//                Landmark {
//                    x: 0.45,
//                    y: 0.95,
//                    z: 0.0,
//                    availability: None,
//                }, // Left ankle
//                Landmark {
//                    x: 0.55,
//                    y: 0.95,
//                    z: 0.0,
//                    availability: None,
//                }, // Right ankle
//                // HEELS
//                Landmark {
//                    x: 0.44,
//                    y: 0.97,
//                    z: 0.0,
//                    availability: None,
//                }, // Left heel
//                Landmark {
//                    x: 0.56,
//                    y: 0.97,
//                    z: 0.0,
//                    availability: None,
//                }, // Right heel
//                // FOOT INDEX
//                Landmark {
//                    x: 0.43,
//                    y: 0.98,
//                    z: 0.0,
//                    availability: None,
//                }, // Left foot index
//                Landmark {
//                    x: 0.57,
//                    y: 0.98,
//                    z: 0.0,
//                    availability: None,
//                }, // Right foot index
//            ],
//        })
//    }
//
//    //fn poll(&mut self) -> Option<DetectionResult> {
//    //    Some(DetectionResult {
//    //        landmarks: (0..33)
//    //            .map(|i| Landmark {
//    //                x: (i as f32) / 33.0,
//    //                y: ((i as f32).sin() + 1.0) / 2.0,
//    //                z: 0.0,
//    //                availability: None,
//    //            })
//    //            .collect(),
//    //    })
//    //}
//}
//
//// Re-export known implementations
////pub use crate::detection::provider::MockDetectionProvider;
pub use crate::network::tcp::TcpDetectionProvider;
