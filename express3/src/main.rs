

 

fn main() {
   
   {
     let mut res = if true {1} else{2};

     println!("res={} \n",res );

     res +=100;
     println!("res={} \n",res );
   }
  
    let a = (100,200);

    let (x,y) = a;

    println!("p.x={},p.y={}",x,y );

    let arr :[i32;5] =[1,2,3,4,5];
    
    for i in arr.iter(){
      println!("i={}",i);
    }

     let mut c= 0;
     loop 
     {
        if c>=5
         {
             break ;
         }
         println!("arr i:{}",arr[c]); 
    
         c+=1;
     }

    }
