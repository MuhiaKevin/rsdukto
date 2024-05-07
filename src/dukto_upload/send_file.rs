use std::fs::File;
use byte_order::NumberWriter;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use crate::PORT;

// GET FILENAME FROM CLAP INSTEAD OF THIS
const FILE_NAME: &'static str = "";

struct SendFile{
    file_handle: File,
    intial_packet: Vec<u8>,
}

impl SendFile {
    // FIX: losts of unnecessary vecs, allocations
    fn build(filename: &str) -> Self {
        // build head section
        let file_handle = File::open(filename).unwrap();
        let mut code = vec![0u8; 8];

        let metadata = file_handle.metadata().unwrap();
        let file_size = metadata.len() as u32;

        let mut le_writer = NumberWriter::with_order(byte_order::ByteOrder::LE, &mut code[..]);
        le_writer.write_u32(file_size).unwrap();


        // build whole section
        let mut header: Vec<u8> = vec![];
        let mut full_packet: Vec<u8> = vec![];
        let mut buf = vec![0u8; 7];

        buf[0] = 1;

        code.insert(0, 0);

        let mut tail = code.clone();
        header.append(&mut buf);
        header.append(&mut code);

        full_packet.append(&mut header);
        full_packet.extend(filename.as_bytes());
        full_packet.append(&mut tail);

        Self {
            file_handle,
            intial_packet: full_packet
        }
    }
}



pub fn send_file(addr: String) -> io::Result<()> {
    let addr = format!("{}:{}", addr, PORT);

    let mut input_file = SendFile::build(FILE_NAME);
    let mut buffer = [0u8; 1024]; // 1 KB buffer

    let mut stream = TcpStream::connect(addr)?;
    println!("Connected to server");
    // let mut total = 0;

    let _ = stream.write(&input_file.intial_packet);

    loop {
        let bytes_read = input_file.file_handle.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }
        stream.write_all(&buffer[..bytes_read])?; // Send the chunk to the server
        // total += bytes_read;
        // println!("STREAAAAAAAAAAAAAAAMING FILE: SENT  {total} bytes");
    }

    println!("File has been successfully sent.");

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_inital_packet() {
        let file_packet = vec![1, 0, 0, 0, 0, 0, 0, 0, 199, 93, 248, 1, 0, 0, 0, 0, 76, 111, 99, 97, 108, 83, 101, 110, 100, 45, 49, 46, 49, 52, 46, 48, 46, 97, 112, 107, 0, 199, 93, 248, 1, 0, 0, 0, 0];
        let input_file = SendFile::build(FILE_NAME);

        assert_eq!(file_packet, input_file.intial_packet) }
}
