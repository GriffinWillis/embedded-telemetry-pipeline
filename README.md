# Embedded Telemetry Pipeline

A binary telemetry ingestion pipeline written in Rust. It reads a stream of bytes coming from an embedded device, parses the packets out of that stream, and validates each one using a checksum.

Built to practice systems programming concepts like binary protocols, multithreading, and message passing.

---

## How it works

The parser runs two threads that talk to each other through a channel:

```
Device/File → [Reader Thread] --channel--> [Parser Thread] → Parsed Packets
```

- **Reader thread** — opens the device stream (or a file), reads raw bytes in 64-byte chunks, and sends them down a channel
- **Parser thread** — receives those chunks, buffers them up, and pulls complete packets out one at a time

The two threads run independently so I/O never blocks parsing and vice versa.

---

## Packet format

Each packet follows a simple binary protocol:

| Byte(s)     | Field      | Description                        |
|-------------|------------|------------------------------------|
| `0`         | Header     | Always `0xAA` (sync byte)          |
| `1`         | Type       | Message type identifier            |
| `2`         | Length     | Number of data bytes that follow   |
| `3..3+n`    | Data       | Payload                            |
| `3+n`       | Checksum   | XOR of all previous bytes          |

The parser scans for the `0xAA` header byte, reads the length, waits until enough bytes have arrived, then verifies the checksum. Bad packets get dropped and the stream re-syncs automatically.

---

## Sample data

There's a small test stream at `data/sample_stream.bin` with two packets baked in:

```
AA 01 03 AA BB CC 75   →  Type=0x01, Data=[AA, BB, CC]
AA 02 02 10 20 9A      →  Type=0x02, Data=[10, 20]
```

---

## Running it

```bash
cd ingest/rust_parser

# Run with the sample stream
cargo run

# Run with a live device stream
cargo run -- /tmp/device_stream
```

Expected output:

```
Starting telemetry collector...
Reader thread started. Opening device...
Device opened. Reading data...
Parser thread started
Parsed packet: Packet { msg_type: 1, length: 3, data: [170, 187, 204], checksum: 117 }
Parsed packet: Packet { msg_type: 2, length: 2, data: [16, 32], checksum: 154 }
Reader thread finished
Parser thread finished
Program finished
```

---

## Project structure

```
embedded-telemetry-pipeline/
├── data/
│   └── sample_stream.bin     # Test binary stream
└── ingest/
    └── rust_parser/
        └── src/
            └── main.rs       # Reader + parser threads, packet struct
```

---

## Tech

- **Rust** (2024 edition, no external crates — stdlib only)
- `std::thread` for concurrency
- `std::sync::mpsc` for the message-passing channel
