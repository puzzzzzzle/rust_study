// use std::iter;
// use std::vec::IntoIter;

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

// 带引用的struct和函数, 都必须指定生命周期
#[derive(Debug)]
#[allow(dead_code)]
struct PersonInfo<'a> {
    name: &'a str,
    location: &'a str,
}

impl<'a> Drop for PersonInfo<'a> {
    fn drop(&mut self) {
        println!("drop {:?}", self)
    }
}
#[test]
fn test_ref_st() {
    let p = PersonInfo {
        name: "tao",
        location: "unknown",
    };
    println!("{:?}", p)
}

// 该函数组合了两个 `Vec <i32>` 并在其上返回一个迭代器。
// 看看它的返回类型多么复杂！
// fn combine_vecs_explicit_return_type(
//     v: Vec<i32>,
//     u: Vec<i32>,
// ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
//     v.into_iter().chain(u.into_iter()).cycle()
// }

// 这是完全相同的函数，但其返回类型使用 `impl Trait`。
// 看看它多么简单！
fn combine_vecs<T: Clone>(v: Vec<T>, u: Vec<T>) -> impl Iterator<Item = T> {
    v.into_iter().chain(u.into_iter()).cycle()
}
#[test]
fn test_ret_type() {
    let v1 = (0..10).collect();
    let v2 = (10..20).collect();
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(v3.next(), Some(0));
    assert_eq!(v3.next(), Some(1));
    assert_eq!(v3.next(), Some(2));
}
fn main() {}
