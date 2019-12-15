use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(v:T)->Self{
        return MyBox(v);
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self)->&T  //解引用的操作，基础也是基于是&self指针
    {
        println!("{}","pre do defref" );
       return &self.0;
    }
}

fn printString(s:&str){
    println!("str:{}",s );
}


fn main() {
   let strBox = MyBox::new(String::from("温伟平"));
//    printString(&strBox);//目标参数要什么就写什么   //before ok
   
   

   std::thread::spawn(move ||{
        println!("{}", strBox.to_string());
   });

  // printString(&strBox);//目标参数要什么就写什么  //after error
}

