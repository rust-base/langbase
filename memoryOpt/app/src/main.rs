

// use std::sync::{Arc,Mutex};
 

// fn getValue<'a>()->&'a i32{
//     let b:static 'a i32 = 32_i32;
//     return &b;
// }

#[derive(Debug)]
struct Student<'a>{
    name :&'a str,
    age :i32
}
impl<'a> Student<'a>  {
    fn new() ->Self{
        let name = "hello world";
        let age =100;
        Student{name,age}
    }
}


fn main() {

    // let a =vec![10,20,30];

    // for value in &a
    // {
    //     println!("v:{}",value);
    // }
  
    let s= Student::new();


    {
        println!("{:#?}",s );
    }



}
