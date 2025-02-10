# 🚗 **Clean Code Template Actix-Web - Rust Backend API** 🚗  

Selamat datang di **Clean Code Template Actix-Web**, backend API sederhana berbasis **Rust** menggunakan **Actix Web** dan **PostgreSQL** untuk manajemen data mobil sewa. Ikuti langkah-langkah berikut untuk meng-clone dan menjalankan aplikasi ini di mesin lokal kamu.

---

## 🛠️ **Langkah-Langkah Instalasi**

### 1. **Clone Repository Ini**
Buka terminal kamu dan jalankan perintah berikut untuk meng-clone repository:

```bash
git clone https://github.com/darmansrb/clean-code-rust-actixweb-postgresql.git
cd clean-code-rust-actixweb-postgresql
```

> 💡 **Tips:** Jangan lupa mengganti `darmansrb` dengan akun GitHub yang sesuai jika kamu menggunakan repository pribadi.

---

### 2. **Buat Database PostgreSQL**
Buka **PostgreSQL** dan buat database baru dengan nama sesuai kebutuhan kamu. Misalnya:

```sql
CREATE DATABASE bayarmobil;
```

---

### 3. **Buat Tabel `cars` di PostgreSQL**
Setelah membuat database, jalankan perintah berikut untuk membuat tabel **cars**. Pastikan kamu terhubung ke database yang baru dibuat.

```sql
CREATE TABLE cars (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    model VARCHAR NOT NULL,
    year INTEGER NOT NULL,
    price_per_day DECIMAL NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

> 💡 **Tips:** Jika terjadi kesalahan pada fungsi `gen_random_uuid()`, pastikan ekstensi **`pgcrypto`** sudah diaktifkan di PostgreSQL:
```sql
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
```

---

### 4. **Ubah File `.env`**
Salin file **`.env.example`** menjadi **`.env`**, lalu sesuaikan nilai variabel berikut sesuai konfigurasi PostgreSQL di mesin kamu:

```plaintext
DATABASE_URL=postgres://username:password@localhost:5432/nama_database
```

Contoh:
```plaintext
DATABASE_URL=postgres://postgres:secret@127.0.0.1:5432/bayarmobil
```

> 🖍️ **Keterangan Variabel:**
- **username**: Nama pengguna PostgreSQL
- **password**: Password pengguna
- **localhost**: IP address server PostgreSQL (ubah jika di server berbeda)
- **5432**: Port default PostgreSQL
- **nama_database**: Nama database yang kamu buat

---

### 5. **Jalankan Aplikasi**
Setelah semua konfigurasi selesai, jalankan perintah berikut di terminal untuk menjalankan aplikasi:

```bash
cargo run
```

Jika semuanya benar, kamu akan melihat log seperti:
```plaintext
Server running on http://127.0.0.1:8080
```

---

## 📡 **Mengakses API**
Kamu bisa mulai mengakses API melalui **Postman**, **curl**, atau aplikasi klien lainnya.

**Contoh Endpoint (GET semua mobil):**
```bash
curl http://127.0.0.1:8080/api/cars
```

---

## 🔧 **Masalah Umum dan Solusi**
- **Gagal menghubungkan ke database:**  
  - Periksa kembali variabel **`DATABASE_URL`** di file **`.env`**. Pastikan database sudah aktif dan username/password benar.
  
- **Ekstensi `gen_random_uuid()` tidak ditemukan:**  
  - Jalankan perintah berikut di PostgreSQL untuk mengaktifkan ekstensi:
    ```sql
    CREATE EXTENSION IF NOT EXISTS "pgcrypto";
    ```

---

## 🛡️ **Kontribusi**
Kami sangat menghargai kontribusi kamu! Silakan buat pull request jika ada penambahan atau perbaikan kode.

---

## 📞 **Dukungan**
Jika mengalami kendala, kamu bisa menghubungi kami di:
- 📧 **Email:** darmanrehiara13@gmail.com

🚀 **Selamat coding dan semoga berhasil menjalankan proyek ini!**
