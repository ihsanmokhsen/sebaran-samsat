# Sebaran Samsat NTT

Peta sebaran 22 kantor Samsat (UPTD Pendapatan Daerah) di 22 kabupaten/kota NTT,
lengkap dengan pejabat UPTD. Ditulis dengan Rust **tanpa dependensi** (hanya `std`).

## Menjalankan

Panduan lengkap untuk pemula: [docs/MENJALANKAN-RUST.md](docs/MENJALANKAN-RUST.md).

```bash
cargo run --release
```

Buka http://127.0.0.1:8080. Ganti port/host: `PORT=3000 HOST=0.0.0.0 cargo run --release`.

Hasil build: satu file `target/release/sebaran-samsat-ntt` (~330 KB) yang sudah
berisi HTML dan data — tinggal salin ke server dan jalankan.

## Struktur

```
src/main.rs          server HTTP mini (TcpListener + thread)
static/index.html    tampilan peta (MapLibre GL, mode 2D/3D)
data/samsat.json     22 Samsat: alamat, koordinat, link Google Maps, pejabat
data/kantor-pusat.json  kantor pusat BPAD (Kantor Gubernur) + pimpinan
foto/                foto kantor Samsat (dibaca dari disk, tidak ditanam)
```

| Endpoint        | Isi                    |
|-----------------|------------------------|
| `/`             | halaman peta           |
| `/api/samsat`   | data JSON 22 Samsat    |
| `/api/kantor-pusat` | data JSON kantor pusat |
| `/foto/<nama>`  | foto dari folder `foto/` (jpg/png/webp) |
| `/health`       | `ok`                   |

Karena file ditanam lewat `include_str!`, setiap mengubah `data/` atau `static/`
perlu `cargo run` ulang.

## Menambah sosmed & foto Samsat

Di `data/samsat.json`, tambahkan pada Samsat yang bersangkutan (keduanya opsional):

```json
"sosmed": [["instagram", "https://www.instagram.com/..."], ["tiktok", "https://www.tiktok.com/@..."]],
"foto": "/foto/samsat-ende.jpg"
```

Platform yang dikenali: `instagram`, `facebook`, `tiktok`. File fotonya disimpan
di `foto/samsat-ende.jpg`. Foto bisa diganti tanpa compile ulang, tetapi menambah
kolom `"foto"` di JSON tetap perlu `cargo run` ulang.

## Sumber data

- Pejabat & alamat UPTD: https://bapenda.nttprov.go.id/profil
- Foto kantor (15 Samsat): foto utama tiap tempat di Google Maps (kiriman pengguna /
  Street View). Ganti dengan foto milik sendiri bila aplikasi dipakai untuk publik.
- Peta dasar: Esri World Imagery (satelit), OpenStreetMap (standar).
- Mode 3D: relief dari AWS Terrain Tiles, gedung 3D dari OpenFreeMap.
- Koordinat: tautan "Buka Maps" resmi di halaman tersebut (Google Maps).
  Khusus Kab. Malaka belum ada tautan resmi; titik diambil dari pencarian
  Google Maps "Kantor Samsat Sementara Malaka Tengah" (Betun).

## Ide latihan Rust berikutnya

1. Pakai `serde` + `serde_json` untuk membaca `samsat.json` ke `struct Samsat`.
2. Tambah endpoint `/api/samsat/{id}`.
3. Ganti server manual dengan `axum` dan bandingkan kodenya.
