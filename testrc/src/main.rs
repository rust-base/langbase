 
 use std::rc::Rc;
 use std::cell::RefCell;
    struct S{
        data:i32
    }
    fn main() {
        let s = Rc::new( RefCell::new(S{data:3}));
        let s1 = Rc::clone(&s);
        s1.borrow_mut().data=43;

        let s2 = Rc::clone(&s);
        s2.borrow_mut().data=100;
        println!("{}",RefCell::borrow(&s).data);
        
    }

