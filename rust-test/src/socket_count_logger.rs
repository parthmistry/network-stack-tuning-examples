use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct SocketCountLogger {
    sockets: Arc<Mutex<Vec<TcpStream>>>,
    pub join_handle: JoinHandle<()>
}

impl SocketCountLogger {

    pub fn new(name: &'static str) -> SocketCountLogger {
        let sockets = Arc::new(Mutex::new(Vec::new()));
        let sockets_clone = Arc::clone(&sockets);
        let join_handle = thread::spawn(move || {
            let sockets = sockets_clone;
            loop {
                thread::sleep(Duration::from_secs(2));
                println!("{}Sockets: {}", name, sockets.lock().unwrap().len());
            }
        });
        SocketCountLogger {
            sockets,
            join_handle
        }
    }

    pub fn add_socket(&mut self, socket: TcpStream) -> () {
        self.sockets.lock().unwrap().push(socket);
    }

}