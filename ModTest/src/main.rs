
pub mod crossAPI;
pub mod crossAPI::msg::*;

fn main() {
 
 
  let p = crossAPI::cross::Person{};
crossAPI::cross::Person::staticFun();
 // p.goo();

  crossAPI::msg::receiveMsg();

 
}
