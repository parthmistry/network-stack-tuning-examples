use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct SocketStat {
    success_count: u64,
    error_count: u64,
    error_messages: BTreeSet<String>
}

impl SocketStat {

    pub fn new() -> SocketStat {
        SocketStat {
            success_count: 0,
            error_count: 0,
            error_messages: BTreeSet::new()
        }
    }

    pub fn add_success_count(&mut self) -> () {
        self.success_count = self.success_count + 1;
    }

    pub fn add_error_count(&mut self, error_message: String) -> () {
        self.error_count = &self.error_count + 1;
        self.error_messages.insert(error_message);
    }

}

pub struct SocketStatLogger {
    pub join_handle: JoinHandle<()>
}

impl SocketStatLogger {

    pub fn new(socket_stat: Arc<Mutex<SocketStat>>) -> SocketStatLogger {
        let socket_stat_clone = Arc::clone(&socket_stat);
        let join_handle = thread::spawn(move || {
            let socket_stat = socket_stat_clone;
            loop {
                thread::sleep(Duration::from_secs(2));
                let mut socket_stat = socket_stat.lock().unwrap();
                println!("success: {}, error: {}", socket_stat.success_count, socket_stat.error_count);
                socket_stat.error_messages.iter().for_each(|error_message| println!("{}", error_message));
                socket_stat.success_count = 0;
                socket_stat.error_count = 0;
                socket_stat.error_messages.clear();
            }
        });
        SocketStatLogger {
            join_handle
        }
    }

}