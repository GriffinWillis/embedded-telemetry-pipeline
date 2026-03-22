# Embedded Telemetry Pipeline

## Overview
This project simulates an embedded telemetry system with binary packet ingestion, parsing, and processing.

## Components
- Rust-based packet parser (ingest layer)
- Planned processing pipeline
- Storage and visualization layers (in progress)

## Structure
- ingest/ → low-level parsing (Rust)
- processing/ → data transformation
- storage/ → database schema
- docs/ → system design

## Input Stream
By default, the parser reads from:
`data/sample_stream.bin`

To use a live device stream:

```bash
cargo run -- /tmp/device_stream