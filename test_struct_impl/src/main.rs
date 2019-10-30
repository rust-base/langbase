

// 参考:
// https://github.com/rust-unofficial/patterns/blob/master/patterns/builder.md
// https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
// https://en.wikipedia.org/wiki/Builder_pattern
use std::io;


// Production
#[derive(Debug)]
struct Car {
    wheels: i32,
    seats: i32,
    color: String
}


// Builder
struct Builder {
    car: Car
}


impl Builder {
    fn new() -> Builder {
        Builder {
            car: Car {                                               // default value
                wheels: 4,
                seats: 5,
                color: String::from("black")
            }
        }
    }

    #[allow(dead_code)]
    fn set_wheels(&mut self, value: i32) -> &mut Builder {
        self.car.wheels = value;
        self                                                         // return self
    }

    fn set_seats(&mut self, value: i32) -> &mut Builder {
        self.car.seats = value;
        self                                                         // return self
    }

    fn set_color(&mut self, value: String) -> &mut Builder {
        self.car.color = value;
        self                                                         // return self
    }

    fn get_car(&mut self) -> io::Result<&Car> {
        Ok(&self.car)                                               // return Result
    }
}


// Director
fn main() {
    let mut builder = Builder::new();
    let car = builder.set_seats(7).set_color(String::from("red")).get_car();
    println!("{:?}", car)
}
