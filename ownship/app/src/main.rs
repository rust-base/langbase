
fn callStr(m:String){//ownship move here
    println!("{}",m.as_str() );
}

fn main() {
    let mut s = String::from("helloworld");
    {
           let m = &mut s;
           m.push_str("string1111");
           println!("{}",m.as_str());
    }

    // rust和go都有对象和引用指针，都可以访问属性和方法 （访问方式一样，采用了 强制解引用多态）
    let s2 = &mut s;//s2是指针，但是它不可以 指针地址不可以改变，它的目标可以改
    s2 = &mut String::from("haha"); //so twice change ,error here
    println!("{}",s2.as_str());
    
}
