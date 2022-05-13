use std::error::Error;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};

use rust_test::socket_stat_logger::{SocketStat, SocketStatLogger};

fn main() -> Result<(), Box<dyn Error>> {
    let socket_stat = Arc::new(Mutex::new(SocketStat::new()));
    let socket_stat_logger = SocketStatLogger::new(Arc::clone(&socket_stat));
    let server_addr = "pmload2.southeastasia.cloudapp.azure.com:9393".to_socket_addrs()?.next().unwrap();

    for _ in 0..4 {
        let socket_stat_clone = Arc::clone(&socket_stat);
        std::thread::spawn(move || {
            let socket_stat = socket_stat_clone;
            loop {
                match TcpStream::connect(server_addr) {
                    Ok(tcp_stream) => {
                        socket_stat.lock().unwrap().add_success_count();
                        drop(tcp_stream);
                    },
                    Err(error) => {
                        let error_message = error.to_string();
                        socket_stat.lock().unwrap().add_error_count(error_message);
                    }
                }
            }
        });
    }

    socket_stat_logger.join_handle.join().unwrap();
    Ok(())
}
