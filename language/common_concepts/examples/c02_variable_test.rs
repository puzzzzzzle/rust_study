#[test]
pub fn test() {
    println!("hello test")
}
#[test]
pub fn var_change() {
    let x = 5;
    println!("The value of x is: {}", x);
    // error : x 不可变
    // x = 6;

    let mut y = 6;
    println!("The value of y is: {}", y);
    y = 7;
    assert_eq!(y, 7);
    println!("The value of y is: {}", y);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    println!("The value of spaces is: {}", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    const C_F: f64 = 4.5;
    println!("The value of C_F is: {}", C_F);
    // error : 不可以覆盖const 类型
    // let mut C_F = "   ";
    // println!("The value of C_F is: {}", C_F);
    // let C_F = 44.5;
}

#[test]
#[allow(unused_variables)]
pub fn value_type() {
    let var: i32;
    // error : var  possibly-uninitialized
    // println!("The value of var is: {}", var);
    var = 42;
    println!("The value of var is: {}", var);
    // error : i32 是不可变类型的, 只允许初始化复制一次, 不允许再修改
    // 注意const类型直接要声明好值, 永远不允许在改变
    // 它们是不同的
    // var = 44;
    // println!("The value of var is: {}", var);

    let var: f64 = 4.4;
    println!("The value of var is: {}", var);

    // char是标准的u8字符, 占用4个字节, 和c++完全不同
    // 使用u8 来实现类似byte的功能
    let var: char = '😻';
    println!("The value of var is: {}", var);

    let var = (4.5, 44, '好', "哈哈哈");
    // error tuple 不会自带 to_string
    // 但是带Debug, 只要所有内部变量都是Debug的
    println!("The value of var is: {:?}", var);
    // 类似c++17的结构化绑定
    // rust中叫 解构（destructuring）
    let (a, b, c, d) = var;
    println!("The value of d is: {}", d);

    let arr = [2, 3, 3, 4, 9];
    // error : 非mut数组连元素也不允许修改
    // arr[2] = 8;
    println!("The value of arr is: {}", arr[2]);

    let mut arr: [i32; 10];

    // error : 每个元素必须初始化后才能使用
    // 看起来rust是按照每个对象记录访问, 初始化的
    // println!("The value of arr is: {}", arr[2]);

    // error: 数组没有初始化, 后面也不再允许使用
    // arr[2] = 42;
    // 初始化后就可以使用了
    arr = [-200; 10];
    arr[2] = 42;
    println!("The value of arr is: {}", arr[2]);

    let mut arr = [-100; 8];

    println!("The value of arr is: {}", arr[2]);
    arr[2] = 42;
    println!("The value of arr is: {}", arr[2]);
}

fn main() {}
