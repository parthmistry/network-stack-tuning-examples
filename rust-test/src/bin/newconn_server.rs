use std::error::Error;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::runtime::Builder;

use rust_test::socket_stat_logger::{SocketStat, SocketStatLogger};

fn main() -> Result<(), Box<dyn Error>> {
    let socket_stat = Arc::new(Mutex::new(SocketStat::new()));
    SocketStatLogger::new(Arc::clone(&socket_stat));

    let runtime = Builder::new_multi_thread().worker_threads(4).enable_time().build().unwrap();

    let listener = TcpListener::bind("0.0.0.0:9393")?;

    loop {
        match listener.accept() {
            Ok((tcp_stream, _)) => {
                socket_stat.lock().unwrap().add_success_count();
                runtime.spawn(async move {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    drop(tcp_stream);
                });
            },
            Err(error) => {
                let error_message = error.to_string();
                socket_stat.lock().unwrap().add_error_count(error_message);
            }
        }
    }
}
