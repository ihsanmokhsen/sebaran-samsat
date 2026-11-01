# Panduan Menjalankan Project Rust

Panduan ini untuk project **Sebaran Samsat NTT**, ditulis untuk yang baru pertama kali memakai Rust.

---

## 1. Memasang Rust (sekali saja)

Rust dipasang lewat **rustup**, penginstal resmi dari https://rustup.rs.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Tutup lalu buka lagi Terminal, atau muat ulang PATH:

```bash
source ~/.cargo/env
```

Cek apakah sudah terpasang:

```bash
cargo --version
```

```bash
rustc --version
```

| Alat     | Fungsi                                                      |
|----------|-------------------------------------------------------------|
| `rustup` | Mengelola versi Rust (pasang, update, hapus)                |
| `rustc`  | Compiler: mengubah kode `.rs` menjadi program               |
| `cargo`  | Manajer project: build, run, test, unduh library (crate)    |

Sehari-hari Anda cukup memakai `cargo`.

---

## 2. Masuk ke folder project

```bash
cd "/Users/ihsanmokhsen/Documents/Proyek Sistem Informasi/Sebaran Samsat NTT"
```

Semua perintah `cargo` dijalankan dari folder yang berisi `Cargo.toml`.

---

## 3. Menjalankan aplikasi

### Mode pengembangan (compile cepat, program lebih lambat)

```bash
cargo run
```

### Mode rilis (compile lebih lama, program kecil dan cepat)

```bash
cargo run --release
```

Jika berhasil, Terminal menampilkan:

```
Sebaran Samsat NTT berjalan di http://127.0.0.1:8080
```

Buka alamat itu di browser. Untuk menghentikan server, tekan **Ctrl + C** di Terminal.

### Mengganti port atau host

```bash
PORT=3000 cargo run --release
```

Agar bisa diakses dari HP/komputer lain di jaringan yang sama:

```bash
HOST=0.0.0.0 cargo run --release
```

Lalu buka `http://<IP-komputer-Anda>:8080` dari perangkat lain.

---

## 4. Perintah `cargo` yang sering dipakai

| Perintah                | Kegunaan                                                    |
|-------------------------|-------------------------------------------------------------|
| `cargo run`             | Compile lalu langsung jalankan                              |
| `cargo build`           | Compile saja → `target/debug/sebaran-samsat-ntt`            |
| `cargo build --release` | Compile versi rilis → `target/release/sebaran-samsat-ntt`   |
| `cargo check`           | Cek error tanpa membuat program (paling cepat)              |
| `cargo fmt`             | Merapikan format kode otomatis                              |
| `cargo clippy`          | Saran perbaikan kode (seperti "guru" Rust)                  |
| `cargo clean`           | Menghapus folder `target/` (hasil build)                    |

`cargo fmt` dan `cargo clippy` perlu dipasang dulu:

```bash
rustup component add rustfmt clippy
```

---

## 5. Alur kerja saat mengubah sesuatu

Data dan HTML **ditanam ke dalam program** saat compile (lihat `include_str!` di `src/main.rs`).
Jadi setiap kali Anda mengubah file berikut, server harus dijalankan ulang:

| Yang diubah                    | Contoh                              |
|--------------------------------|-------------------------------------|
| `data/samsat.json`             | Ganti nama pejabat, geser koordinat |
| `data/kantor-pusat.json`       | Ganti pimpinan / titik kantor pusat |
| `static/index.html`            | Ubah tampilan                       |
| `src/main.rs`                  | Tambah endpoint baru                |

Langkahnya:

1. Hentikan server: **Ctrl + C**
2. Jalankan lagi: `cargo run --release`
3. Muat ulang browser (**Cmd + R**)

---

## 6. Menjalankan tanpa `cargo` (setelah build)

Hasil `cargo build --release` adalah **satu file** yang sudah berisi HTML dan data:

```bash
./target/release/sebaran-samsat-ntt
```

File ini (~350 KB) bisa disalin ke server lain dengan OS yang sama dan langsung dijalankan,
tanpa perlu memasang Rust di sana.

### Untuk server Linux

Build di Mac menghasilkan program untuk macOS. Untuk server Linux, cara paling mudah adalah
memasang Rust di server tersebut lalu menjalankan `cargo build --release` di sana.

---

## 7. Struktur project

```
Sebaran Samsat NTT/
├── Cargo.toml           ← identitas project & pengaturan build
├── src/
│   └── main.rs          ← kode Rust (server web)
├── static/
│   └── index.html       ← tampilan peta
├── data/
│   ├── samsat.json      ← data 22 Samsat
│   └── kantor-pusat.json← kantor pusat & pimpinan BPAD
├── docs/
│   └── MENJALANKAN-RUST.md
└── target/              ← hasil build (otomatis, jangan diedit)
```

---

## 8. Membaca `src/main.rs` (urutan yang disarankan)

1. **`fn main()`** — titik awal program: membaca `HOST`/`PORT`, membuka port, menunggu koneksi.
2. **`fn handle()`** — membaca baris pertama request, misalnya `GET /api/samsat HTTP/1.1`.
3. **`fn route()`** — `match` yang memilih respons berdasarkan method dan path.
4. **`struct Response`** — wadah sederhana untuk status, tipe konten, dan isi.

Konsep Rust yang muncul di sini:

| Konsep                  | Contoh di kode                                  |
|-------------------------|-------------------------------------------------|
| `const` & `&'static str`| `const INDEX_HTML: &str = include_str!(...)`    |
| `struct`                | `struct Response { ... }`                       |
| `match` pola tuple      | `match (method, path) { ("GET", "/") => ... }`  |
| `Result` & `Option`     | `.unwrap_or(...)`, `.is_err()`, `.expect(...)`  |
| Iterator                | `listener.incoming().flatten()`                 |
| Closure & thread        | `thread::spawn(\|\| handle(stream))`            |
| Macro                   | `println!`, `format!`, `include_str!`           |

---

## 9. Masalah yang sering terjadi

**`cargo: command not found`**
Jalankan `source ~/.cargo/env`, atau tutup dan buka lagi Terminal.

**`Gagal membuka port: Address already in use`**
Port 8080 sedang dipakai (mungkin server lama masih jalan). Hentikan dengan Ctrl + C di
Terminal lama, atau pakai port lain: `PORT=8081 cargo run --release`.

**Perubahan data tidak muncul di browser**
Server belum dijalankan ulang. Ingat: data ditanam saat compile (lihat bagian 5).

**Error saat compile setelah mengedit JSON**
Rust tidak memeriksa isi JSON, jadi error biasanya muncul di browser (peta kosong).
Periksa koma yang hilang/berlebih di `data/samsat.json`, misalnya dengan:

```bash
python3 -m json.tool data/samsat.json > /dev/null && echo "JSON valid"
```

---

## 10. Memperbarui Rust

```bash
rustup update
```
