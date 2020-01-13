















fn main() {
    let mut msg = String::from("art\n");
    msg.push_str("math");

    msg.extend(&['1','2','3','\n']);

    println!("msg:{}",msg);


    let msg_clone = msg.clone();
    println!("msg_clone:{}",msg_clone);

    let haha = msg_clone+" \n addtion string";

    println!("haha:{}",&haha);
    
//String implements std::ops::Add,因此相加一定要有一个String 拥有主权的的String ，否则报错
    // let haha2 =/****String::new()+****/&haha+&String::from(" hellworld ")+" \n addtion222222 string";
    let haha2 =String::new()+&haha+&String::from(" hellworld ")+" \n addtion222222 string";
    println!("haha2:{}",&haha2);

    let fmt_link_str = format!("{}:{}",&haha2,&haha);
    println!("fmt_link_str:{}",fmt_link_str);
     
    let num =100;

    let res = num.to_string().parse::<i32>();

    match res {
        Ok(v) => {
            println!("res:{}",v);
        },
        Err(e) => {

        },
    }

    let f = 65.232_f64;

    let fstr = f.to_string().parse::<f64>();

    if let Ok(v) = fstr{
        println!("vvvvfloat64:{}",v);
    }


    let arrRef :&[i32] = &[1,2,3];

    let vvec:Vec<i32> = arrRef.to_vec();

    let b = Box::new(arrRef);

    println!("v:{:?}",&&&&&b);//*b  or b  is ok


 


    
}
