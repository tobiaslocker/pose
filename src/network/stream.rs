use std::future::Future;
use std::pin::Pin;

pub trait PayloadStream {
    fn next_payload<'a>(&'a mut self)
    -> Pin<Box<dyn Future<Output = Option<Vec<u8>>> + Send + 'a>>;
}
