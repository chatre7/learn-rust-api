# คู่มือใช้งาน (ภาษาไทย) — Rust REST API (Axum + SQLx + PostgreSQL)

โปรเจกต์นี้เป็นตัวอย่าง REST API ด้วย Rust/Axum เชื่อมต่อฐานข้อมูล PostgreSQL ผ่าน SQLx พร้อมระบบ Migration แบบฝังในตัว (embedded). มี CRUD สำหรับทรัพยากร "Book" และมี Dockerfile/docker-compose สำหรับใช้งานท้องถิ่นได้สะดวก

## สิ่งที่ต้องมี (Prerequisites)
- Rust toolchain (แนะนำติดตั้งผ่าน rustup)
- Docker Desktop หรือ Docker Engine (สำหรับรัน PostgreSQL ง่าย ๆ ด้วย docker-compose)
- เครื่องมือเสริม (ทางเลือก): `curl` หรือ Postman สำหรับทดสอบ API

## โครงสร้างโปรเจกต์ (ย่อ)
- `src/domain` โครงสร้างข้อมูล (entities, DTOs)
- `src/repo` อินเตอร์เฟส repository และ implementation ด้วย SQLx
- `src/service` ธุรกิจลอจิกและ validation
- `src/handlers` Axum handlers + unit tests แบบ in-memory
- `src/infrastructure` การเชื่อมต่อ DB และตัวรัน migrations
- `src/routes.rs` ประกาศเส้นทาง (routes)
- `migrations/` ไฟล์ SQL สำหรับ migrations (รันอัตโนมัติเมื่อแอปรัน)

## เริ่มต้นแบบเร็ว (Quickstart)
1) สตาร์ทฐานข้อมูล PostgreSQL ด้วย Docker
   - คำสั่ง: `docker compose up -d db`
2) ตั้งค่า Environment
   - คัดลอก `.env.example` เป็น `.env`
   - ตรวจสอบ/แก้ไข `DATABASE_URL` ให้ตรงกับ DB ที่ใช้งาน เช่น `postgres://app:app@localhost:5432/app`
3) รันแอปพลิเคชัน
   - คำสั่ง: `cargo run`
   - แอปจะฟังพอร์ตจากตัวแปร `PORT` (ค่าเริ่มต้น 8080)
4) รันทดสอบ
   - คำสั่ง: `cargo test`
5) รันด้วยคอนเทนเนอร์ทั้งหมด
   - คำสั่ง: `docker compose up --build`

## ตัวอย่างการเรียกใช้งาน API (ด้วย curl)
- ตรวจสุขภาพระบบ
  - `curl http://localhost:8080/health`
- สร้าง Book ใหม่
  - `curl -X POST http://localhost:8080/books -H "content-type: application/json" -d '{"title":"Rust Book","author":"Ferris"}'`
- เรียกดูรายการ Book
  - `curl "http://localhost:8080/books?offset=0&limit=20"`
- เรียกดู Book ตาม id
  - `curl http://localhost:8080/books/<BOOK_ID>`
- อัปเดต Book (บางฟิลด์)
  - `curl -X PUT http://localhost:8080/books/<BOOK_ID> -H "content-type: application/json" -d '{"title":"New Title"}'`
- ลบ Book
  - `curl -X DELETE http://localhost:8080/books/<BOOK_ID>`

หมายเหตุ: โครงสร้าง Book
- id: UUID
- title: string
- author: string
- created_at, updated_at: เวลาแบบ UTC (TIMESTAMPTZ)

## คอนฟิกและพฤติกรรมสำคัญ
- การ Migrate: แอปจะรัน migrations จากโฟลเดอร์ `migrations/` โดยอัตโนมัติเมื่อสตาร์ท (ดู `src/infrastructure/db.rs`)
- CORS: อนุญาตทุก origin/method ตามที่กำหนดใน `main.rs` ผ่าน `CorsLayer`
- Validation: มีการตรวจสอบความถูกต้องที่ชั้น service (เช่น `title`/`author` ห้ามว่าง และมีความยาวสูงสุด)
- การจำกัดข้อมูล: endpoint list ใช้ `limit` ค่าที่ clamp อยู่ระหว่าง 1 ถึง 100 และ `offset` ไม่ติดลบ

## ตัวแปรสภาพแวดล้อม (Environment Variables)
- `DATABASE_URL` เช่น `postgres://app:app@localhost:5432/app`
- `PORT` พอร์ตที่แอปฟัง (ค่าเริ่มต้น 8080 ถ้าไม่ตั้ง)
- `RUST_LOG` ระดับ log (เช่น `info`, `debug`)

## เคล็ดลับ/ปัญหาที่พบบ่อย (Troubleshooting)
- ต่อฐานข้อมูลไม่ได้:
  - ตรวจสอบว่า container `db` รันอยู่ (`docker ps`)
  - ตรวจสอบค่า `DATABASE_URL` ให้ตรงกับพอร์ต/ผู้ใช้/รหัสผ่านของ Postgres
- ตารางไม่ถูกสร้าง:
  - เช็ค log ตอนสตาร์ทว่ามีการรัน migrations หรือไม่
  - ดูไฟล์ใน `migrations/*/up.sql`
- Windows/WSL:
  - ถ้าใช้งานผ่าน WSL ให้แน่ใจว่า Docker Desktop เปิด integration กับ WSL และเช็คการแมป network ให้เรียบร้อย

## การหยุดการทำงาน
- ถ้ารันด้วย `cargo run` กด `Ctrl+C` เพื่อปิดอย่างสุภาพ (graceful shutdown)
- ถ้ารันด้วย Docker Compose: `docker compose down`

## หมายเหตุสำหรับนักพัฒนา
- เพิ่มคอลัมน์/ตารางใหม่: สร้างโฟลเดอร์ migrations ใหม่ตามชื่อ timestamp และเพิ่มไฟล์ `up.sql`/`down.sql` แล้วแอปจะรัน migrations ให้เมื่อสตาร์ทครั้งถัดไป
- แยกชั้นส่วนรับผิดชอบชัดเจน: แก้ไข logic ที่ชั้น `service/` แก้ไขการเข้าถึง DB ที่ชั้น `repo/`
- มี unit tests ตัวอย่างทั้งที่ `handlers/` และ `service/`

## Endpoints สรุป
- `GET /health` -> "ok"
- `POST /books` -> สร้าง Book ใหม่
- `GET /books?offset=&limit=` -> รายการ Book แบบแบ่งหน้า
- `GET /books/:id` -> รายการ Book ตาม id
- `PUT /books/:id` -> อัปเดตบางฟิลด์ของ Book
- `DELETE /books/:id` -> ลบ Book ตาม id

ขอให้สนุกกับการเรียนรู้ Rust ครับ/ค่ะ!
