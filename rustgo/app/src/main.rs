use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use std::io::Write;
use std::io::ErrorKind;


 

fn handlerAcceptConnection(mut stream:TcpStream){
    let msg = "helloworld protobuffer";
    loop{
       let res=  stream.write(msg.as_bytes());
        
       match res{
          Ok(num)=>{
            println!("has write :{}",num);
          },
          Err(e)=>{
              println!("{}, socket exit", e);
              break ;
            //  match e{
            //     Ok(errr)=>{
            //         println!("exit .....{}",errr);
            //         break;
            //     },
            //     Err(xx)=>{
            //         println!("{}", xx);
            //     }
                
            //  }
          
          }
       }

        std::thread::sleep_ms(1000);
    }

}

fn startListener(ip:&str,port:i32)
{
    let addr = format!("{0}:{1}",ip,port);
    println!("addr:{}",addr);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming(){
        match stream{
            Ok(mut expr)=>{
                    handlerAcceptConnection(expr);
            },
            Err(e)=>{
            }

        }

    }
}






fn main() {

    let ip ="127.0.0.1";
    let port = 9000;

    startListener(ip,port);
    

}
