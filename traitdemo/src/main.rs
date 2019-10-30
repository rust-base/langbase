
trait IRun{
   
   fn new()->Self;
   fn run(&self);
      
}

struct Person{
    id :i8
}

impl IRun for Person{
    
 fn new()->Self
 {
    Person{id:10}
 }
   fn run(&self)
   {
      println!("id:{}", self.id);
   }
      
}

struct Amimal{
    name:String,
    id:i8

}

impl IRun for Amimal{
   
   fn new()->Self
   {
       Amimal{
            id:32,
            name:String::from("dog")

       }
       
   }

   fn run(&self)
   {
      println!("id:{},name={}",self.id,self.name );

   }

}

fn doExec( runable:impl IRun){
     runable.run();
}


fn doExecuteByTemplate<T:IRun>(r:T)
{
    r.run();
}

fn doExecuteByTemplate2<T>(r:T) where T:IRun
{
    r.run();
}

fn main() {
     let r = Person::new();
    //  doExec(r);
doExecuteByTemplate2(r);
     let a = Amimal::new();
    //  doExec(a);

doExecuteByTemplate(a);
}
