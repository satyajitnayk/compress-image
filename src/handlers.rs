use crate::compressor::compress_jpeg;
use axum::{
    body::Body,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

use axum::extract::Multipart;

pub async fn compress_image(mut multipart: Multipart) -> Response {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("image") {
            let data = field.bytes().await.unwrap();
            match compress_jpeg(&data) {
                Ok(compressed_bytes) => {
                    return Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "image/jpeg")
                        .body(Body::from(compressed_bytes)) // Use `Body::from` here
                        .unwrap();
                }
                Err(_) => {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            }
        }
    }

    // If no "image" field found, return a bad request response
    StatusCode::BAD_REQUEST.into_response()
}
