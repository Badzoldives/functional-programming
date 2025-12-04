use axum::extract::Multipart;
use image::{GenericImageView, ImageOutputFormat, imageops::FilterType};
use std::io::{Cursor, Write};
use zip::write::FileOptions;

pub async fn process(mut multipart: Multipart) -> Vec<u8> {
    let mut photos: Vec<(String, Vec<u8>)> = vec![];
    let mut watermark_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let filename = field.file_name().unwrap().to_string();
        let bytes = field.bytes().await.unwrap().to_vec();

        if name == "watermark" {
            watermark_bytes = Some(bytes);
        } else if name == "photos" || name == "photos[]" {
            photos.push((filename, bytes));
        }
    }

    let wm_data = watermark_bytes.expect("Watermark tidak ditemukan!");
    let wm_img = image::load_from_memory(&wm_data).unwrap();

    let mut zip_buffer = Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut zip_buffer);
        let options =
            FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        for (filename, bytes) in photos {
            let mut img = image::load_from_memory(&bytes).unwrap();
            let (w, h) = img.dimensions();

            let wm_small = wm_img.resize(w / 4, h / 4, FilterType::Lanczos3);

            let (wm_w, wm_h) = wm_small.dimensions();
            let pos_x = (w - wm_w - 20) as i64;
            let pos_y = (h - wm_h - 20) as i64;

            image::imageops::overlay(&mut img, &wm_small, pos_x, pos_y);

            let mut out_bytes = Vec::new();
            img.write_to(
                &mut Cursor::new(&mut out_bytes),
                ImageOutputFormat::Png,
            )
            .unwrap();

            zip.start_file(filename, options).unwrap();
            zip.write_all(&out_bytes).unwrap();
        }
    }
    zip_buffer.into_inner()
}
