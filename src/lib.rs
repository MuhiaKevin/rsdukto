mod dukto_download;
mod client_discovery;


use std::sync::mpsc;
use std::net::SocketAddr;
use anyhow::Result;


#[derive(Debug)]
struct DuktoClientMessage {
    message: String,
    address: SocketAddr,
}


pub fn run() -> Result<()> {
    let (sender, reciever) = mpsc::channel();
    let mut clients = std::collections::HashMap::new();

    client_discovery::discover_clients(sender);

    // while let Ok(dukto_client)  = reciever.recv() {
    //     let _ = &clients.insert(
    //         dukto_client.message,
    //         dukto_client.address,
    //     );
    //
    //     println!("Result: {:?}\n\n", clients);
    // }


    // spawn threads that sends files, folders or text to other dukto clients
    for dukto_client in reciever {
        // 1. check if client is in map and add the client to hashmap if not in
        // 2. if client not in hashmap, add the client to hash map and send the file/folder
        let _ = &clients.insert(
            dukto_client.message,
            dukto_client.address,
        );

        println!("Result: {:?}\n\n", clients);
    }

    //dukto_download::download()?;
    Ok(())
}
