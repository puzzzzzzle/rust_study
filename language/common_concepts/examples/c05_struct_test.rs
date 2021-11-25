#[test]
#[allow(unused_variables)]
fn test_struct() {
    let mut user1 = User {
        username: "khalid".to_string(),
        email: "khalid".to_string(),
        sign_in_count: 0,
        active: true,
    };
    user1.email = String::from("khalid@kkkkk.com");
    let user2 = build_user("kkk".to_string());
    let black = Color(0, 0, 0);
    println!("{}", black.0)
}
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(username: String) -> User {
    User {
        username,
        email: "khalid".to_string(),
        sign_in_count: 0,
        active: true,
    }
}
#[allow(dead_code)]
struct Color(i32, i32, i32);
#[allow(dead_code)]
struct Point(i32, i32, i32);

#[test]
fn test_struct_func() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {}
