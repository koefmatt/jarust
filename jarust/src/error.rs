use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite::http::header::InvalidHeaderValue;

#[derive(thiserror::Error, Debug)]
pub enum JaError {
    /* Transformed Errors */
    #[error("Websocket error: {0}")]
    WebSocket(#[from] tungstenite::Error),
    #[error("InvalidHeaderValue: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("Failed to parse json: {0}")]
    JsonParsingFailure(#[from] serde_json::Error),
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),
    /* Custom Errors */
    #[error("Error while parsing an incomplete packet")]
    IncompletePacket,
    #[error("Transport is not opened")]
    TransportNotOpened,
    #[error("Session is not part of any connection")]
    DanglingSession,
    #[error("Handle is not part of any session")]
    DanglingHandle,
    #[error("Invalid Janus request")]
    InvalidJanusRequest,
    #[error("Can't send data in closed channel")]
    SendError,
}
