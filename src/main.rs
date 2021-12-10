use chrono::{DateTime, Utc};
use std::io::Read;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::thread;
// use std::time::Duration;

const ALLOW_HOST: &str = "0.0.0.0";
const PORT: usize = 8888;

struct Message {
    recived_time: DateTime<Utc>,
    connection_from: SocketAddr,
    body: Vec<u8>,
}

impl Message {
    fn new(connection_from: SocketAddr) -> Message {
        Message {
            recived_time: Utc::now(), // Время открытия соединения
            body: vec![0],
            connection_from,
        }
    }

    fn print_bytes(&self) {
        println!("Raw bytes array:\n{:?}", &self.body)
    }

    fn print_info(&self) {
        println!(
            "start connect: {:?} \nconnect from: {} \nsize: {}",
            self.recived_time,
            self.connection_from,
            self.body.len()
        )
    }
}

fn main() {
    let address_port = format!("{}:{}", ALLOW_HOST, PORT);
    run_server(&address_port);
}

fn run_server(address_port: &str) {
    let listener = TcpListener::bind(address_port).unwrap();
    println!("Server listening on port {}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Example timeout: no reciveding bytes within 5 seconds -> close stream
                // let stop_stream = stream.set_read_timeout(Some(Duration::new(5, 0))).unwrap();

                let mut buff_stream = Message::new(stream.peer_addr().unwrap());
                thread::spawn(move || handle_stream(&stream, &mut buff_stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: &TcpStream, buffer: &mut Message) {
    loop {
        match stream.read_to_end(&mut buffer.body) {
            Ok(size) => {
                if size <= 0 {
                    break;
                }
                buffer.print_info();
                buffer.print_bytes();
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
