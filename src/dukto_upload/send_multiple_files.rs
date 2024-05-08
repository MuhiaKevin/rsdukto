use std::env;
use std::fs::File;
use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::path::Path;
use byte_order::NumberWriter;
use crate::PORT;

pub fn send_multiple_files(addr: String) -> io::Result<()>  {
    let mut buffer = [0u8; 1024]; // 1 KB buffer
    let mut size_of_files_sent = 0;
    let addr = format!("{}:{}", addr, PORT);
    
    let args: Vec<String> = env::args().collect();

    // TODO: GET files from commandline and use 'Path'
    let file_paths = &args[1..];
    println!("{file_paths:?}");

    let total_size =  get_total_size(&file_paths).unwrap();

    let mut stream = TcpStream::connect(addr)?;
    println!("Connected to server");


    let first_pack = create_intial_packet(total_size, file_paths.len());
    let _ = stream.write(&first_pack);


    for file in file_paths {
        let mut input_file = File::open(file)?;

        let metadata = input_file.metadata().unwrap();
        let file_size = metadata.len();

        let file_pack = create_individual_file_packet(file, file_size);
        let _ = stream.write(&file_pack);

        // FIX: USE BUFREADER HERE
        loop {
            let bytes_read = input_file.read(&mut buffer)?;
            if bytes_read == 0 {
                break; 
            }
            stream.write_all(&buffer[..bytes_read])?;
            size_of_files_sent += bytes_read;

            println!("Sent {size_of_files_sent}  of file {file}");
        }

        println!("File has been successfully sent.");
    }

    Ok(())
}

fn get_total_size(paths: &[String]) -> Result<u64, io::Error> {
    let mut total_size: u64 = 0;

    for path in paths {
        let metadata = std::fs::metadata(Path::new(path))?;
        total_size += metadata.len();
    }

    Ok(total_size)
}


 fn create_intial_packet(total_size_of_files: u64, num_of_files: usize) -> Vec<u8>{
    let mut first_pack: Vec<u8> = vec![];
    let mut num_of_files_buf = vec![0u8; 8];
    let mut total_size_buf = vec![0u8; 8];

    num_of_files_buf[0] = num_of_files as u8;

    let mut le_writer = NumberWriter::with_order(byte_order::ByteOrder::LE, &mut total_size_buf[..]);
    le_writer.write_u64(total_size_of_files).unwrap();

    first_pack.append(&mut num_of_files_buf);
    first_pack.append(&mut total_size_buf);

    first_pack
}


fn create_individual_file_packet(file_name: &str, file_size: u64) -> Vec<u8> {
    // FIX: Get the real name instead of the file path

    let mut file_pack: Vec<u8> = vec![];
    let mut file_size_buf = vec![0u8; 8];

    let mut le_writer = NumberWriter::with_order(byte_order::ByteOrder::LE, &mut file_size_buf[..]);
    le_writer.write_u64(file_size).unwrap();


    file_pack.extend(get_filename(file_name).as_bytes());
    file_pack.push(0);
    file_pack.append(&mut file_size_buf);
    
    file_pack
}


fn get_filename(filename: &str) -> &str {
    let file_path = Path::new(filename);
    let true_filename = file_path.file_name().unwrap().to_str().unwrap();

    println!("TRUE_FILENAME {:?}", true_filename);
    println!("FILE PATH: {}", filename);

    true_filename
}
