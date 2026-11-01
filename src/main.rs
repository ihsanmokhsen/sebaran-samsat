//! Sebaran Samsat NTT — server web mini tanpa dependensi.
//!
//! Semua file (HTML + data JSON) "ditanam" ke dalam binary saat kompilasi
//! lewat `include_str!`, jadi hasil build hanya satu file yang bisa
//! dijalankan di mana saja.

use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// `include_str!` membaca file saat compile time dan menjadikannya &'static str.
const INDEX_HTML: &str = include_str!("../static/index.html");
const SAMSAT_JSON: &str = include_str!("../data/samsat.json");
const KANTOR_PUSAT_JSON: &str = include_str!("../data/kantor-pusat.json");

/// Satu respons HTTP sederhana: kode status, tipe konten, dan isi (dalam byte,
/// supaya bisa berisi teks maupun gambar).
struct Response {
    status: &'static str,
    content_type: &'static str,
    body: Vec<u8>,
}

/// Melayani foto kantor dari folder `foto/` di disk, mis. `/foto/samsat-ende.jpg`.
/// Berbeda dengan HTML/JSON, foto TIDAK ditanam ke binary, jadi bisa ditambah
/// tanpa compile ulang.
fn foto(nama: &str) -> Response {
    // Tolak nama aneh (mis. "../") agar orang tidak bisa membaca file lain.
    let aman = !nama.is_empty()
        && nama.chars().all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
        && !nama.contains("..");
    let content_type = match nama.rsplit('.').next() {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        _ => "",
    };

    match fs::read(format!("foto/{nama}")) {
        Ok(isi) if aman && !content_type.is_empty() => Response {
            status: "200 OK",
            content_type,
            body: isi,
        },
        _ => Response {
            status: "404 Not Found",
            content_type: "text/plain; charset=utf-8",
            body: "404 - foto tidak ditemukan".into(),
        },
    }
}

/// Router: mencocokkan method + path ke respons yang sesuai.
fn route(method: &str, path: &str) -> Response {
    // Buang query string, mis. "/api/samsat?x=1" -> "/api/samsat"
    let path = path.split('?').next().unwrap_or("/");

    if let Some(nama) = path.strip_prefix("/foto/") {
        return foto(nama);
    }

    match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => Response {
            status: "200 OK",
            content_type: "text/html; charset=utf-8",
            body: INDEX_HTML.into(),
        },
        ("GET", "/api/samsat") => Response {
            status: "200 OK",
            content_type: "application/json; charset=utf-8",
            body: SAMSAT_JSON.into(),
        },
        ("GET", "/api/kantor-pusat") => Response {
            status: "200 OK",
            content_type: "application/json; charset=utf-8",
            body: KANTOR_PUSAT_JSON.into(),
        },
        ("GET", "/health") => Response {
            status: "200 OK",
            content_type: "text/plain; charset=utf-8",
            body: "ok".into(),
        },
        _ => Response {
            status: "404 Not Found",
            content_type: "text/plain; charset=utf-8",
            body: "404 - halaman tidak ditemukan".into(),
        },
    }
}

/// Menangani satu koneksi: baca baris pertama request, lalu kirim respons.
fn handle(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }

    // Baris pertama berbentuk: "GET /api/samsat HTTP/1.1"
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    let res = route(method, path);
    println!("{method} {path} -> {}", res.status);

    let header = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n",
        res.status,
        res.content_type,
        res.body.len()
    );
    // Abaikan error tulis (mis. browser menutup koneksi lebih dulu).
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(&res.body);
}

fn main() {
    // Port & host bisa diatur lewat variabel lingkungan, default 127.0.0.1:8080.
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("{host}:{port}");

    let listener = TcpListener::bind(&addr).expect("Gagal membuka port");
    println!("Sebaran Samsat NTT berjalan di http://{addr}");

    // Setiap koneksi ditangani di thread terpisah agar tidak saling menunggu.
    for stream in listener.incoming().flatten() {
        thread::spawn(|| handle(stream));
    }
}
