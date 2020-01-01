use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

use std::collections::HashMap;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use std::time;
use std::io::Write;

use std::io::ErrorKind;


struct ClientConnection{
       stream:TcpStream,
}

impl ClientConnection{
    fn new(safeBox:TcpStream)->Self{
        return  ClientConnection{stream:safeBox};
    }
    fn run(&self){
        let safeBox = Arc::new(Mutex::new(self.stream));
        self.doLoopRead(safeBox);
        self.doLoopWrite(safeBox);
    }

    fn doLoopRead(&self,safeBox:Arc<Mutex<TcpStream>>){
        let h = thread::spawn(move || {
            let readerExecuter = safeBox.lock().unwrap();

        });
        h.join().unwrap();
    }

    fn doLoopWrite(&self,safeBox:Arc<Mutex<TcpStream>>){
        let msg = "hello world write".as_bytes();
        let h =  thread::spawn(move || {
            let mut writerExecuter = safeBox.lock().unwrap();
            loop{
                thread::sleep_ms(1000);
                match writerExecuter.write(msg)
                {
                    Ok(s)=>
                    {
    
                    },
                    Err(e)=>{
                        match e.kind(){
                            ErrorKind::ConnectionAborted=>{
                                println!("client exit ip:{:#?}",writerExecuter.peer_addr());
                                break;
                            },
                            _=>{
    
                            }
    
                        }
                    }
                }



            }

  
        });
         h.join().unwrap();
}
}
 

struct ClientCtx<'a>{
    clients:HashMap<i32,&'a ClientConnection>
 
}
static  ctx:ClientCtx = ClientCtx{clients:HashMap::new()};
 

static UUID :Arc<Mutex<u32>>= Arc::new(Mutex::new(0));

fn genUUId()->i32{
    let mut id = UUID.lock().unwrap();
    id+=1;
    id
}

impl<'a> ClientCtx<'a>{
    fn handleClient(tcpStream:TcpStream){
        let id = genUUId();
        let conn = ClientConnection::new(tcpStream);
        ctx.clients.insert(id,&'a conn);
        conn.run();
    }

}




trait IAcceptor<'a>{
      fn start(&self,ip:&'a str,port:i32);
      fn wait(&self);
}


struct ListenerAcceptor<'a>{
    listener:TcpListener,
    ip:&'a str,
    port :i32,
}

impl<'a> IAcceptor for ListenerAcceptor<'a>{

      fn start(&self,ip:&'a str,port:i32)
    {
        self.ip = ip;
        self.port = port;
        let addr = format!("{}:{}",self.ip,self.port);
        self.listener = TcpListener::bind(addr).unwrap();
        
        thread::spawn(move || {
            for streamBox in self.listener.incoming(){
                match streamBox {
                    Ok(stream) => {
                        ClientCtx::handleClient(stream);
                    },
                    Err(e) => {


                    },
                }

            }
        });


    }
      fn wait(&self)
    {


    }

}


fn main() {

    let mut v = Vec::new();
    let shareValueBox = Arc::new(Mutex::new(0));
    for i in 0..100
    {
        // println!("{}",i );
        // let v = &i;
        let data  =shareValueBox.clone();
        v.push(thread::spawn(move || {
            let mut d =data.lock().unwrap();
            *d+=1;
            println!("thread id :{}", *d);
            println!("thread id xxxx :{}", d);



        }));
    }

    for handler in v
    {
        handler.join().unwrap();
    }
}
