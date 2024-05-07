mod dukto_download;
mod client_discovery;
mod dukto_upload;


use std::sync::mpsc;
use anyhow::Result;
use dukto_upload::send_file;

// https://stackoverflow.com/questions/56535634/propagating-errors-from-within-a-closure-in-a-thread-in-rust

#[derive(Debug)]
struct DuktoClientMessage {
    device_name: String,
    address: String,
}


const PORT: u32 = 4644;
const MY_DEVICE_NAME: &'static str = "Chifu at Kizunu (Rustlang)";


pub fn run() -> Result<()> {
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

        if device_name != "Bye Bye" && device_name != MY_DEVICE_NAME {
            if !clients.contains_key(&device_name) {
                let _ = &clients.insert(
                    device_name,
                    dukto_client.address.clone(),
                );

                // TODO: use threadpool instead of this
                std::thread::spawn(move|| {
                    send_file::send_file(dukto_client.address)
                });
            }
        }

        println!("Result: {:#?}\n\n", clients);
    }

    Ok(())
}
