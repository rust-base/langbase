
use std::sync::{Arc,Mutex};
use std::thread;
use std::io::ErrorKind;
use std::io::Write;
use std::time::Duration;

use std::net::{TcpListener,TcpStream};

static msg4 :&'static str= "static helloworld";
fn goWriteRoutine(safeBox:Arc<Mutex<TcpStream>>)  {
    // let msg = "helloworld".as_bytes();
    // let msg2 = String::from("hell world");
    const msg3: &str= "hell world";
    thread::spawn(move || {
        let mut stream = safeBox.lock().unwrap();
        loop{
            thread::sleep(Duration::from_millis(1000));
            // match stream.write(msg)
            // match stream.write(msg2.as_bytes())
            match stream.write(msg3.as_bytes())
            
            {
                Ok(s)=>{
                    
                },
                Err(e)=>{
                    match e.kind() {
                        ErrorKind::ConnectionAborted=>{
                            println!("client exit ip={:?},reason:ConnectionAborted",stream.peer_addr());
                            break;
                        },
                        _=> {

                        },
                    }

                }
            }
            
        }
    });

}

fn createListner(ip:&str,port :i32)->TcpListener{
    let addr = format!("{}:{}",ip,port);
    let listener = TcpListener::bind(addr).unwrap();
    return listener;
}

fn main() {
    
    let lis =createListner("127.0.0.1",9000);
   
    for streamResult in lis.incoming(){
        match streamResult {
            Ok(mut stream) => {
                let safeBox = Arc::new(Mutex::new(stream));
                goWriteRoutine(safeBox);

            },
            Err(e) => {
                
            },
        }

    }

}
