pub mod crossApi{
// 利用结构体定义成员变量
pub struct Fruit {
    color: String,
    weight: f32
}
// 利用impl关键字来定义结构体成员方法
impl Fruit {
    // 相当于方法Fruit::new()调用
    pub fn new(color: String,weight:f32) -> Self {
        Fruit {
            color: color,
            weight: weight
        }
    }
    pub fn printInfo(&self) {
        println!("{},{}",self.color,self.weight);
    }
}

}