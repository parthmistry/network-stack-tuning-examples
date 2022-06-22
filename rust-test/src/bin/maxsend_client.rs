use std::error::Error;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let server_host: &String = args.get(0).unwrap();
    let server_port: i32 = args.get(1).unwrap().parse().unwrap();
    let parallel_count: i32 = args.get(2).unwrap().parse().unwrap();

    let close_wait_duration: u64 = args.get(3).map_or(90, |arg| arg.parse().unwrap());

    let server_addr = format!("{}:{}", server_host, server_port).to_socket_addrs()?.next().unwrap();

    let connected_counts_lock = Arc::new(Mutex::new(0));

    let (client_sender, _) = tokio::sync::broadcast::channel(1);

    let client_handles: Vec<_> = (0..parallel_count).into_iter().map(|_| {
        // adding a delay because tokio TcpListener is not considering accept backlog from linux configuration
        // and too many parallel requests may drop connection requests on server side
        std::thread::sleep(Duration::from_micros(2));
        let mut client_receiver = client_sender.subscribe();
        let connected_counts_lock = Arc::clone(&connected_counts_lock);
        tokio::spawn(async move {
            let tcp_stream = TcpStream::connect(server_addr).await.unwrap();
            {
                let mut connected_counts = connected_counts_lock.lock().unwrap();
                *connected_counts = *connected_counts + 1;
                println!("connected sockets: {}", *connected_counts)
            }
            client_receiver.recv().await.unwrap();
            drop(tcp_stream);
        })
    }).collect();

    tokio::time::sleep(Duration::from_secs(close_wait_duration)).await;

    println!("start closing connections");
    client_sender.send("close client").unwrap();

    for client_handle in client_handles.into_iter() {
        client_handle.await.unwrap();
    }

    Ok(())
}
