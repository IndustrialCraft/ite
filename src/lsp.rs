use std::io::{Read, Write};
use std::process::{Child, ChildStdout, Stdio};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use serde::Serialize;
use serde_json::json;

pub struct LSP{
    process: Child,
    msg_id: u32
}
impl LSP{
    pub fn new() -> Self{
        let mut process = std::process::Command::new("rust-analyzer").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
        Self::create_listener_thread(process.stdout.take().unwrap());
        LSP{
            process,
            msg_id: 0
        }
    }
    pub fn send<T: Serialize>(&mut self, method: &str, message: T) {
        let message = format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"method\":\"{}\",\"params\":{}}}", self.msg_id, method, serde_json::to_string(&message).unwrap());
        let stdin = self.process.stdin.as_mut().unwrap();
        stdin.write_all(format!("Content-Length: {}\r\n\r\n{}", message.len(), message).as_str().as_ref()).unwrap();
        stdin.flush().unwrap();
    }
    pub fn create_listener_thread(mut stdout: ChildStdout)/* -> Receiver<String>*/{
        std::thread::spawn(move ||{
            let length: u32 = {
                let mut stdout = (&mut stdout).bytes();
                stdout.nth("Content-Length: ".len() - 1);
                let mut length = String::new();
                for ch in stdout {
                    let ch = ch.unwrap();
                    if ch == b'\r' || ch == b'\n' {
                        break;
                    }
                    length.push(ch as char);
                }
                length.parse().unwrap()
            };
            let mut buf = vec![0u8; length as usize + 3];
            stdout.read_exact(&mut buf).unwrap();

            println!("output len = {}, json = {}", length, String::from_utf8_lossy(&buf[3..]).as_ref().to_string());
        });
    }
}