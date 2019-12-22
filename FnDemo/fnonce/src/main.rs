use std::ops::FnOnce;

fn apply(f:impl FnOnce()->()) //FnOnce 只能执行一次
{
  f();
}


fn main() {
   let  num =100;
   let f =||{
       println!("{}",num);
       drop(num);
   };
   apply(f);
   
}
