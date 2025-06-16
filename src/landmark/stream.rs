use crate::protocol::LandmarkFrame;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Stream {
    pub provider: Box<dyn crate::landmark::StreamProvider>,
    pub latest: Option<LandmarkFrame>,
}

impl Stream {
    pub fn update(&mut self) {
        if let Some(result) = self.provider.poll() {
            self.latest = Some(result);
        }
    }

    pub fn latest(&self) -> Option<&LandmarkFrame> {
        self.latest.as_ref()
    }

    pub fn system_update(mut detection: ResMut<Self>) {
        detection.update();
    }

    pub fn from_channel(rx: tokio::sync::mpsc::Receiver<LandmarkFrame>) -> Self {
        Self {
            provider: Box::new(crate::landmark::provider::ChannelStreamProvider::new(rx)),
            latest: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::LandmarkFrame;
    use crate::protocol::fbs::landmark::{Availability, Landmark};

    struct DummyProvider;

    impl crate::landmark::StreamProvider for DummyProvider {
        fn poll(&mut self) -> Option<LandmarkFrame> {
            Some(LandmarkFrame {
                landmarks: vec![Landmark {
                    x: 1.0,
                    y: 2.0,
                    z: 3.0,
                    availability: Some(Availability {
                        visibility: 0.9,
                        presence: 0.8,
                    }),
                }],
                timestamp: 1234.5678,
            })
        }
    }

    #[test]
    fn test_detection_resource_update() {
        let mut resource = Stream {
            provider: Box::new(DummyProvider),
            latest: None,
        };

        resource.update();
        assert!(resource.latest().is_some());
        assert_eq!(resource.latest().unwrap().landmarks[0].x, 1.0);
    }
}
