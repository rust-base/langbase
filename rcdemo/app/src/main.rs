
use std::sync::Arc;
use std::thread;

fn printStr(s:&str){
   println!("{}",s );
}



fn main(){
  let s = "helloworld".to_string();

  let rc = Arc::new(s);

  thread::spawn(move || {
    printStr(&rc);
   // thread::us_sleep(1000);
  });

//   printStr(&rc);
}