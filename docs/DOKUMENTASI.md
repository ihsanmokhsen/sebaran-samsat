# Dokumentasi Sistem Informasi Sebaran Samsat NTT

> “Data Samsat Akurat, Pelayanan Lebih Dekat.”

| | |
|---|---|
| **Nama aplikasi** | Sebaran Samsat NTT |
| **Pemilik** | Badan Pendapatan dan Aset Daerah (BPAD) Provinsi Nusa Tenggara Timur |
| **Pengembang** | IT-BPAD (https://ihsanmokhsen.com) |
| **Versi** | 0.1.0 (November 2026) |
| **Repositori** | https://github.com/ihsanmokhsen/sebaran-samsat |
| **Teknologi** | Rust (tanpa dependensi) · MapLibre GL 4.7.1 · HTML/CSS/JS murni |

---

## Daftar Isi

1. [Ringkasan](#1-ringkasan)
2. [Fitur](#2-fitur)
3. [Panduan Pengguna](#3-panduan-pengguna)
4. [Arsitektur](#4-arsitektur)
5. [API](#5-api)
6. [Struktur Data](#6-struktur-data)
7. [Instalasi & Menjalankan](#7-instalasi--menjalankan)
8. [Deploy ke Server Linux](#8-deploy-ke-server-linux)
9. [Pemeliharaan Data](#9-pemeliharaan-data)
10. [Mengubah Tampilan](#10-mengubah-tampilan)
11. [Sumber Data & Lisensi](#11-sumber-data--lisensi)
12. [Keterbatasan & Rencana Pengembangan](#12-keterbatasan--rencana-pengembangan)

---

## 1. Ringkasan

Aplikasi web berbasis peta yang menampilkan lokasi **22 kantor Samsat (UPTD Pendapatan Daerah)**
di **22 kabupaten/kota** Provinsi NTT, ditambah **Kantor Pusat BPAD NTT** di sayap kiri Gedung
Sasando (Kantor Gubernur NTT), Kupang.

Setiap titik memuat alamat, foto kantor, pejabat UPTD, tautan Google Maps, tombol rute, dan
akun media sosial (bila ada).

Server ditulis dengan Rust hanya memakai pustaka standar. Hasil build adalah **satu file
program ±350 KB** yang sudah berisi halaman web dan data, sehingga ringan dijalankan di server
mana pun.

### Cakupan data

| Pulau   | Jumlah Samsat | Samsat |
|---------|:---:|--------|
| Timor   | 6 | Kota Kupang, Kab. Kupang, Soe, Kefamenanu, Atambua, Betun |
| Flores  | 8 | Larantuka, Maumere, Ende, Mbay, Bajawa, Borong, Ruteng, Labuan Bajo |
| Sumba   | 4 | Waingapu, Waibakul, Waikabubak, Tambolaka |
| Rote    | 1 | Rote Ndao |
| Sabu    | 1 | Sabu Raijua |
| Alor    | 1 | Kalabahi |
| Lembata | 1 | Lewoleba |
| **Total** | **22** | + 1 Kantor Pusat BPAD NTT |

---

## 2. Fitur

### Peta
- **Dua peta dasar:** Satelit (Esri World Imagery) dan Standar (OpenStreetMap).
- **Mode 3D:** relief pegunungan, bayangan bukit, dan gedung 3D. Peta dimiringkan 60°.
  Ada tombol **Putar** (rotasi otomatis) dan **Tegak** (kembali menghadap utara).
- **Titik berwarna per pulau.** Kantor pusat memakai titik oranye yang lebih besar dengan cincin navy.
- **Nama Samsat muncul otomatis** di samping titik saat peta diperbesar (zoom ≥ 10).
- **Legenda** jumlah titik per pulau, bisa diciutkan.

### Daftar & pencarian
- **Sidebar** berisi daftar kantor lengkap dengan foto kecil. Bisa dibuka/tutup dengan tombol ☰.
- **Pencarian** berdasarkan nama Samsat, kabupaten/kota, alamat, atau **nama pejabat**.
- **Filter pulau:** Semua, Timor, Rote, Sabu, Alor, Lembata, Flores, Sumba.
- **Klik item** → peta terbang ke lokasi kantor dan membuka popup.

### Popup kantor
- Foto kantor, nama, kabupaten/kota, dan alamat.
- Daftar pejabat: Kepala UPTD, Kasubag Tata Usaha, Kasi Pendapatan, Kasi Penagihan
  (kantor pusat: Kepala BPAD, Sekretaris, dan 4 Kepala Bidang).
- Tombol **Google Maps**, **Rute** (navigasi Google Maps dari lokasi pengguna), dan
  **Instagram / TikTok / Facebook** bila tersedia.

### Header
- Logo, nama aplikasi, dan slogan.
- Tombol **3D** dan **↻** (reset pencarian dan filter, tampilkan semua titik).

### Tampilan
- Gaya **glassmorphism** ala Apple pada semua panel: semi transparan dengan efek blur.
- **Responsif:** di HP (≤ 760 px) subjudul header disembunyikan, sidebar menjadi panel penuh, legenda otomatis diciutkan, dan sidebar menutup sendiri setelah memilih kantor.

---

## 3. Panduan Pengguna

| Ingin… | Caranya |
|---|---|
| Melihat semua kantor | Tekan **↻** di kanan atas |
| Mencari kantor | Ketik di kotak **Cari** (mis. `ende`, `sumba`, atau nama pejabat) |
| Menyaring per pulau | Klik tombol pulau di bawah kotak pencarian |
| Melihat detail kantor | Klik nama di daftar, atau klik titik di peta |
| Pergi ke kantor | Di popup, tekan **Rute ↗** |
| Ganti peta satelit/biasa | Tombol **Satelit / Standar** di kanan bawah |
| Melihat 3D | Tekan **3D** di header. Klik kanan + geser (desktop) atau dua jari (HP) untuk memiringkan/memutar |
| Memperbesar peta | Scroll / cubit, atau tombol **+ / −** di kiri atas |
| Menyembunyikan daftar | Tekan **☰** |

---

## 4. Arsitektur

```
 Browser                                  Server Rust (satu file program)
┌─────────────────────────────┐          ┌──────────────────────────────────┐
│ index.html                  │  GET /   │ static/index.html  ─┐            │
│  ├─ MapLibre GL (unpkg)     │ ───────▶ │ data/samsat.json    ├ ditanam    │
│  ├─ fetch /api/samsat       │ ───────▶ │ data/kantor-pusat…  ┘ saat build │
│  ├─ fetch /api/kantor-pusat │          │                                  │
│  └─ <img src=/foto/...>     │ ───────▶ │ folder foto/  (dibaca dari disk) │
└──────────────┬──────────────┘          └──────────────────────────────────┘
               │ ubin peta (langsung dari browser)
               ▼
  Esri · OpenStreetMap · AWS Terrain · OpenFreeMap
```

### Server (`src/main.rs`)

| Bagian | Tugas |
|---|---|
| `main()` | Membaca `HOST` & `PORT`, membuka port, membuat satu thread per koneksi |
| `handle()` | Membaca baris pertama request (`GET /path HTTP/1.1`) lalu menulis respons |
| `route()` | Mencocokkan method + path ke respons |
| `foto()` | Mengirim file gambar dari folder `foto/`, dengan pemeriksaan nama file |
| `struct Response` | Status, tipe konten, dan isi (byte) |

Catatan teknis:
- **Tanpa dependensi.** Hanya memakai `std::net`, `std::thread`, `std::fs`, dan `std::io`.
- **Data ditanam saat compile** dengan `include_str!`. Mengubah HTML/JSON berarti perlu build ulang.
- **Foto dibaca dari disk**, jadi foto bisa diganti tanpa build ulang.
- **Keamanan foto:** hanya nama berisi huruf, angka, `-`, `_`, dan `.` yang diterima, tanpa `..`,
  dan hanya `.jpg`, `.jpeg`, `.png`, `.webp`. Ini mencegah akses ke file lain di server.
- **Cache-Control: no-cache**, sehingga perubahan data langsung terlihat setelah server di-restart.
- **Profil rilis** (`Cargo.toml`): `opt-level = "z"`, `lto`, `codegen-units = 1`,
  `panic = "abort"`, `strip`, agar ukuran program sekecil mungkin.

### Frontend (`static/index.html`)

Satu file berisi HTML, CSS, dan JavaScript tanpa framework atau proses build.
Library satu-satunya adalah **MapLibre GL 4.7.1**, dimuat dari unpkg.com.

| Fungsi JS | Tugas |
|---|---|
| `init()` | Mengambil data dari API, membuat penanda, chips, dan legenda |
| `render()` | Menerapkan pencarian + filter ke daftar dan penanda, lalu menyesuaikan zoom |
| `pilih(id)` | Terbang ke kantor dan membuka popupnya |
| `tandai(id)` | Menyorot item & titik yang aktif |
| `buatMarker()` | Membuat titik (pembungkus + `.titik` + label `.nama`) dan popup |
| `popupHtml()` | Menyusun isi popup |
| `setTigaD()` | Menyalakan/mematikan relief, bayangan, gedung 3D, dan kemiringan |
| `putar()` | Rotasi otomatis di mode 3D |

Layer peta:

| Layer | Sumber | Keterangan |
|---|---|---|
| `satelit` | Esri World Imagery | Default |
| `standar` | OpenStreetMap | |
| `bayangan` | AWS Terrain Tiles (terrarium) | Hanya di mode 3D |
| `gedung-3d` | OpenFreeMap (vector, layer `building`) | Hanya di mode 3D, mulai zoom 14 |

---

## 5. API

Semua endpoint hanya menerima `GET`.

| Endpoint | Tipe | Isi |
|---|---|---|
| `/` atau `/index.html` | `text/html` | Halaman peta |
| `/api/samsat` | `application/json` | Array 22 Samsat |
| `/api/kantor-pusat` | `application/json` | Objek kantor pusat |
| `/foto/<nama>` | `image/*` | Foto dari folder `foto/` |
| `/health` | `text/plain` | `ok` (untuk cek server hidup) |
| lainnya | `text/plain` | `404` |

Contoh:

```bash
curl http://127.0.0.1:8080/api/samsat
```

```bash
curl http://127.0.0.1:8080/health
```

---

## 6. Struktur Data

### `data/samsat.json` — array objek

| Kolom | Wajib | Tipe | Keterangan |
|---|:---:|---|---|
| `id` | ✔ | angka | Nomor urut 1–22, harus unik |
| `nama` | ✔ | teks | Mis. `"Samsat Ende"` |
| `kabkota` | ✔ | teks | Mis. `"Kab. Ende"` |
| `pulau` | ✔ | teks | Salah satu: `Timor`, `Rote`, `Sabu`, `Alor`, `Lembata`, `Flores`, `Sumba` |
| `alamat` | ✔ | teks | Alamat lengkap kantor |
| `lat`, `lng` | ✔ | angka | Koordinat desimal (lintang negatif untuk NTT) |
| `maps` | ✔ | teks | Tautan Google Maps |
| `pejabat` | ✔ | array | Pasangan `["Nama", "Jabatan"]`. Isi `"-"` untuk jabatan kosong |
| `foto` |  | teks | Mis. `"/foto/samsat-13.jpg"` |
| `sosmed` |  | array | Pasangan `["instagram" \| "facebook" \| "tiktok", "url"]` |
| `catatan` |  | teks | Catatan tambahan di popup (warna oranye) |

Contoh satu entri:

```json
{
  "id": 13,
  "nama": "Samsat Ende",
  "kabkota": "Kab. Ende",
  "pulau": "Flores",
  "foto": "/foto/samsat-13.jpg",
  "alamat": "Jl. ...",
  "lat": -8.84, "lng": 121.66,
  "maps": "https://maps.app.goo.gl/...",
  "sosmed": [["instagram", "https://www.instagram.com/..."]],
  "pejabat": [
    ["Nama Kepala", "Kepala UPTD"],
    ["Nama", "Kasubag Tata Usaha"],
    ["Nama", "Kasi Pendapatan"],
    ["Nama", "Kasi Penagihan"]
  ]
}
```

### `data/kantor-pusat.json` — satu objek

Kolomnya sama seperti Samsat, tanpa `id` dan `pulau`. Saat ini kantor pusat memiliki
Instagram (`@bpad_ntt`), TikTok (`@bpadntt`), dan Facebook.

### Warna pulau

Diatur di konstanta `WARNA` dalam `static/index.html`. Jika menambah pulau baru, tambahkan juga warnanya di sana.

---

## 7. Instalasi & Menjalankan

Panduan lengkap untuk pemula: [MENJALANKAN-RUST.md](MENJALANKAN-RUST.md).

```bash
git clone https://github.com/ihsanmokhsen/sebaran-samsat.git
```

```bash
cd sebaran-samsat
```

```bash
cargo run --release
```

Buka http://127.0.0.1:8080.

| Variabel | Default | Fungsi |
|---|---|---|
| `HOST` | `127.0.0.1` | Alamat yang didengarkan. `0.0.0.0` = bisa diakses dari jaringan |
| `PORT` | `8080` | Port server |

> Program harus dijalankan **dari folder project** (folder yang berisi `foto/`), karena
> foto dibaca dari path relatif `foto/...`.

---

## 8. Deploy ke Server Linux

### 8.1 Build di server

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
git clone https://github.com/ihsanmokhsen/sebaran-samsat.git /opt/sebaran-samsat
```

```bash
cd /opt/sebaran-samsat && cargo build --release
```

### 8.2 Jalankan sebagai layanan (systemd)

Buat file `/etc/systemd/system/sebaran-samsat.service`:

```ini
[Unit]
Description=Sebaran Samsat NTT
After=network.target

[Service]
WorkingDirectory=/opt/sebaran-samsat
ExecStart=/opt/sebaran-samsat/target/release/sebaran-samsat-ntt
Environment=HOST=127.0.0.1
Environment=PORT=8080
Restart=always
User=www-data

[Install]
WantedBy=multi-user.target
```

`WorkingDirectory` wajib diisi agar folder `foto/` ditemukan.

```bash
sudo systemctl daemon-reload && sudo systemctl enable --now sebaran-samsat
```

### 8.3 Domain & HTTPS (nginx)

Server Rust ini tidak menangani HTTPS, jadi pasang nginx di depannya:

```nginx
server {
    server_name samsat.bpadntt.cloud;   # ganti dengan domain Anda
    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
    }
}
```

Lalu aktifkan HTTPS, misalnya dengan Certbot:

```bash
sudo certbot --nginx -d samsat.bpadntt.cloud
```

HTTPS juga diperlukan bila nanti ditambah fitur lokasi pengguna (geolocation).

### 8.4 Memperbarui aplikasi di server

```bash
cd /opt/sebaran-samsat && git pull && cargo build --release && sudo systemctl restart sebaran-samsat
```

---

## 9. Pemeliharaan Data

| Tugas | File | Perlu build ulang? |
|---|---|:---:|
| Ganti pejabat (mutasi/pelantikan) | `data/samsat.json` → `pejabat` | Ya |
| Ganti alamat / geser titik | `data/samsat.json` → `alamat`, `lat`, `lng`, `maps` | Ya |
| Tambah akun sosmed | `data/samsat.json` → `sosmed` | Ya |
| Ganti file foto yang sudah ada | `foto/samsat-NN.jpg` (nama sama) | Tidak |
| Tambah foto untuk Samsat yang belum punya | simpan file di `foto/` + isi kolom `foto` | Ya |
| Ganti data kantor pusat | `data/kantor-pusat.json` | Ya |

### Mengambil koordinat dari Google Maps
1. Buka lokasi kantor di Google Maps.
2. Klik kanan pada titik kantor. Baris pertama menu adalah koordinat, mis. `-8.8432, 121.6612`.
3. Klik untuk menyalin. Angka pertama → `lat`, angka kedua → `lng`.

### Foto
- Format `.jpg`, `.png`, atau `.webp`. Disarankan lebar ±800 px dan ukuran < 200 KB.
- Penamaan: `samsat-NN.jpg` dengan `NN` = `id` dua digit (mis. `samsat-07.jpg`).

### Cek JSON sebelum build

```bash
python3 -m json.tool data/samsat.json > /dev/null && echo "JSON valid"
```

Jika JSON salah (mis. koma kurang), program tetap bisa di-build tetapi peta akan kosong di browser.

---

## 10. Mengubah Tampilan

Semua gaya ada di blok `<style>` pada `static/index.html`.

| Ingin mengubah… | Ubah di |
|---|---|
| Tingkat transparansi kaca | `--kaca-bg` (mis. `rgba(255,255,255,.85)` agar lebih pekat) |
| Kekuatan blur | `--kaca-blur` |
| Warna utama (navy) | `--navy` |
| Warna titik per pulau | konstanta `WARNA` (JavaScript) |
| Warna titik kantor pusat | `WARNA_PUSAT` |
| Zoom mulai munculnya nama | `ZOOM_NAMA` (default 10) |
| Tinggi relief 3D | `exaggeration: 1.5` di `setTigaD()` |
| Teks kredit di pojok peta | `customAttribution` |

---

## 11. Sumber Data & Lisensi

| Data | Sumber |
|---|---|
| Pejabat & alamat UPTD | https://bapenda.nttprov.go.id/profil |
| Koordinat | Tautan "Buka Maps" resmi pada halaman di atas (Google Maps) |
| Koordinat Samsat Betun (Malaka) | Pencarian Google Maps, karena belum ada tautan resmi |
| Foto kantor | Foto utama tiap tempat di Google Maps (kiriman pengguna / Street View) |
| Peta satelit | © Esri, Maxar, Earthstar Geographics |
| Peta standar | © OpenStreetMap contributors |
| Relief 3D | AWS Terrain Tiles (Mapzen) |
| Gedung 3D | OpenFreeMap (data OpenStreetMap) |
| Library peta | MapLibre GL JS (lisensi BSD-3) |

> **Penting:** foto kantor dari Google Maps adalah milik pengunggahnya masing-masing.
> Untuk penggunaan publik resmi, ganti dengan foto dokumentasi milik BPAD.

---

## 12. Keterbatasan & Rencana Pengembangan

### Keterbatasan saat ini
- **Foto belum ada** untuk 7 Samsat: Lewoleba, Bajawa, Ruteng, Labuan Bajo, Waibakul,
  Waikabubak, Tambolaka.
- **Sosmed baru tercatat** untuk 4 Samsat (Kota Kupang, Kab. Kupang, Atambua, Larantuka)
  dan kantor pusat.
- **Titik Samsat Betun** berasal dari pencarian, belum dari tautan resmi Bapenda.
- **Ubin peta dan MapLibre** dimuat dari layanan luar, jadi aplikasi butuh internet.
- **Perubahan data** masih dilakukan dengan mengedit JSON lalu build ulang.
- **Server** tidak melayani HTTPS sendiri (pakai nginx, lihat bagian 8.3).

### Rencana pengembangan
1. Cari Samsat terdekat berdasarkan lokasi pengguna.
2. Jam layanan dan nomor telepon/WhatsApp tiap kantor.
3. Lokasi Samsat Keliling, Gerai Samsat, dan Samsat Desa.
4. Grafik penerimaan PKB per wilayah.
5. Mode gelap.
6. Ekspor daftar kantor & pejabat ke Excel/PDF.
7. Halaman admin untuk mengubah data tanpa mengedit JSON (latihan Rust: `serde`, `axum`).
