use crate::network::stream::PayloadStream;
use std::future::Future;
use std::pin::Pin;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;

pub struct FramedPayloadStream {
    reader: BufReader<TcpStream>,
}

impl FramedPayloadStream {
    pub async fn connect(addr: &str) -> std::io::Result<Self> {
        use tokio::net::TcpStream;
        let stream = TcpStream::connect(addr).await?;
        Ok(Self {
            reader: BufReader::new(stream),
        })
    }
}

impl PayloadStream for FramedPayloadStream
where
    Self: Send,
{
    fn next_payload<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<u8>>> + Send + 'a>> {
        Box::pin(async move {
            let mut length_buf = [0u8; 4];
            if self.reader.read_exact(&mut length_buf).await.is_err() {
                return None;
            }

            let length = u32::from_le_bytes(length_buf) as usize;

            if length == 0 || length > 65536 {
                eprintln!("[TCP] Unexpected message length: {length} bytes — skipping!");
                return Some(Vec::new());
            }

            let mut buffer = vec![0; length];
            if self.reader.read_exact(&mut buffer).await.is_err() {
                return None;
            }

            Some(buffer)
        })
    }
}
