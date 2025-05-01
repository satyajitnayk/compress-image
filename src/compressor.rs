use image::{codecs::jpeg::JpegEncoder, load_from_memory};
use std::io::Cursor;

pub fn compress_jpeg(data: &[u8]) -> Result<Vec<u8>, String> {
    // Decode the image from memory
    let img = load_from_memory(data).map_err(|e| e.to_string())?.to_rgb8();

    // Output buffer
    let mut buffer = Cursor::new(Vec::new());

    // Create encoder with desired quality (e.g., 60)
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 60);

    // Encode the image
    encoder
        .encode(
            &img,
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| e.to_string())?;

    Ok(buffer.into_inner())
}
