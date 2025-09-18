# water (WebAssembly TExt foRmat compiler)

`water` มีเป้าหมายเพื่อเป็นคอมไพเลอร์ WebAssembly Text Format ที่มีขนาดเล็กและมีประสิทธิภาพ

## คุณสมบัติ

- คอมไพเลอร์ WebAssembly Text Format ที่มีประสิทธิภาพ
- เขียนด้วยภาษา Rust เพื่อความปลอดภัยและประสิทธิภาพ
- ออกแบบมาให้มีขนาดเล็กและรวดเร็ว

## การติดตั้ง

```bash
cargo build --release
```

## การใช้งาน

```bash
./target/release/water input.wat
```

## ใบอนุญาต

ดูไฟล์ LICENSE สำหรับรายละเอียดเพิ่มเติม