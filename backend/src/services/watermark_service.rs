// Mengimpor PathBuf untuk merepresentasikan path file sistem
use std::path::PathBuf;
// Mengimpor Command untuk menjalankan proses eksternal
use std::process::Command;
// Mengimpor Uuid untuk menghasilkan identifier unik untuk nama file output
use uuid::Uuid;

// Fungsi publik untuk memproses banyak gambar dengan watermark secara multiprocess
// Parameter:
// - images: vector berisi path gambar-gambar yang akan diberi watermark
// - watermark: path file watermark yang akan ditempelkan
// Mengembalikan vector berisi path file hasil pemrosesan
pub fn process_multiprocess(
    images: Vec<PathBuf>,      // Vector berisi path gambar-gambar input
    watermark: PathBuf,        // Path file watermark
) -> Vec<PathBuf> {            // Mengembalikan vector path file output

    // Mengkonversi vector images menjadi iterator untuk diproses satu per satu
    images
        .into_iter()
        // Memetakan setiap gambar (img) ke operasi pemrosesan
        .map(|img| {
            // Membuat path output unik menggunakan UUID random
            // Format: tmp/<uuid-random>.png
            let output = PathBuf::from(format!("tmp/{}.png", Uuid::new_v4()));

            // Membuat command baru untuk menjalankan cargo
            Command::new("cargo")
                // Menambahkan argumen-argumen untuk cargo command
                .args([
                    "run",                              // Subcommand cargo untuk menjalankan binary
                    "--bin",                            // Flag untuk menentukan binary tertentu
                    "watermark_worker",                 // Nama binary worker yang akan dijalankan
                    img.to_str().unwrap(),             // Path gambar input (dikonversi ke string)
                    watermark.to_str().unwrap(),       // Path watermark (dikonversi ke string)
                    output.to_str().unwrap(),          // Path output (dikonversi ke string)
                ])
                // Spawn (jalankan) proses secara asinkron tanpa menunggu selesai
                .spawn()
                // Jika spawn gagal, panic dengan pesan error
                .expect("Gagal spawn worker process");

            // Mengembalikan path output untuk setiap gambar yang diproses
            output
        })
        // Mengumpulkan semua hasil map menjadi vector PathBuf
        .collect()
}