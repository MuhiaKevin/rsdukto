use std::net::UdpSocket;
use std::str::from_utf8;
use std::sync::mpsc::Sender;
use crate::DuktoClientMessage;


// custom type to hold the message and where the message comes from
pub fn discover_clients(client: Sender<DuktoClientMessage>)  {
    std::thread::spawn(move || {
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
}
