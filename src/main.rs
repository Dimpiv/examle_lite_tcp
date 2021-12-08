use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

const ALLOW_HOST: &str = "0.0.0.0";
const PORT: usize = 8888;

fn main() {
    let address_port = format!("{}:{}", ALLOW_HOST, PORT);
    run_server(&address_port);
}

fn handle_client(mut stream: &TcpStream) {
    let mut buffer = [0 as u8; 16];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size <= 0 {
                    break;
                }
                read_message(&buffer, &size);
                // flush buffer
                buffer = [0 as u8; 16];
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

fn read_message(raw_data: &[u8; 16], size: &usize) {
    println!("Recived bytes: {}", size);
    println!("Raw bytes: {:?}", raw_data);

    let text = from_utf8(raw_data).unwrap();
    println!("Text: {}", text);
}

fn run_server(address_port: &str) {
    let listener = TcpListener::bind(address_port).unwrap();
    println!("Server listening on port {}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(&stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
