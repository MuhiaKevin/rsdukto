use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std:: thread;
use anyhow::Result;

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 1024];
    let size = stream.read(&mut buf)?;
    // let name = format!("/tmp/{}", from_utf8(&buf[16..size - 9])?);
    let name = from_utf8(&buf[16..size - 9])?;
    println!("{:?}", &buf[16..size - 9]);
    println!("Filename: {}", name);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::create(&name) {
        Err(err) => panic!("Err: {} ", err),
        Ok(file) => file,
    };

    loop {
        match stream.read(&mut buf) {
            Ok(size) => {
                // println!("Read {:?}", size);
                // println!("What has been read {:?}", &buf[..size]);
                if size == 0 {
                    break;
                } 
                // stream.write_fmt(format_args!("Get Some DATAAAAA"))?;
                // stream.write(&buf[..size]).unwrap();
                file.write(&buf[..size])?;
                continue;
                
            },
            Err(_) =>{
                println!("An error occurred, terminating connection with {}", stream.peer_addr()?);
                stream.shutdown(std::net::Shutdown::Both)?;
            }
        }

    }

    Ok(())
}

pub fn download() -> Result<()> {
    // let listener = TcpListener::bind("127.0.0.1:4644")?; // start tcp server
    // let listener = TcpListener::bind("192.168.0.103:4644")?; // start tcp server
    let listener = TcpListener::bind("0.0.0.0:4644")?; // start tcp server

    // gets all incoming tcp connections
    for stream in listener.incoming(){
        // check if there is an error
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                // start a thread for each connection
                thread::spawn(move || {
                    handle_client(stream)
                });

            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }

    Ok(())
}
