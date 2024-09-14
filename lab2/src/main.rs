use std::{f32::consts::PI, fmt::format};

trait Shape{
    fn area(&self) -> f32;
}

trait IPrint: ToString{
    fn print(&self){
        println!("{}",self.to_string())
    }
}

struct Rectangle{
    x: f32,
    y: f32,
}

struct Square { x: f32}

struct Circle {
    r: f32
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.x * self.y
    }
}

impl ToString for Rectangle {
    fn to_string(&self) -> String {
        return format!("Length is {}, height is {}, area is {}", self.x, self.y, self.area());
    }
}

impl IPrint for Rectangle{}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.x * self.x
    }
}

impl ToString for Square {
    fn to_string(&self) -> String {
    return format!("Length is {}, area is {}", self.x, self.area());
    }
}

impl IPrint for Square{}

impl ToString for Circle {
    fn to_string(&self) -> String {
        return format!("Radius is {}, area is {}", self.r, self.area());
    }
}

impl  Shape for Circle {
    fn area(&self) -> f32 {
        self.r * self.r * PI
    }
}

impl IPrint for Circle{}


fn main() { 
    let (r_length, r_height) = (2.0, 4.0);
    let s_length = 2.5;
    let radius = 5.0;
    let rect = Rectangle{x: r_length, y: r_height};
    let sq: Square = Square{x: s_length};
    let Mikhail: Circle = Circle{r: radius};
    // println!("{}", Mikhail.to_string());
    rect.print();
    sq.print();
    Mikhail.print();
}
