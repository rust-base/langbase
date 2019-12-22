

fn executePrintFn(printFn:impl Fn()->()){ //如果是 FnMut 就是可变借用函数的意思，但是本质上，pf函数实际上是不可变借用函数，所以用FnMut会报错误
    printFn();
}

fn main() {
    let mut num =1000;
    
    let pf = ||{ 
        // 注释掉 Fn
        // num=num+1;//FnMut: 定位fn是 FnMut 空间，什么空间决定了什么样的借用
        println!("hello world {}", num);//num属于可变借用
    };
    executePrintFn(pf); 
    println!("before num:{}",num);//不可以变 ，变外面这里 println 属于不可借用 ,因此同一个上下文 程序矛盾
    
}
