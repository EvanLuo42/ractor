use bytes::BytesMut;
use prost::{EncodeError, Message};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::protos::global::ErrorResponse;

pub enum ErrorCode {
    SceneNotExist = 1,
    NetworkError = 2,
    DecodeProtoFailed = 3
}

pub fn create_error_response(error_code: ErrorCode) -> Result<BytesMut, EncodeError> {
    let response = ErrorResponse {
        error_code: error_code as u32
    };
    let mut frame = BytesMut::with_capacity(response.encoded_len());
    response.encode(&mut frame)?;
    Ok(frame)
}

pub async fn respond_error(mut stream: TcpStream, error_code: ErrorCode) {
    stream
        .write_buf(&mut create_error_response(error_code).unwrap())
        .await
        .unwrap();
}
