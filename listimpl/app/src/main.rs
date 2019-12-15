use List::*;
 
enum List { 
    Cons1(i32, Box<List>), 
    Nil,
}
 
// Methods can be attached to an enum
impl List { 
     #[inline]
    fn new() -> List { 
        Nil
    }
 
    #[inline]
    fn prepend(self, elem: i32) -> List { 
        Cons1(elem, Box::new(self))
    }
 
    fn len(&self) -> i32 { 
        match *self { 
            Cons1(_, ref tail) => 1 + tail.len(), 
            Nil => 0
        }
    }
 
    fn stringify(&self) -> String {
        match *self {
            Cons1(head, ref tail) => { 
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}
 
fn reverseList(list:List, acc:List ) -> List{
	match list{
		Cons1(val,tail) => {
			reverseList(*tail,acc.prepend(val))
		}
		Nil => acc
	}
} 
 
 
fn main() {   
	let mut head = List::new(); 
	let mut i=0; 
	while i < 10 {
		i+=1;
		head = head.prepend(i);
	}
	println!("{:30}",head.stringify());
	let result = List::new();
	let result = reverseList(head,result); 
    println!("{:30}",result.stringify());
}