mod dukto_download;
mod client_discovery;
pub mod dukto_upload;


use std::{env, sync::mpsc};
use anyhow::Result;
use dukto_upload::{send_file, send_multiple_files};

// https://stackoverflow.com/questions/56535634/propagating-errors-from-within-a-closure-in-a-thread-in-rust
// TODO: Add support for Microsoft Windows

#[derive(Debug)]
struct DuktoClientMessage {
    device_name: String,
    address: String,
}


const PORT: u32 = 4644;
const DEVICE_NAME: &'static str = "Chifu at Kizunu (Rustlang)";


pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Enter atlease 1 file or folder");
    }

    let (sender, reciever) = mpsc::channel();
    let mut clients = std::collections::HashMap::new();

    // TODO: Support downloading multiple files, folders and text
    // wait to download a single file
    std::thread::spawn(move || dukto_download::download().unwrap());

    // send and receive udp broadcast message for discovery
    client_discovery::discover_clients(sender);

    for dukto_client in reciever {
        let device_name = &dukto_client.device_name[1..];
        let device_name = device_name.to_string();

        if device_name != "Bye Bye" && device_name != DEVICE_NAME {
            if !clients.contains_key(&device_name) {
                let _ = &clients.insert(
                    device_name,
                    dukto_client.address.clone(),
                );

                // TODO: use threadpool instead of this
                if args.len() >= 3 {
                    std::thread::spawn(move|| {
                        send_multiple_files::send_multiple_files(dukto_client.address)
                    });

                } else {
                    std::thread::spawn(move|| {
                        send_file::send_file(dukto_client.address)
                    });
                }
            }
        }

        println!("Result: {:#?}\n\n", clients);
    }

    Ok(())
}
