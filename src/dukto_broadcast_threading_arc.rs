use std::net::{SocketAddr, UdpSocket};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;


// custom type to hold the message and where the message comes from
#[derive(Debug)]
struct DuktoClientMessage {
    message: String,
    address: SocketAddr,
}

fn main() {
    // let devices: ArcDuktoClientMessage = Arc::new(Mutex::new(vec![]));
    // let client = Arc::clone(&devices);
    let (sender, reciever) = mpsc::channel();
    

    // let mut handles = vec![];

    let handle = std::thread::spawn(move || {
        let udp_socket =
            UdpSocket::bind("0.0.0.0:4644").expect("Failed to bind socket to port 4644");

        // FIX: Read on Error handling in threads
        loop {
            let mut buf: [u8; 1024] = [0; 1024];

            let (number_of_bytes_read, src_addr) = udp_socket
                .recv_from(&mut buf)
                .expect("Failed to get message from remote ip");

            let message = from_utf8(&buf[..number_of_bytes_read]).unwrap();

            println!("The data in bytes: {:?}", &buf[..number_of_bytes_read]);
            println!("Number of bytes read to buffer: {}", number_of_bytes_read);
            println!("Address of the Dukto client: {}", src_addr);
            println!("Messaage is: {}\n\n\n", message);

            let dc = DuktoClientMessage {
                message: message.to_string(),
                address: src_addr,
            };

            let mut num = client.lock().unwrap();

            num.push(dc);
        }
    });

    // let reading = Arc::clone(&devices);
    //
    // let handle2 = std::thread::spawn(move || {
    //     loop {
    //         println!("Result: {:?}", *reading.lock().unwrap());
    //     }
    // });
    //
    // handles.push(handle);
    // handles.push(handle2);
    //
    // for hd in handles {
    //     hd.join().unwrap();
    // }

    handle.join().unwrap();

    println!("Result: {:?}", *devices.lock().unwrap());
}
