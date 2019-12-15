

fn demoFor(){

    let mut range = 0..10;

    loop{
       match range.next()
       {
           Some(x)=>{
                println!("{}",x);
           },
           None=>{
                break;
           }

       }

    }

}

fn doWhile(){
    let mut num:i32 =100;
    while num>0{
        num -=1;
        println!("{}",num);
    }
}

fn doLoop(){
    loop{


    }
}


fn main() {
    doLoop();
}
