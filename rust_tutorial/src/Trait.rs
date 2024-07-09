#![allow(unused)]
fn main() {

   
    trait Shape {
         fn new(length: f32, width:f32) ->Self;
         fn area(&self) -> f32;
    }
    struct Rectangle{length: f32, width:f32};
    struct Circle{length: f32, width:f32};

    //let rec = Rectangle{length:4, height:10.5};

    impl Shape for Rectangle {
        fn new(length: f32, width:f32) ->Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    const PI:f32 = 3.141592;
    
    impl Shape for Circle {
        fn new(length: f32, width:f32) ->Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Area of Rectangle : {}",rec.area());
    println!("Area of Circle : {}",circ.area());
}
