

trait BaseObject {
      fn attach(&self);
}

trait Command{
    fn execute(&self);
}
struct GameObject{
  
}
impl  GameObject{
 fn new()->Self{
    GameObject{}
 }
}

impl BaseObject for GameObject {
     fn attach(&self){
         println!("attach....");
     }
}

impl Command for GameObject{
    fn execute(&self)
    {
        println!("Command:exec");
    }
}

fn callBaseObject<T:BaseObject>(base:&T)
{
    base.attach();
}

fn callCmd<T:Command>(cmd:&T){
    cmd.execute();
}


fn main() {
    
   let o = GameObject::new();
   callCmd(&o);
   callBaseObject(&o);

}
