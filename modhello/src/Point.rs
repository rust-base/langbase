
    pub mod mmath{
        pub struct Point{
            x:f32,
            y:f32,
            z:f32
        }

        impl Point{
        
        pub  fn new(x:f32,y:f32,z:f32)->Point{
            Point{
                    x,y,z
            }
        }

        pub fn sqrt(&self)->f32{
            self.x*self.x+self.y*self.y+self.z*self.z
        }
        }
    }
