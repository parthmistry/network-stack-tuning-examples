use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use rust_test::stats_util::StatsUtil;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let server_port: i32 = args.get(0).unwrap().parse().unwrap();
    let connection_wait_duration: u64 = args.get(1).map_or(15, |arg| arg.parse().unwrap());
    let write_wait_duration: u64 = args.get(2).map_or(15, |arg| arg.parse().unwrap());
    let close_wait_duration: u64 = args.get(3).map_or(30, |arg| arg.parse().unwrap());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", server_port)).await?;
    println!("server started");

    let (listener_sender, listener_receiver) = tokio::sync::oneshot::channel();

    let buf: Arc<Vec<u8>> = Arc::new(Vec::from([0; 64000]));

    let listener_handle = tokio::spawn(async move {
        let mut accepted_tcp_streams: Vec<TcpStream> = Vec::new();

        tokio::pin!(listener_receiver);

        loop {
            tokio::select! {
                _ = &mut listener_receiver => {
                    break;
                },
                result = listener.accept() => {
                    accepted_tcp_streams.push(result.unwrap().0);
                }
            };
        };

        accepted_tcp_streams
    });

    tokio::time::sleep(Duration::from_secs(connection_wait_duration)).await;
    listener_sender.send("stop accept loop").unwrap();

    let accepted_tcp_streams = listener_handle.await.unwrap();

    let (writer_sender, _) = tokio::sync::broadcast::channel(1);

    println!("start writing to connections");

    let writer_handles: Vec<_> = accepted_tcp_streams.into_iter().map(|mut tcp_stream| {
        let mut writer_receiver = writer_sender.subscribe();
        let buf = Arc::clone(&buf);
        tokio::spawn(async move {
            let mut sent_bytes: u64 = 0;
            loop {
                tokio::select! {
                    _ = writer_receiver.recv() => {
                        break;
                    },
                    _ = tcp_stream.write_all(&buf[..]) => {
                        sent_bytes += buf.len() as u64;
                    }
                }
            }
            (sent_bytes, tcp_stream)
        })
    }).collect();

    tokio::time::sleep(Duration::from_secs(write_wait_duration)).await;
    writer_sender.send("stop write operation").unwrap();

    let mut sent_bytes: Vec<u64> = Vec::new();
    let mut connected_tcp_streams: Vec<TcpStream> = Vec::new();

    for write_handle in writer_handles.into_iter() {
        let (write_handle_sent_bytes, tcp_stream) = write_handle.await.unwrap();
        sent_bytes.push(write_handle_sent_bytes);
        connected_tcp_streams.push(tcp_stream);
    }

    sent_bytes.sort();

    StatsUtil::print_stats(&sent_bytes);

    tokio::time::sleep(Duration::from_secs(close_wait_duration)).await;

    println!("start closing connections");
    for connected_tcp_stream in connected_tcp_streams.into_iter() {
        drop(connected_tcp_stream);
    }

    Ok(())
}
