

trait IGo{
    fn gogo(&self);
}


trait IRun:IGo{
   fn new()->Self;
   fn run(&self);
   fn gogo(&self);
}

struct Person(i32);


impl IRun for Person{
   fn new()->Self
   {
       Person(23)
   }
   fn run(&self)
   {
       println!("value={}",self.0);
   }
   fn gogo(&self){

   }
}

struct Amimal(i32);

impl IRun for Amimal{
      fn new()->Self{
           Amimal(20)
      }

      fn run(&self)
      {
          println!("value={}",self.0);
      }

      fn gogo(&self){
       
      }
}


fn Context_exec(r:& impl IRun)
{
    r.run();
    // r.gogo()
}


fn main() {
  
    let a = Amimal::new();
    Context_exec(&a);

    let p = Person::new();
    Context_exec(&p);


    

    println!("Hello, world!");
}
