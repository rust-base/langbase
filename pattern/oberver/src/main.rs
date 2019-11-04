
#[derive(Debug,Clone)]
struct Task{
    id:i32
}
trait IWorker{
   fn run(&self,t:Task);
}
struct Human{

}
impl IWorker for Human{
    fn run(&self,t:Task)
    {
        println!("Human eat task={}", t.id);
    }
} 

struct Ami{

}
impl IWorker for Ami{
    fn run(&self,t:Task)
    {
        println!("Ami eat task={}", t.id);
    }
} 



struct WorerMaster{
    workers:Vec<Box<IWorker>>
}

impl  WorerMaster {
    fn registerWorker(&mut self,worker:Box<IWorker>){
        self.workers.push(worker);
    }
    fn dispatchNewWorker(&self,task:Task){
        for w in &self.workers{
            w.run(task.clone());
        }
    }
}




fn main() {
     let mut mst = WorerMaster{workers:Vec::new()};

     let h = Human{};
     mst.registerWorker(Box::new(h));

     let a = Ami{};
     mst.registerWorker(Box::new(a));


     let t = Task{id:1000};

     mst.dispatchNewWorker(t);

  
}
