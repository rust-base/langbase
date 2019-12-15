//rust web server
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::io::Write;


fn main() {
    let listener:TcpListener = TcpListener::bind("127.0.0.1:9000").unwrap();

    for st in listener.incoming(){
        println!("client is comming ...");
        handle_connection(st.unwrap());
    }
    
}

fn handle_connection(mut stream:TcpStream)
{
   
    let handle = thread::spawn(move || {
        let buffer = b"hello world!!!!";
        loop {
            stream.write(buffer);
            thread::sleep_ms(1000);
        }
    });
     handle.join();
}
