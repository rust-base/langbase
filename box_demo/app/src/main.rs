use std::ops::Deref;

struct MyBox<T>(T);


impl<T> MyBox<T> {
    
    fn new(value:T) ->Self {
        MyBox(value)
    }
}


impl<T> Deref for MyBox<T>{
    type Target =T;
    fn deref(&self)->&Self::Target{
        println!("pre callback trigger");
        return &self.0;
    }
}


fn printName(s:&String){

    println!("{}", s);

}


fn main(){
   let b = MyBox::new(10000);

   println!("{}", *b);

   assert_eq!(*b,10000);

   let name= MyBox::new(String::from("温伟平"));

   printName(&(*name));//自然获得
   printName(&name); //func或method强制解引用多态
}
