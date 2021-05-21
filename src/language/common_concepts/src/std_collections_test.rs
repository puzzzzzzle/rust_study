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
        println!("{:?}",v2)
    }

    #[derive(Debug)]
    enum MyVecItem {
        NONE,
        STRING(String),
        INT(i32),
    }
}