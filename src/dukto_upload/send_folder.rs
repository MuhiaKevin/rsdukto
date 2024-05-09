use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::net::TcpStream;
use std::os::unix::fs::MetadataExt;
use std::io::{self, Read, Write};
use std::path::Path;

use byte_order::NumberWriter;

use crate::PORT;

// TODO: Send folder given absolute or relative path to folder
// FIX: For now can only send using absolute path


#[derive(Debug)]
struct SendFolder {
    folder_name: String,
    intial_packet: Vec<u8>,
    files_and_their_packets:  HashMap<String, Vec<u8>>,
}


pub fn send_folder(addr: String) -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let folder_name = std::path::Path::new(&args[1]);

    let send_folder = create_folder_info(folder_name);
    let mut buffer = [0u8; 1024]; // 1 KB buffer

    let addr = format!("{}:{}", addr, PORT);

    let mut stream = TcpStream::connect(addr)?;
    println!("Connected to server");
    // let mut total = 0;

    let _ = stream.write(&send_folder.intial_packet);

    let mut closure =  |entry: &DirEntry|  {
        if entry.path().exists() && entry.path().is_file() && !entry.path().is_symlink() {
            let path = entry.path();
            let file_path_str = path.to_str().unwrap();
            let split_by_folder_name = file_path_str
                .split_once(&send_folder.folder_name)
                .unwrap();

            let new_path = send_folder.folder_name.clone() + split_by_folder_name.1;
            println!("{new_path:?}");


            // get file pack 
            if let Some(file_pack) = send_folder.files_and_their_packets.get(&new_path) {
                // println!("File: Pack {file_pack:?}");
                // send file_pack
                let _ = stream.write(&file_pack);
                
                // open file
                let mut input_file = File::open(file_path_str).unwrap();

                // Stream file
                loop {
                    let bytes_read = input_file.read(&mut buffer).unwrap();
                    if bytes_read == 0 {
                        break;
                    }

                    // FIX: This error shows up when sending certain folders
                    // thread '<unnamed>' panicked at src/dukto_upload/send_folder.rs:67:61:
                    // called `Result::unwrap()` on an `Err` value: Os { code: 104, kind: ConnectionReset, message: "Connection reset by peer" }
                    // The other dukto client resets the connection
                    stream.write_all(&buffer[..bytes_read]).unwrap();
                }

                println!("File has been successfully sent.");
            }
        }
    };

    let show_entry: Box<&mut dyn FnMut(&DirEntry)> = Box::new(&mut closure);
    visit_dirs(folder_name, *show_entry).unwrap();

    Ok(())
}

// pub fn main() {
//     let args: Vec<_> = std::env::args().collect();
//     let path = std::path::Path::new(&args[1]);
//
//     // for entry in path.read_dir().expect("read_dir call failed") {
//     //     if let Ok(entry) = entry {
//     //         println!("{:?}", entry.path());
//     //     }
//     // }
//
//     // let show_entry: &dyn Fn(&DirEntry) = &|entry: &DirEntry|  println!("{:?}", entry.path());
//     //
//     // visit_dirs(path, show_entry ).unwrap()
//
//     let send_folder = create_folder_info(path);
//     // println!("{:#?}", send_folder);
//
//     //
//     // let args: Vec<_> = std::env::args().collect();
//     // let folder_name = std::path::Path::new(&args[1]);
//     //
//     //
//     // let send_folder = create_folder_info(folder_name);
//     let mut buffer = [0u8; 1024]; // 1 KB buffer
//
//     // let addr = format!("{}:{}", addr, PORT);
//
//     // let mut stream = TcpStream::connect(addr)?;
//     // println!("Connected to server");
//     // // let mut total = 0;
//     //
//     // let _ = stream.write(&send_folder.intial_packet);
//
//
//     let mut closure =  |entry: &DirEntry|  {
//         if entry.path().exists() && entry.path().is_file() && !entry.path().is_symlink() {
//             let path = entry.path();
//             let file_path_str = path.to_str().unwrap();
//             let split_by_folder_name = file_path_str
//                 .split_once(&send_folder.folder_name)
//                 .unwrap();
//
//             let new_path = send_folder.folder_name.clone() + split_by_folder_name.1;
//             println!("{new_path:?}");
//
//
//             // get file pack 
//             if let Some(file_pack) = send_folder.files_and_their_packets.get(&new_path) {
//                 println!("File: Pack {file_pack:?}");
//                 // send file_pack
//                 // let _ = stream.write(&file_pack);
//                 
//                 // open file
//                 let mut input_file = File::open(file_path_str).unwrap();
//
//                 // Stream file
//                 loop {
//                     let bytes_read = input_file.read(&mut buffer).unwrap();
//                     if bytes_read == 0 {
//                         break; // End of file
//                     }
//                     stream.write_all(&buffer[..bytes_read])?; // Send the chunk to the server
//                     // total += bytes_read;
//                     // println!("STREAAAAAAAAAAAAAAAMING FILE: SENT  {total} bytes");
//                 }
//
//                 println!("File has been successfully sent.");
//             }
//         }
//     };
//
//     let show_entry: Box<&mut dyn FnMut(&DirEntry)> = Box::new(&mut closure);
//     visit_dirs(path, *show_entry).unwrap();
// }



fn create_folder_info(folder_name: &Path) -> SendFolder {
    let mut full_packet: Vec<u8> = vec![];
    let mut total_num_of_files = vec![0u8; 8];
    let mut total_size_of_folder = vec![0u8; 8];
    let mut end_bytes = vec![0xffu8; 8];

    let root_name = folder_name.file_name().unwrap().
        to_str().unwrap();


    let mut total_size: u64 = 0;
    let mut num_of_files: u64 = 0;
    let mut files_and_their_packets: HashMap<String, Vec<u8>> = HashMap::new();

    let mut closure =  |entry: &DirEntry|  {
        if entry.path().exists() && entry.path().is_file() && !entry.path().is_symlink() {
            let path = entry.path();

            let file_size = path.metadata().unwrap().size();

            map_file_to_intial_packet(root_name, &path, file_size, &mut files_and_their_packets);

            total_size += file_size;
            num_of_files += 1;
        }
    };

    let show_entry: Box<&mut dyn FnMut(&DirEntry)> = Box::new(&mut closure);
    visit_dirs(folder_name, *show_entry).unwrap();

    // println!("{num_of_files} files with total size of : {total_size} bytes");
    // println!("{num_of_files} files with total size of : {total_size} bytes");
    // println!("{files_and_their_packets:#?}");



    let mut le_writer = NumberWriter::with_order(byte_order::ByteOrder::LE, &mut total_num_of_files[..]);
    le_writer.write_u32(num_of_files as u32).unwrap(); // FIX: remove casting to u32

    let mut le_writer = NumberWriter::with_order(byte_order::ByteOrder::LE, &mut total_size_of_folder[..]);
    le_writer.write_u32(total_size as u32).unwrap(); // FIX: remove casting to u32


    full_packet.append(&mut total_num_of_files);
    full_packet.append(&mut total_size_of_folder);
    full_packet.extend(root_name.as_bytes());
    full_packet.push(0);
    full_packet.append(&mut end_bytes);

    SendFolder {
        folder_name: root_name.to_string(),
        intial_packet: full_packet,
        files_and_their_packets
    }
}


fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}



// TODO: write test 
fn map_file_to_intial_packet(folder_name: &str, file_path: &Path, file_size: u64, files_and_their_packets: &mut HashMap<String, Vec<u8>>)  {
    let split_by_folder_name = file_path.to_str().unwrap()
        .split_once(folder_name).unwrap();

    let new_path = folder_name.to_string() + split_by_folder_name.1;

    let mut full_packet: Vec<u8> = vec![];
    let mut total_size_bytes: Vec<u8> = vec![0u8; 8];

    full_packet.extend(new_path.as_bytes());
    full_packet.push(0);


    let mut le_writer = NumberWriter::with_order(byte_order::ByteOrder::LE, &mut total_size_bytes[..]);
    le_writer.write_u32(file_size as u32).unwrap(); // FIX: remove casting to u32

    full_packet.append(&mut total_size_bytes);

    // println!("{full_packet:?}");
    // panic!("Just crash");

    files_and_their_packets.insert(new_path, full_packet);
}
