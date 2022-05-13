use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use crossbeam::channel::Receiver;

use rust_test::socket_stat_logger::{SocketStat, SocketStatLogger};

fn main() -> Result<(), Box<dyn Error>> {
    let socket_stat = Arc::new(Mutex::new(SocketStat::new()));
    SocketStatLogger::new(Arc::clone(&socket_stat));

    let (sender, receiver) = crossbeam::channel::unbounded();

    let listener = TcpListener::bind("0.0.0.0:9393")?;

    for _ in 0..4 {
        let receiver: Receiver<TcpStream> = receiver.clone();
        std::thread::spawn(move || {
            loop {
                if let Ok(mut tcp_stream) = receiver.recv() {
                    let mut buffer: [u8; 1] = [0; 1];
                    tcp_stream.read(&mut buffer).unwrap();
                }
            }
        });
    }

    loop {
        match listener.accept() {
            Ok((tcp_stream, _)) => {
                socket_stat.lock().unwrap().add_success_count();
                sender.send(tcp_stream)?;
            },
            Err(error) => {
                let error_message = error.to_string();
                socket_stat.lock().unwrap().add_error_count(error_message);
            }
        }
    }
}
