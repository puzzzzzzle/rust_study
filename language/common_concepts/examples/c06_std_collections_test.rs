#[allow(dead_code)]
mod test_collections {
    #[test]
    fn test_vec() {
        let mut v1: Vec<i32> = Vec::new();
        v1.push(42);
        v1[0] = 2;
        println!("{:?}", v1);
        let v1 = vec![1, 2, 3];
        println!("{:?}", v1);

        let mut v2 = vec![MyVecItem::NONE];
        v2.push(MyVecItem::STRING(String::from("hello")));
        println!("{:?}", v2)
    }

    #[derive(Debug)]
    enum MyVecItem {
        NONE,
        STRING(String),
        INT(i32),
    }

    #[test]
    #[allow(dead_code)]
    fn test_string() {
        let h = "hello";
        let w = "world";
        // error : string 相加时, 第一个必须是 String类型, 不能是&str, 且执行后,第一个就被拿走所有权了
        // let hw = h+w;
        let mh = h.to_string();
        let hw = mh + w;
        println!("{} {} {} {}", h, w, "mh cannot use now", hw);

        let mh = h.to_string();
        let hw = format!("{} {}", mh, w);
        println!("{} {} {} {}", h, w, "mh cannot use now", hw);

        let s1 = "hello中文".to_string();
        // error : 不允许直接索引字符串
        // let h = &s1[0];

        // ascii 只占1个u8
        let h = &s1[0..5];
        println!("{}", h);
        // error  取半个字符会有运行时错误
        // let h = &s1[5..6];
        let h = &s1[5..8];
        println!("{}", h);

        // 最好用chars 遍历
        for c in s1.chars() {
            println!("{}", c)
        }
        for c in s1.bytes() {
            println!("{}", c)
        }
        println!("{} {:?}", s1.chars().count(), s1.chars())
    }

    use std::collections::HashMap;

    #[test]
    fn test_map() {
        // map 会拿走所有权
        let mut map1 = HashMap::new();
        // 类似c++的[]符号, 不存在就创建, 存在就更新
        map1.insert(String::from("k1"), "v1".to_string());
        println!("{:?}", map1.get("k1"));
        // [] Index运算符, get的简化
        println!("{:?}", map1["k1"]);
        // 访问不存在的元素岁抛错误
        // 类似Python的逻辑, get or expect
        // println!("{:?}",map1["k2"]);
        println!("{:?}", map1.get("k2").unwrap_or(&"not found".to_string()));
        // 不存在再更新等逻辑
        map1.entry("k2".to_string()).or_insert("v2".to_string());
        map1.entry("k2".to_string()).or_insert("v22".to_string()); // 以存在后, 不会更新
        println!("{:?}", map1["k2"]);

        {
            let et = map1.entry("k2".to_string()).or_insert("v2".to_string());
            *et = format!("{}  hahaha", et);
            // et1 不可更新了 变为immutable
            println!("{:?}", map1["k2"]);
            // et1 不可用了 second mutable
            let et2 = map1.entry("k2".to_string()).or_insert("v2".to_string());
            *et2 = format!("{}  hahaha", et2);
            println!("{:?}", map1["k2"]);
            // error
            // *et ="".to_string()
        }
        // 一切恢复

        println!("{:?}", map1["k2"]);
    }
}

fn main() {}
