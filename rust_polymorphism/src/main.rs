use std::ops::Deref;

struct Mybox<T>(T);

impl<T> Mybox<T>{
    fn new(v:T)->Self{
        Mybox(v)
    }
}

 impl<T> Deref for Mybox<T>{
     type Target= T;//接口内 模板参数实现
     fn deref(&self)->&T
     {
         &self.0  //引用为了保持传递
     }
 }

fn printMsg(msg:&str){
println!("msg:{}",msg );
}
 fn main(){
    let b = Mybox::new(String::from("add friend msg"));//box 越多 decision ly越多
    printMsg(&b);
 }