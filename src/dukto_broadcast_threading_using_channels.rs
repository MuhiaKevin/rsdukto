use std::net::{SocketAddr, UdpSocket};
use std::str::from_utf8;
use std::sync::mpsc;


// custom type to hold the message and where the message comes from
#[derive(Debug)]
struct DuktoClientMessage {
    message: String,
    address: SocketAddr,
}

pub fn main() {
    let (sender, reciever) = mpsc::channel();
    let mut clients = std::collections::HashMap::new();
    // let mut handles = vec![];

    let handle = std::thread::spawn(move || {
        let client = sender.clone();

        let udp_socket =
            UdpSocket::bind("0.0.0.0:4644").expect("Failed to bind socket to port 4644");

        // FIX: Read on Error handling in threads
        loop {
            let mut buf: [u8; 1024] = [0; 1024];

            let (number_of_bytes_read, src_addr) = udp_socket
                .recv_from(&mut buf)
                .expect("Failed to get message from remote ip");

            let message = from_utf8(&buf[..number_of_bytes_read]).unwrap();
            
            // println!("The data in bytes: {:?}", &buf[..number_of_bytes_read]);
            // println!("Number of bytes read to buffer: {}", number_of_bytes_read);
            // println!("Address of the Dukto client: {}", src_addr);
            // println!("Messaage is: {}\n\n\n", message);

            let dc = DuktoClientMessage {
                message: message.to_string(),
                address: src_addr,
            };

            client.send(dc).unwrap();
            // println!("Finished sending to channel");
        }
    });


    // handle.join().unwrap();

    while let Ok(dukto_client)  = reciever.recv() {
        &clients.insert(
            dukto_client.message,
            dukto_client.address,
        );

        println!("Result: {:?}\n\n", clients);
    }
}
