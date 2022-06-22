use std::io::Read;
use std::net::TcpStream;

fn main() -> () {
    let env_args: Vec<String> = std::env::args().skip(1).collect();
    let server_host: &String = env_args.get(0).unwrap();
    let server_port: u16 = env_args.get(1).unwrap().parse().unwrap();

    let mut buf: Vec<u8> = Vec::from([0; 1050624]);

    let mut tcp_stream = TcpStream::connect(format!("{}:{}", server_host, server_port)).unwrap();
    loop {
        let read_size = tcp_stream.read(&mut buf).unwrap();
        if read_size == 0 {
            break;
        }
    }
}
