

use std::sync::{Arc,Mutex};
 

fn getValue<'a>()->&'a i32{
    let b:static 'a i32 = 32_i32;
    return &b;
}


fn main() {

    println!("value:{}", getValue());
}
