use crate::network::stream::PayloadStream;
use futures_util::SinkExt;
use futures_util::StreamExt;
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

pub struct FramedPayloadStream {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl FramedPayloadStream {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let url = Url::parse(url)?;
        let (ws_stream, _) = connect_async(url).await?;
        Ok(Self { ws_stream })
    }
}

impl PayloadStream for FramedPayloadStream {
    fn next_payload<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn std::future::Future<Output = Option<Vec<u8>>> + Send + 'a>> {
        Box::pin(async move {
            match self.ws_stream.next().await {
                Some(Ok(Message::Binary(data))) => Some(data),
                Some(Ok(Message::Ping(payload))) => {
                    if let Err(e) = self.ws_stream.send(Message::Pong(payload)).await {
                        eprintln!("Failed to send Pong: {}", e);
                        return None;
                    }
                    self.next_payload().await
                }
                Some(Ok(Message::Pong(_))) => {
                    eprintln!("Received Pong");
                    self.next_payload().await
                }
                Some(Ok(_)) => self.next_payload().await,
                Some(Err(e)) => {
                    eprintln!("WebSocket error: {}", e);
                    None
                }
                None => {
                    eprintln!("WebSocket stream closed");
                    None
                }
            }
        })
    }
}
