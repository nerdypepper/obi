use std::io::Cursor;
use std::mem::size_of;

use obi::{CompressionType, Image, ImageInfoHeader};

#[test]
fn size_of_image_info_header() {
    let file_header_size = size_of::<ImageInfoHeader>();
    assert_eq!(16, file_header_size);
}

#[test]
fn compression() {
    let mut img = Image::new(50, 5);
    img.use_compression(CompressionType::RLE);
    let encoded = img.encode().unwrap();
    let mut cursor = Cursor::new(encoded);
    let decoded = Image::decode(&mut cursor).unwrap();
    assert_eq!(
        CompressionType::from_u32(decoded.image_info_header.compression_type),
        CompressionType::RLE
    );
}

#[test]
fn size_no_rounding() {
    let img = Image::new(100, 80);
    let encoded = img.encode().unwrap();
    assert_eq!(encoded.len(), 26 + 100 * 80 / 8);
    let mut cursor = Cursor::new(encoded);
    let decoded = Image::decode(&mut cursor).unwrap();
    assert_eq!(decoded.image_info_header.width, 100);
    assert_eq!(decoded.image_info_header.height, 80);
}

#[test]
fn size_round_nearest() {
    let img = Image::new(50, 5);
    let encoded = img.encode().unwrap();
    assert_eq!(encoded.len(), 26 + 256 / 8);
    let mut cursor = Cursor::new(encoded);
    let decoded = Image::decode(&mut cursor).unwrap();
    assert_eq!(decoded.image_info_header.width, 50);
    assert_eq!(decoded.image_info_header.height, 5);
}

#[test]
fn size_compressed() {
    let mut img = Image::new(100, 80);
    img.use_compression(CompressionType::RLE);
    let encoded = img.encode().unwrap();
    // 26 - headers
    // 4 - first and only length
    // 4 - separator zero
    // 1 - first and only data point
    assert_eq!(encoded.len(), 26 + 4 + 4 + 1);
    let mut cursor = Cursor::new(encoded);
    let decoded = Image::decode(&mut cursor).unwrap();
    assert_eq!(decoded.image_info_header.width, 100);
    assert_eq!(decoded.image_info_header.height, 80);
    assert!(decoded.data.iter().all(|x| !*x));
}
