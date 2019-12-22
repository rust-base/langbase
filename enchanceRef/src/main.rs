
fn mutableInSubScope(){
let mut a= 100;
{
    //a mutable borrwo in sub scope
    let y = &mut a;//an mutable borrow
    *y+=1;
}

{
    //a mutable borrwo in sub scope
    let y = &mut a;//an mutable borrow
    *y+=1;
}

  println!("res:{}", a);//a immutable borrow
}

fn mutableInSameScope(){
  let mut a= 100;
  let y = &mut a ;
  *y=*y+100;
  println!("y:{}",y);
}

fn main(){
    mutableInSubScope();
}