use crate::detection::provider::{DetectionProvider, MockDetectionProvider};
use crate::protocol::DetectionResult;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Detection {
    pub provider: Box<dyn DetectionProvider>,
    pub latest: Option<DetectionResult>,
}

impl Detection {
    pub fn update(&mut self) {
        self.latest = self.provider.poll();
    }

    pub fn latest(&self) -> Option<&DetectionResult> {
        self.latest.as_ref()
    }

    pub fn system_update(mut detection: ResMut<Self>) {
        detection.update();
    }

    pub fn from_tcp(rx: tokio::sync::mpsc::Receiver<DetectionResult>) -> Self {
        use crate::detection::provider::TcpDetectionProvider;

        Self {
            provider: Box::new(TcpDetectionProvider::new(rx)),
            latest: None,
        }
    }

    pub fn from_mock() -> Self {
        Self {
            provider: Box::new(MockDetectionProvider::new()),
            latest: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::DetectionResult;
    use crate::protocol::fbs::detection::{Availability, Landmark};

    struct DummyProvider;

    impl DetectionProvider for DummyProvider {
        fn poll(&mut self) -> Option<DetectionResult> {
            Some(DetectionResult {
                landmarks: vec![Landmark {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                    availability: Some(Availability {
                        visibility: 0.9,
                        presence: 0.8,
                    }),
                }],
            })
        }
    }

    #[test]
    fn test_detection_resource_update() {
        let mut resource = Detection {
            provider: Box::new(DummyProvider),
            latest: None,
        };

        resource.update();
        assert!(resource.latest().is_some());
        assert_eq!(resource.latest().unwrap().landmarks[0].x, 1.0);
    }
}
