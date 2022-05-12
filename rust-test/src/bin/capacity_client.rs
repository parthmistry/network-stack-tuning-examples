use std::error::Error;
use std::net::{TcpStream, ToSocketAddrs};

use rust_test::socket_count_logger::SocketCountLogger;

fn main() -> Result<(), Box<dyn Error>> {
    let mut socket_count_logger = SocketCountLogger::new("connected");
    let server_addr = "pmload2.southeastasia.cloudapp.azure.com:9393".to_socket_addrs()?.next().unwrap();

    loop {
        match TcpStream::connect(server_addr) {
            Ok(tcp_stream) => socket_count_logger.add_socket(tcp_stream),
            Err(error) => {
                println!("{}", error.to_string());
                break;
            }
        }
    }

    socket_count_logger.join_handle.join().unwrap();
    Ok(())
}
