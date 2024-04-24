use std::fs::File;
use byte_order::NumberWriter;
use std::io::{self, Read, Write};
use std::net::TcpStream;

const PORT: u32 = 4644;
const FILE_NAME: &'static str = "";

// FIX: losts of unnecessary vecs, allocations
fn build_head() -> Vec<u8> {
    let mut buf = vec![0u8; 8];

    let input_file = File::open(crate::dukto_upload::FILE_NAME).unwrap(); 
    let metadata = input_file.metadata().unwrap();
    let file_size = metadata.len() as u32;

    let mut le_writer = NumberWriter::with_order(byte_order::ByteOrder::LE, &mut buf[..]);
    le_writer.write_u32(file_size).unwrap();

    buf
}


fn full_packet() -> Vec<u8> {
    let file_name = crate::dukto_upload::FILE_NAME;
    let mut header: Vec<u8> = vec![];
    let mut full_packet: Vec<u8> = vec![];
    let mut buf = vec![0u8; 7];

    buf[0] = 1;

    let mut code = build_head();
    code.insert(0, 0);

    let mut tail = code.clone();
    header.append(&mut buf);
    header.append(&mut code);

    full_packet.append(&mut header);
    full_packet.extend(file_name.as_bytes());
    full_packet.append(&mut tail);

    println!("{:?}", full_packet);

    full_packet
}


// 1. check if file exists
// 2. get intial packet 
// 3. send initial packet to client 
// 4. stream the file to the client
// 5. TODO: support sending multiple files

fn stream_file_to_client(mut stream: TcpStream, file_path: &str) -> io::Result<()> {
    // Open the file for reading
    let mut file = File::open(file_path)?;
    
    // Buffer for reading chunks of the file
    let mut buffer = [0u8; 1024]; // 1 KB buffer
    
    // Read the file and send it to the client
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }
        stream.write_all(&buffer[..bytes_read])?; // Send the chunk to the client
    }

    Ok(())
}

pub fn send_file(addr: String) -> io::Result<()> {
    let addr = format!("{}:{}", addr, PORT);
    
  // Connect to the TCP server
    let mut stream = TcpStream::connect(addr)?;
    println!("Connected to server");
    let mut total = 0;


    let intial_packet =  full_packet();
    let _ = stream.write(&intial_packet);

    let mut input_file = File::open(crate::dukto_upload::FILE_NAME)?;
    let mut buffer = [0u8; 1024]; // 1 KB buffer


    // Read the file in chunks and send to the server
    loop {
        let bytes_read = input_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }
        stream.write_all(&buffer[..bytes_read])?; // Send the chunk to the server
        total += bytes_read;
        println!("STREAAAAAAAAAAAAAAAMING FILE: SENT  {total} bytes");
    }

    println!("File has been successfully sent.");

    Ok(())
}
