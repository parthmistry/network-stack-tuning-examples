use std::fmt::{Display, Formatter};
use std::process::Command;
use regex::Regex;
use rust_test::byte_util::ByteUtil;
use rust_test::stats_util::StatsUtil;

struct SocketMemory {
    remote_port: u64,
    rmem_alloc: u64,
    rcv_buf: u64,
    wmem_alloc: u64,
    snd_buf: u64,
    wmem_queued: u64
}

impl Display for SocketMemory {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "remote_port={},rmem_alloc={},rcv_buf={},wmem_alloc={},snd_buf={},wmem_queued={}",
               self.remote_port,
               ByteUtil::transform_bytes(self.rmem_alloc),
               ByteUtil::transform_bytes(self.rcv_buf),
               ByteUtil::transform_bytes(self.wmem_alloc),
               ByteUtil::transform_bytes(self.snd_buf),
               ByteUtil::transform_bytes(self.wmem_queued)
        )
    }

}

fn main() {
    let env_args: Vec<String> = std::env::args().skip(1).collect();
    let server_port: u16 = env_args.get(0).unwrap().parse().unwrap();

    let process_output = Command::new("sh").arg("-c").arg("ss -tm").output().unwrap();
    let output_string = String::from_utf8(process_output.stdout).unwrap();
    let mut output_line_iterator = output_string.lines().into_iter();

    let socket_line_pattern: String = format!(".*:{}.*:([\\d]+)", server_port);
    let socket_line_regex = Regex::new(&socket_line_pattern).unwrap();

    let stats_line_regex = Regex::new("r([\\d]+),rb([\\d]+),t([\\d]+),tb([\\d]+),f([\\d]+),w([\\d]+),o([\\d]+),bl([\\d]+),d([\\d]+)").unwrap();

    let mut socket_memories: Vec<SocketMemory> = Vec::new();

    while let Some(line) = output_line_iterator.next() {
        if let Some(socket_line_captures) = socket_line_regex.captures(&line) {
            let remote_port = socket_line_captures.get(1).unwrap().as_str();
            if let Some(stats_line) = output_line_iterator.next() {
                if let Some(stats_line_captures) = stats_line_regex.captures(&stats_line) {
                    socket_memories.push(SocketMemory {
                        remote_port: remote_port.parse().unwrap(),
                        rmem_alloc: stats_line_captures.get(1).unwrap().as_str().parse().unwrap(),
                        rcv_buf: stats_line_captures.get(2).unwrap().as_str().parse().unwrap(),
                        wmem_alloc: stats_line_captures.get(3).unwrap().as_str().parse().unwrap(),
                        snd_buf: stats_line_captures.get(4).unwrap().as_str().parse().unwrap(),
                        wmem_queued: stats_line_captures.get(6).unwrap().as_str().parse().unwrap()
                    });
                }
            } else {
                panic!("unexpected error occurred");
            }
        }
    }

    socket_memories.sort_by(|s1, s2| s1.wmem_queued.cmp(&s2.wmem_queued));
    for (i, socket_memory) in socket_memories.iter().enumerate() {
        println!("{:7} : {}", (i + 1), socket_memory);
    }

    let wmem_queued_list: Vec<u64> = socket_memories.iter().map(|socket_memory| socket_memory.wmem_queued).collect();

    StatsUtil::print_stats(&wmem_queued_list);
}