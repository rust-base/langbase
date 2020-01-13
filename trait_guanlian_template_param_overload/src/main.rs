
trait Eat<T> {
    fn eat(&self)->T;//虚函数 生成器
}

struct Animal();

 impl Eat<i32> for Animal{ ////和上面是同一个空间【里面函数被所属的空间是：===> 空间的形成： Eat<i32> +  Animal】
    fn eat(&self)->i32{ //这个实现函数 是由Eat<i32> + Animal+eat
            println!("Animal eat override i32");
            10_i32
    }
 }

 impl Eat<f64> for  Animal{//和上面是同一个空间【里面函数被所属的空间是：===> 空间的形成： Eat<f64> +  Animal】
    fn eat(&self)->f64{//这个实现函数 是由Eat<f64> + Animal+eat  不会和上面 fn eat()->i32冲突
        println!("Animal eat override f64");
        25.0_f64
    } 
 }


trait Converter{
    type Output;
    fn convert(&self)->Self::Output;
}

struct MyNumber();

impl Converter for MyNumber{
    type Output = i32;
    fn convert(&self)->i32{
        println!("MyNumber convert override i32");
        100_i32
    }
}

// impl Converter for  MyNumber{//和上面是同一个空间【里面函数被所属的空间是：===> 空间的形成： Converter +  MyNumber】 ,所以会和上面冲突 compiler error
//     type Output = f64;
//     fn convert(&self)->f64{
//         println!("MyNumber convert override f64");
//         12.36_f64
//     }
// }

//综上，如果要实现，重载，要通过 trait 模板来实现,关联参数只能实现传递一个类型参数，并不能实现根本上实现多态

fn main() {
    let amial = Animal();
    let ai32 :i32= amial.eat();

    let bf64:f64 = amial.eat();

    println!("ai32:{},bf64:{}",ai32,bf64);


    let cv =MyNumber();
    let ci32:i32 = cv.convert();
    // let cf64 :f64 =cv.convert();
    println!("ci32:{}",ai32);

}
