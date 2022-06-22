use std::io::Write;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use rust_test::bitrate_logger::{BitRateLogger, BitRateLoggerData};
use rust_test::byte_util::ByteUtil;
use rust_test::timeout_checker::TimeoutChecker;

fn main() -> () {
    let env_args: Vec<String> = std::env::args().skip(1).collect();
    let server_port: u16 = env_args.get(0).unwrap().parse().unwrap();

    let mut transferred_bytes: u64 = 0;

    let buf: Vec<u8> = Vec::from([0; 1050624]);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", server_port)).unwrap();
    let (mut tcp_stream, _) = listener.accept().unwrap();
    drop(listener);

    let start_time = Instant::now();
    let bit_rate_logger_data_lock = Arc::new(Mutex::new(BitRateLoggerData::new()));
    let bit_rate_logger = BitRateLogger::new(Arc::clone(&bit_rate_logger_data_lock));
    let timeout_checker = TimeoutChecker::new(Duration::from_secs(60));
    while !timeout_checker.is_timeout() {
        tcp_stream.write_all(&buf).unwrap();
        transferred_bytes += buf.len() as u64;
        let mut bit_rate_logger_data = bit_rate_logger_data_lock.lock().unwrap();
        bit_rate_logger_data.transferred_bytes += buf.len() as u64;
    }
    {
        let mut bit_rate_logger_data = bit_rate_logger_data_lock.lock().unwrap();
        bit_rate_logger_data.stopped = true;
    }
    let end_time = Instant::now();

    let transferred_mega_bits = ByteUtil::bytes_to_megabits(transferred_bytes);
    let duration_in_seconds = (end_time - start_time).as_millis() as f64 / 1000.0;

    println!("average bitrate: {} Mbps", ByteUtil::format_value(transferred_mega_bits / duration_in_seconds));

    drop(tcp_stream);
    bit_rate_logger.join_handle.join().unwrap();
}
