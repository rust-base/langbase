
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::ErrorKind;


// extern crate encoding;
// use encoding::all::GB18030;
// use encoding::{DecoderTrap, EncoderTrap, Encoding};

fn createListener(ip:&str,port :i32)->TcpListener
{
    let addr = format!("{}:{}",ip.clone(),port);
    println!("addr:{}", addr);
    let listener = TcpListener::bind(addr).unwrap();
    listener
}
//传入前复制
fn writeRoutine(conn:Arc<Mutex<TcpStream>>){
    // let msg = "曾永红发红包了...[100元]大吉大利！！！".ToString();
    let arc_conn = conn.clone();
    let writeHandler = thread::spawn( move|| {
     let mut cc=    arc_conn.lock().unwrap();
        loop{
            thread::sleep_ms(1000);
            match cc.write(b"Mrs.Zeng send redPack [888] gongxifacai")
            {
                Ok(s)=>{

                },
                Err(e)=>{
                       println!("write err:{:#?}",e );
                      break;
                         
                }
            }
            // arc_conn.lock().unwrap().write( GB18030.encode(&msg, EncoderTrap::Strict).unwrap(););
        }
    });
    // writeHandler.join();
}

fn readRoutine(conn:Arc<Mutex<TcpStream>>){
   let mut buffer = [0u8,1024];

//    let connArc = conn.lock().unwrap();err

   let readHdl = thread::spawn(move || {
    loop{
        thread::sleep_ms(1000);
        conn.lock().unwrap().read(&mut buffer);
        // connArc.read(&mut buffer);err
         println!("{:?}",buffer);
    }
       });

    //    readHdl.join();
       
}


fn main() {
    let lis = createListener("127.0.0.1",9000);
    //accept
     for stream in lis.incoming(){
        match stream{
              Ok(mut streamConn)=>{
                println!("client in addr:{:#?}",streamConn.peer_addr());
                let arc_conn = Arc::new(Mutex::new(streamConn));
                writeRoutine(arc_conn.clone());
                // readRoutine(arc_conn.clone());
               
              }, 
              Err(e)=>{
                println!("{:?}", e);
              } 

        }
     }


   
}
