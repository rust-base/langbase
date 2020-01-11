//引用 有 自动解引用作用
fn getMax<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if (a > b) {
        a
    } else {
        b
    }
}

fn main() {
    let a = 100;
    let mut max = &0;// 可变变量max 是一个指针
    {
        const  b:i32 = 200;
        max = getMax(&a, &b);
        println!("c:{}", max);
    }
}
