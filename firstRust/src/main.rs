
use std::collections::HashMap;

fn new_look(trou: String) ->String {
   println!("cool trou:{}",trou);
   trou
}
//凭证
fn passByRef(trou:&String){
   println!("passByRef trou:{}",trou);
}

fn changeValueByRef(str :&mut String) -> bool {
   str.pop();
   str.pop();
   str.pop();
   true
}

fn  printBooks(books:&[String]){

   for  book  in books.iter(){
     println!("book name:{}", book)
   }
}


fn selectBookToPrint(selctor :usize , books:&[String])
{
 
    println!("xxxuxsize selctor={} ,value= {}",selctor,books[selctor]) 
  
}


fn printIntArr(intArr :&[i32]){

    for  i  in intArr.iter(){
     
       println!("i={}", i);
       

    }

}

fn test1() {

   let mut clothes = String::from("#####");
   clothes = new_look(clothes);

    println!("clothes={} ",clothes);
    passByRef(&clothes);
    changeValueByRef( &mut clothes);
    println!("clothes={} ",clothes);

   
    let  books = vec![
              String::from("1 english"),
              String::from("2 math"),
              String::from("3 pyhics")  
    ];

    let  mut intArr = vec![1,2];
     for i in 1..10
    {
        intArr.push(i);
    }
     

     printBooks(&books);
     selectBookToPrint(2 as usize,&books);
     printIntArr(&intArr);
}

fn testHash(){
    let mut booksMap = HashMap::new();

    booksMap.insert(1,String::from("english"));
    booksMap.insert(2,String::from("math"));
    booksMap.insert(3,String::from("phycis"));
   
  

}


 fn main() {
    testHash();
   
    
}
