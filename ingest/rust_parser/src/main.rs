use std::fs::File;
use std::io::Read;

struct Packet {
    msg_type: u8,
    length: u8,
    data: Vec<u8>,
    checksum: u8,
}

fn main() {
    println!("Opening device...");

    let mut file = File::open("/tmp/device_stream")
        .expect("Failed to open device stream");

    println!("Device opened. Reading data...");

    let mut buffer = [0u8; 64];
    let mut stream: Vec<u8> = Vec::new();

    loop {
        let bytes_read = file.read(&mut buffer)
            .expect("Failed to read from stream");

        if bytes_read > 0 {
            // Add new data to stream buffer
            stream.extend_from_slice(&buffer[..bytes_read]);

            // Try to parse packets
            while let Some(packet) = try_parse_packet(&mut stream){
                println!("Parsed packet: {:?}", packet.msg_type);
            }
        }
    }
}

fn try_parse_packet(stream: &mut Vec<u8>) -> Option<Packet> {
    // Need at least header + type + length
    if stream.len() < 3 {
        return None;
    }

    // Find header (0xAA)
    if stream[0] != 0xAA {
        stream.remove(0);
        return None;
    }

    let msg_type = stream[1];
    let length = stream[2] as usize;

    let total_length = 3 + length + 1; // header + type + length + data + checksum

    if stream.len() < total_length {
        return None; // wait for more data
    }

    let data = stream[3..3 + length].to_vec();
    let checksum = stream[3 + length];

    // Remove parsed packet from stream
    stream.drain(0..total_length);

    Some(Packet {
        msg_type,
        length: length as u8,
        data,
        checksum,
    })
}