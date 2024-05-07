use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use std::str::from_utf8;
use std::sync::mpsc::Sender;
use crate::DuktoClientMessage;



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


            if let SocketAddr::V4(addr) = src_addr {
                let dc = DuktoClientMessage {
                    device_name: message.to_string(),
                    address: addr.ip().to_string(),
                };

                client.send(dc).unwrap();
            }
        }
    });


    std::thread::spawn(|| {
        get_discovered_by_clients();

    });
}


fn send_discover_message() -> Vec<u8>{
    let mut header: Vec<u8> =  vec![1];
    let message = b"Chifu at Kizunu (Rustlang)";

    header.extend_from_slice(&message[..]);
    header
}


fn get_discovered_by_clients() {
    let msg = send_discover_message();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    socket.set_broadcast(true).expect("set_broadcast call failed");

    // let broadcast_ip = Ipv4Addr::new(255, 255, 255, 255); 
    // let broadcast_port = 4644; 
    // let broadcast_socket_addr = SocketAddrV4::new(broadcast_ip, broadcast_port);


    loop {
        println!("Sending broadcast message");
        // socket.send_to(&msg, broadcast_socket_addr).expect("couldn't send data");
        socket.send_to(&msg, "255.255.255.255:4644").expect("couldn't send data");
        std::thread::sleep(Duration::from_secs(10))
    }
}
