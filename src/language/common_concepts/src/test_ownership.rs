#[test]
fn test_ownership() {
    let s1 = String::from("hello ");
    let s2 = s1;
    // s1 不再可用
    // println!("{}",s1) //error
    println!("{}", s2);

    let s3 = s2.clone();
    // s3 只是个克隆, 正常
    println!("{} {}", s2, s3);
    take_ownership(s3);
    // s3 不再可用
    // println!("{}", s3); // error
    let s2 = takes_and_gives_back(s2);
    // s2 再次被返回
    println!("{}", s2);
}

fn take_ownership(s: String) {
    println!("{}", s);
}
// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(s: String) -> String {
    // a_string 进入作用域

    s // 返回 a_string 并移出给调用的函数
}
#[test]
fn test_ref() {
    let mut s1 = String::from("hello ");
    {
        borrow(&mut s1);
        println!("{}", s1);
        //在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
        // 引用必须总是有效的。
        let s2 = &s1;
        let s3 = &s1;
        println!("{} {}", s2, s3);
    }

    // s2 s3 不再可用
    let mut s4 = &mut s1;
    println!("{}", s4);
}
fn borrow(s: &mut String) {
    s.push_str(" world")
}

#[test]
fn test_slice() {
    let mut a = [0; 10];
    println!("{}", str_arr(&a[..]));
    let sl1 = &mut a[4..8];
    for i in sl1.iter_mut() {
        *i = 42;
    }
    println!("{}", str_arr(&a[..]));
}
fn str_arr(arr: &[i32]) -> String {
    let mut ret = String::from("");
    for i in arr.iter() {
        ret.push_str(&i.to_string());
        ret.push_str(",");
    }
    ret
}
