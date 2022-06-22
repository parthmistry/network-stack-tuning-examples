use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use crate::byte_util::ByteUtil;

pub struct BitRateLoggerData {
    pub transferred_bytes: u64,
    pub stopped: bool
}

impl BitRateLoggerData {

    pub fn new() -> BitRateLoggerData {
        BitRateLoggerData {
            transferred_bytes: 0,
            stopped: false
        }
    }

}

pub struct BitRateLogger {
    pub join_handle: JoinHandle<()>
}

impl BitRateLogger {

    pub fn new(bitrate_logger_data_lock: Arc<Mutex<BitRateLoggerData>>) -> BitRateLogger {
        let join_handle = std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(1));
                let mut bitrate_logger_data = bitrate_logger_data_lock.lock().unwrap();
                if bitrate_logger_data.stopped {
                    break;
                }
                let transferred_bytes_per_second = bitrate_logger_data.transferred_bytes;
                let mega_bits_per_second = ByteUtil::bytes_to_megabits(transferred_bytes_per_second);
                println!("bitrate: {:.3} Mbps", mega_bits_per_second);
                bitrate_logger_data.transferred_bytes = 0;
            }
        });
        BitRateLogger {
            join_handle
        }
    }

}