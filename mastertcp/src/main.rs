
use std::net::{TcpListener,TcpStream};
use std::time::Duration;
use std::io::{Write,Read};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::ErrorKind;
use std::io;
 


fn createListener(ip:&str,port:i32)->TcpListener{
   let addr = format!("{}:{}",ip,port);
   let listener = TcpListener::bind(addr).unwrap();
   listener
}

fn doWriteRoutine(conn:Arc<Mutex<TcpStream>>){
     let msg = String::from("hell world");
     thread::spawn(move||{
         let bt = msg.as_bytes();
          loop{
            thread::sleep(Duration::from_millis(1000));
            match  conn.lock().unwrap().write(bt){
                Ok(size)=>{
                //   println!("write size:{}",size );
                },
                Err(e)=>{
                    match e.kind(){
                      ErrorKind::ConnectionAborted=>{
                          println!("client exit one err:{:#?}",e);
                      },
                      _=>{
                          println!("err:{:#?}",e )
                      }
                    }
                    break;
                }
            }
          }
     });
}

fn doReadRoutine(conn:Arc<Mutex<TcpStream>>){
  
}

fn main() {
    let mut conn = 0;
    let arcConn = Arc::new(Mutex::new(conn));
    for i in 0..100{

        let thread_Conn = arcConn.clone();
        let listener = createListener("192.168.0.103",9000+i);
        thread::spawn(move || {
            println!("开启端口:{}", 9000+i);
            for stream in listener.incoming(){
                match stream{
                    Ok(mut st)=>{
                       let mut y=  thread_Conn.lock().unwrap();
                       *y+=1;
                        println!("客户count:{}",*y);
                        let safeArcMutBox = Arc::new(Mutex::new(st));
                        doWriteRoutine(safeArcMutBox.clone());
                        doReadRoutine(safeArcMutBox.clone());
                    },
                    Err(e)=>{
        
                    }
                }
             }


        });




    }


     let mut buf = String::new();
    io::stdin().read_line(&mut buf);  
}
