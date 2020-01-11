
trait AbstractCmd{

    fn execute(&self)
    {
        println!("default execute!!");
    }

}
trait AbstractCmd2{

    fn execute(&self)
    {
        println!("22222222222default execute!!");
    }

}
struct DeleteCmd(i32);

impl AbstractCmd for DeleteCmd {
    
    fn execute(&self){ //error 重写了 override
            println!("DeleteCmd execute");
    }

}

// impl AbstractCmd2 for DeleteCmd { //error 多个trait有相同的方法 报错
    
//     // fn execute(&self){
//     //         println!("DeleteCmd execute");
//     // }

// }


fn main() {
     let cmd = DeleteCmd(0);
     cmd.execute();

     AbstractCmd::execute(&cmd);
    //  AbstractCmd2::execute(&cmd);
}
