
#[derive(Debug)]
enum Opt {
    Delete=0,
    Update=1,
    Insert=2,
    Add =3,
}


trait  BaseCmd<T>
{
    fn new(o:Opt)->Target
    {
        return T(o);
    }
    fn execute(&self);
}

struct Delete(Opt);

impl<T> BaseCmd<T> for Delete {
     T= Delete
    fn execute(&self)
    {
        if self.0==Opt::Delete{
            println!("haha my execute is delete");
        }
        
    }

    fn new(o:Opt)->Target
    {
        return T(o);
    }
}



fn main() {
   let name:&str = "wenweiping";

   println!("res:{}",name);

   let a  =name as *const str;
   let b = a as * const u8;
   println!("{:?}",b);

   let mut myBooks = String::new();
   myBooks.push_str("english\n");
   myBooks.push_str("French\n");
   myBooks.push_str("math");
  
   println!("{}",myBooks);


   let mm = 200;

   let mmC_Style = &mm as *const i32 as usize;

   let targetPtr = mmC_Style as *mut i32;

   unsafe{
       *targetPtr =51000;
   }
   let targetPtr3 = mmC_Style as *mut i32;

   unsafe{
       *targetPtr3 =6000;
   }


   let targetPtr4 = mmC_Style as *mut i32;

   unsafe{
       *targetPtr4 =8000;
   }

   println!("changValue:{}", mm);


   
   let cmd =Delete::new();
   cmd.execute();

}
