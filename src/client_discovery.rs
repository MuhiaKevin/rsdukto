use std::net::{SocketAddr, UdpSocket};
use std::str::from_utf8;
use std::sync::mpsc;


// custom type to hold the message and where the message comes from
#[derive(Debug)]
struct DuktoClientMessage {
    message: String,
    address: SocketAddr,
}

pub fn discover_clients() {
    let (sender, reciever) = mpsc::channel();
    let mut clients = std::collections::HashMap::new();

    std::thread::spawn(move || {
        let client = sender.clone();

        let udp_socket = UdpSocket::bind("0.0.0.0:4644")
            .expect("Failed to bind socket to port 4644");

        println!("Waiting for dukto clients to connect...");

        // FIX: Read on Error handling in threads
        loop {
            let mut buf: [u8; 1024] = [0; 1024];

            let (number_of_bytes_read, src_addr) = udp_socket
                .recv_from(&mut buf)
                .expect("Failed to get message from remote ip");

            let message = from_utf8(&buf[..number_of_bytes_read]).unwrap();

            let dc = DuktoClientMessage {
                message: message.to_string(),
                address: src_addr,
            };

            client.send(dc).unwrap();
        }
    });

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
        let _ = &clients.insert(
            dukto_client.message,
            dukto_client.address,
        );

        println!("Result: {:?}\n\n", clients);
    }
}
