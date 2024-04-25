mod dukto_download;
mod client_discovery;
pub mod dukto_upload;


use std::sync::mpsc;
use anyhow::Result;
// use dukto_upload::send_file::send_file;
use dukto_upload::send_multiple_files::send_multiple_files;


#[derive(Debug)]
struct DuktoClientMessage {
    message: String,
    address: String,
}


pub fn run() -> Result<()> {
    let (sender, reciever) = mpsc::channel();
    let mut clients = std::collections::HashMap::new();


    // std::thread::spawn(|| {
    //     dukto_download::download().unwrap();
    // });

    client_discovery::discover_clients(sender);

    // while let Ok(dukto_client)  = reciever.recv() {
    //     let _ = &clients.insert(
    //         dukto_client.message,
    //         dukto_client.address,
    //     );
    //
    //     println!("Result: {:?}\n\n", clients);
    // }


    for dukto_client in reciever {
        let message = &dukto_client.message[1..];
        let message = message.to_string();

        if message != "Bye Bye" {
            // 1. check if client is in map and add the client to hashmap if not in
            if !clients.contains_key(&message) {
                // 2. if client not in hashmap, add the client to hash map and send the file/folder
                let _ = &clients.insert(
                    message,
                    dukto_client.address.clone(),
                );

                // 3. spawn threads that sends files, folders or text to other dukto clients
                std::thread::spawn(move|| {
                    // send_file(dukto_client.address)
                    send_multiple_files(dukto_client.address)
                });
            }
        }

        println!("Result: {:#?}\n\n", clients);
    }

    Ok(())
}
