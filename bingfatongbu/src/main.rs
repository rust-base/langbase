

use std::sync::Arc;
use std::sync::Mutex;
use std::time;
use std::thread;
use std::time::Duration;
fn main() {
   let  mylist = vec![1,2,3];
   let safeArc = Arc::new(Mutex::new(mylist));//new 一个所有权  ，一个作用域只 能new一个所有权
   let s2 =  safeArc.clone();

   let mut handles =vec![];
   for i in 0..10
   {
       let data = safeArc.clone();
       let h = thread::spawn(move || {
           let mut d = data.lock().unwrap();
           d[0]+=1;
       });
       handles.push(h);
   }

   for h in handles{
       h.join();
   }
   println!("value:{}", s2.lock().unwrap()[0] );
}
