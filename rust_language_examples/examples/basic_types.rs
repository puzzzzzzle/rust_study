use std::{fmt};
use std::str::FromStr;

#[derive(Debug)]
struct  Printable(i32);
impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value is : {}", self.0)
    }
}

#[test]
fn test_fmt() {
    let p = Printable(10);
    assert_eq!(format!("{}", p), "value is : 10");
    assert_eq!(format!("{:?}", p), "Printable(10)");
    println!("{}", p);
    println!("{:?}", p); // debug
}

#[test]
fn test_basic_types() {
    let x = 5;
    let x = 5f32; // float
    let x = ("sss",42i128);

}

#[test]
fn test_array(){
    let xs = [1, 2, 3];
    let xs = [0;5];
    let xs :[f32;5]=[0f32;5];
    println!("{}",xs.len());
    println!("{}",size_of_val(&xs));
    for i in 0..xs.len() + 1{
        match xs.get(i) {
            None => {
                println!("None");
            }
            Some(data) => {
                println!("Some({})",data);
            }
        }
    }
    let s = &xs[1..4];
    println!("{:?}",s);
    println!("{}",size_of_val(s));
}
#[test]
fn test_vector()
{
    let mut vec = Vec::new();
    // 允许延迟绑定类型
    vec.push(1);
    vec.push(2);
    println!("{:?}",vec);
    let mut vec = vec![1, 2, 3];
    println!("{:?}",vec);
    type I32Vec = Vec<i32>;
    let mut vec: I32Vec = vec![1, 2, 3];
    println!("{:?}",vec);
}
#[derive(Debug)]
struct Vec2D(i32,i32);
#[derive(Debug)]
struct Point
{
    x: i32,
    y: i32,
}
// 实现了from trait, 等价于实现了into trait
// 反之则不满足
impl From<Vec2D> for Point {
    fn from(vec2d: Vec2D) -> Self {
        Point {
            x: vec2d.0,
            y: vec2d.1,
        }
    }
}
impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Invalid input".to_string());
        }
        let x: i32 = parts[0].parse().or(Err("Invalid input".to_string()))?;
        let y: i32 = parts[1].parse().or(Err("Invalid input".to_string()))?;
        Ok(Point { x, y })
    }
}
#[test]
fn test_from_str(){
    // 下面3中方式等价
    let point :Point= "1,2".parse().unwrap();
    let point = Point::from_str("1,2").unwrap();
    let point  = "1,2".parse::<Point>().unwrap();

    println!("{:?}",point);
    let vec2d = Vec2D(point.x,point.y);
    println!("{:?}",vec2d);
    let point: Point = vec2d.into();
    println!("{:?}",point);
}
fn main() {}