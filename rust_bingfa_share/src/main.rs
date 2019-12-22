


fn main(){

    use std::sync::{Arc,Mutex};
     //share list

     let myList =vec![0,1];
     //safe所有权管理，具有安全的所有权访问
     let myListSafeBox = Arc::new(Mutex::new(myList));
     let mainThreadBox = myListSafeBox.clone();
     use std::thread;
     //并发线程
     for _i in 0..100{
         let b = myListSafeBox.clone();
         thread::spawn(move || {
             b.lock().unwrap()[0]+=1;
         });

     }
     thread::sleep_ms(1000);
    println!("value:{}",mainThreadBox.lock().unwrap()[0] );
}
