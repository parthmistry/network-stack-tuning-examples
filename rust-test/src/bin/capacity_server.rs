use std::error::Error;
use std::net::TcpListener;

use rust_test::socket_count_logger::SocketCountLogger;

fn main() -> Result<(), Box<dyn Error>> {
    let mut socket_count_logger = SocketCountLogger::new("accepted");
    let listener = TcpListener::bind("0.0.0.0:9393")?;

    loop {
        match listener.accept() {
            Ok((tcp_stream, _)) => socket_count_logger.add_socket(tcp_stream),
            Err(error) => {
                println!("{}", error.to_string());
                break;
            }
        }
    }

    drop(listener);
    socket_count_logger.join_handle.join().unwrap();
    Ok(())
}
