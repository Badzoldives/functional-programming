// Mengimpor StatusCode untuk merepresentasikan HTTP status code
use axum::http::StatusCode;
// Mengimpor trait IntoResponse untuk mengkonversi nilai menjadi HTTP response
use axum::response::IntoResponse;
// Mengimpor fungsi process_multiprocess dari modul watermark_service
use crate::services::watermark_service::process_multiprocess;
// Mengimpor PathBuf untuk merepresentasikan path file sistem
use std::path::PathBuf;

// Fungsi handler async untuk endpoint pemrosesan watermark
// async karena operasi I/O yang mungkin membutuhkan waktu
// impl IntoResponse berarti fungsi ini mengembalikan tipe yang bisa dikonversi ke HTTP response
pub async fn process_watermark() -> impl IntoResponse {

    // Membuat vector berisi path gambar-gambar yang akan diproses
    // contoh (biasanya dari multipart handler)
    let images = vec![
        // Path gambar pertama di folder tmp
        PathBuf::from("tmp/img1.png"),
        // Path gambar kedua di folder tmp
        PathBuf::from("tmp/img2.png"),
    ];

    // Membuat PathBuf untuk file watermark yang akan ditempelkan
    let watermark = PathBuf::from("tmp/watermark.png");

    // Memanggil fungsi process_multiprocess untuk memproses semua gambar
    // Fungsi ini akan menjalankan worker process terpisah untuk setiap gambar
    let results = process_multiprocess(images, watermark);

    // Mengembalikan tuple yang berisi:
    (
        StatusCode::OK,  // HTTP status code 200 OK
        // String yang menunjukkan jumlah file yang diproses
        format!("Diproses {} file secara MULTIPROCESSING", results.len()),
    )
}