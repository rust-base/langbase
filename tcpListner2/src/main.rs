use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn createListener(ip: &str, port: i32) -> TcpListener {
    let addr = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(addr).unwrap();
    listener
}

fn writeConroutine(safeBox: Arc<Mutex<TcpStream>>) {
    let msg = "hellworld";
    thread::spawn(move || {
        let mut cc = safeBox.lock().unwrap();
        loop {
            thread::sleep_ms(1000);
            match cc.write(msg.as_bytes()) {
                Ok(size) => {

                },
                Err(e) => {
                    break;
                }
            }
        }
    });
}

fn readConroutine(safeBox: Arc<Mutex<TcpStream>>) {}

fn main() {
    let listener = createListener("127.0.0.1", 9000);

    for streamReult in listener.incoming() {
        match streamReult {
            Ok(mut stream) => {
                let b = Arc::new(Mutex::new(stream));
                writeConroutine(b.clone());
                // readConroutine(b.clone());
            },
            Err(e) => {

            }
        }
    }
}

