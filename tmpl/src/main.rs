
 
struct Point<T>
{
    x:T,
    y:T
}
#[derive(Debug)]
struct Point2<A,B>
{
    x:A,
    y:B
}

impl<T> Point<T>{
  
    fn getX(&self)->&T{
       & self.x
    }   
    fn getY(&self)->&T{
       & self.y
    }

}




fn max<T:Copy+PartialOrd>(list:&[T])->T
{
    let mut maxValue =list[0];
    for &item in list.iter(){
        if maxValue<item{
            maxValue = item;
        }
    }
    maxValue
}




fn main() {
   let p =  Point{x:12,y:32};
   
   println!("x:{},y:{}",p.getX(),p.getY());

   let arr = [1,2,3];
   let m =max(&arr);

   let vec1 = vec![10,20,1,0];

   let m2 = max(&vec1);

   println!("max:{},",m2);
}
