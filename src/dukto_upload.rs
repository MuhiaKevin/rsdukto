use std::fs::File;
use byte_order::NumberWriter;

// FIX: losts of unnecessary vecs, allocations
fn build_head() -> Vec<u8> {
    let mut buf = vec![0u8; 8];
    buf.insert(0, 0);

    let f = File::open("Cargo.toml").unwrap();
    let metadata = f.metadata().unwrap();
    let file_size = metadata.len() as u32;

    let mut le_writer = NumberWriter::with_order(byte_order::ByteOrder::LE, &mut buf[..]);
    le_writer.write_u32(file_size).unwrap();


    buf
}


fn full_packet() -> Vec<u8> {
    let file_name = "Cargo.toml";
    let mut header: Vec<u8> = vec![];
    let mut full_packet: Vec<u8> = vec![];
    let mut buf = vec![0u8; 7];

    buf[0] = 1;

    let mut code = build_head();
    let mut tail = code.clone();

    header.append(&mut buf);
    header.append(&mut code);


    full_packet.append(&mut header);
    full_packet.extend(file_name.as_bytes());
    full_packet.append(&mut tail);


    println!("{:?}", full_packet);

    full_packet
}


fn send_file() {
}
