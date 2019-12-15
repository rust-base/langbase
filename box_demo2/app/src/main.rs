

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value:T) ->Self{
        MyBox(value)
    }
}

use std::ops::Deref;

impl <T> Deref for MyBox<T>{
      type Target= T;
      fn deref(&self)-> &Self::Target{
         &self.0
      }
}

fn printMyName(s:&str){
 
    println!("{}",s );

}

fn main() {
    //  let num = MyBox::new(1000);
    //  assert_eq!(*num,100);
    let myName  = MyBox::new(String::from("wenweiping"));
    printMyName(&(*myName));
    printMyName(&myName);
    }
