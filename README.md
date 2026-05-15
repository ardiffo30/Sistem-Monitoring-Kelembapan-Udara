# 🌤️ Sistem Monitoring Kelembapan Udara

Aplikasi monitoring kelembapan udara berbasis terminal menggunakan bahasa pemrograman **Rust**.

---

## 📋 Informasi Project

| | |
|---|---|
| **Mata Kuliah** | Algoritma dan Pemrograman |
| **Program Studi** | Teknik Instrumentasi |
| **Institusi** | Institut Teknologi Sepuluh Nopember (ITS) |
| **Semester** | Genap 2025/2026 |
| **Dosen Pengampu** | Ahmad Radhy, S.Si., M.Si. |

---

## 👥 Anggota Kelompok

| Nama | NRP |
|------|-----|
| [Nama Mahasiswa 1] | [NRP 1] |
| [Nama Mahasiswa 2] | [NRP 2] |

---

## 📝 Deskripsi Project

Sistem Monitoring Kelembapan Udara adalah aplikasi berbasis terminal (*console application*) yang mensimulasikan monitoring kelembapan udara menggunakan sensor **DHT22**. Sistem ini dirancang untuk memantau kondisi kelembapan udara secara berkala dan mengambil tindakan kontrol otomatis berdasarkan threshold yang telah ditentukan.

### Fitur Utama:
- ✅ Membaca dan mengkalibrasi data sensor secara otomatis
- ✅ Menghitung **Moving Average** dari 5 data terakhir sebagai filter noise
- ✅ Mendeteksi kondisi udara: Normal, Warning, dan Danger
- ✅ Mengaktifkan **Humidifier** otomatis jika udara terlalu kering
- ✅ Mengaktifkan **Dehumidifier** otomatis jika udara terlalu lembap
- ✅ Validasi input sensor untuk mencegah pembacaan error
- ✅ Menampilkan ringkasan akhir monitoring

---

## 🔧 Spesifikasi Sistem

### Sensor
| Parameter | Nilai |
|-----------|-------|
| Tipe Sensor | DHT22 |
| Satuan | %RH (Relative Humidity) |
| Rentang Valid | 0 - 100 %RH |
| Gain Kalibrasi | 1.01 |
| Offset Kalibrasi | 0.5 %RH |

### Threshold Kontrol
| Kondisi | Rentang | Aksi |
|---------|---------|------|
| 🟢 Normal | 30% - 80% RH | Semua OFF |
| 🟡 Warning Kering | < 30% RH | Humidifier ON |
| 🔴 Danger Kering | < 20% RH | Humidifier ON + Alarm |
| 🟡 Warning Lembap | > 80% RH | Dehumidifier ON |
| 🔴 Danger Lembap | > 90% RH | Dehumidifier ON + Alarm |
| ⚫ Sensor Error | < 0% atau > 100% | Semua OFF |

---

## 🏗️ Struktur Program (OOP)

Program menggunakan konsep **Object-Oriented Programming** dengan 3 objek utama:

```
MonitoringSystem
├── Sensor (DHT22)
│   ├── is_valid()       → validasi rentang nilai
│   ├── record()         → simpan riwayat data
│   ├── moving_average() → komputasi moving average
│   ├── deviation()      → hitung deviasi data
│   └── calibrate()      → kalibrasi linear sensor
│
└── Controller
    ├── evaluasi()        → tentukan status & kontrol
    └── display_status()  → tampilkan output
```

---

## 📐 Komputasi Numerik

### 1. Kalibrasi Linear
```
RH_cal = (raw × gain) + offset
RH_cal = (raw × 1.01) + 0.5
```

### 2. Moving Average
```
MA = (RH₁ + RH₂ + ... + RHₙ) / n     (n = maks. 5 data)
```

### 3. Deviasi
```
Deviasi = |RH_current - Moving Average|
```

---

## 🚀 Cara Menjalankan Program

### Prasyarat
Pastikan **Rust** sudah terinstall di laptop kamu. Cek dengan:
```bash
rustc --version
cargo --version
```
Kalau belum ada, install di: https://rustup.rs

### Langkah Menjalankan
```bash
# 1. Clone repository ini
git clone https://github.com/username/monitoring-kelembapan-udara-ets.git

# 2. Masuk ke folder project
cd monitoring-kelembapan-udara-ets

# 3. Jalankan program
cargo run
```

### Contoh Input untuk Testing
| Input | Kondisi yang Diuji |
|-------|-------------------|
| `65` | Normal |
| `25` | Warning Kering |
| `15` | Danger Kering |
| `85` | Warning Lembap |
| `95` | Danger Lembap |
| `110` | Sensor Error |

---

## 📁 Struktur Repository

```
monitoring-kelembapan-udara-ets/
│
├── src/
│   └── main.rs          → Source code Rust utama
│
├── laporan/
│   ├── laporan_ets.tex  → Source laporan LaTeX
│   └── laporan_ets.pdf  → Laporan dalam format PDF
│
├── flowchart/
│   └── flowchart.png    → Flowchart sistem
│
├── screenshot/
│   └── demo.png         → Screenshot hasil program
│
├── Cargo.toml           → Konfigurasi project Rust
└── README.md            → Dokumentasi project
```

---

## 📸 Screenshot Program

![Demo Program](screenshot/demo.png)

---

## 🔄 Flowchart Sistem

![Flowchart](flowchart/flowchart.png)

---

## 📊 Contoh Output Program

```
+==========================================+
   SISTEM MONITORING KELEMBAPAN UDARA
   Teknik Instrumentasi - ITS
+==========================================+

Info Sensor:
  Sensor   : DHT22
  Satuan   : %RH
  Rentang  : 0.0 - 100.0 %RH
  Threshold: Kering <30% (Warning) | <20% (Danger)
             Lembap >80% (Warning) | >90% (Danger)

Masukkan jumlah siklus pembacaan: 3

--- Siklus 1 dari 3 ---
  Input kelembapan udara raw (%RH): 65
  Input Raw    : 65.00 %RH
  Terkalibrasi : 66.15 %RH
  Moving Avg   : 66.15 %RH
  Deviasi      : 0.00 %RH
  Status       : [ NORMAL        ] Udara optimal (30-80%)
  Alarm        : OFF
  Humidifier   : OFF
  Dehumidifier : OFF
```

---

## 📚 Referensi

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [DHT22 Sensor Datasheet](https://www.sparkfun.com/datasheets/Sensors/Temperature/DHT22.pdf)
- Bentley, J. P. (2005). *Principles of Measurement Systems*. Pearson Education.

---

> **ETS Algoritma dan Pemrograman** — Teknik Instrumentasi ITS, Semester Genap 2025/2026
