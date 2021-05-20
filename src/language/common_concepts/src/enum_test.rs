use std::fmt;

mod row_enum {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    #[test]
    fn emun_test() {
        let v4 = IpAddrKind::V4;
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }
}

mod enum_with_str {
    use std::fmt;
    use std::fmt::Formatter;

    #[derive(Debug)]
    struct UnknownIpAddr {
        name: String,
        msg: String,
        value: i32,
    }

    impl fmt::Display for UnknownIpAddr {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{},{},{}", self.name, self.msg, self.value)
        }
    }

    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
        UNKNOWN(UnknownIpAddr),
    }

    impl fmt::Display for IpAddr {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                IpAddr::V4(s) => write!(f, "ip4 :{}", s),
                IpAddr::V6(s) => write!(f, "ip6 :{}", s),
                IpAddr::UNKNOWN(s) => write!(f, "UNKNOWN :{}", s),
            }
        }
    }

    #[test]
    fn enum_test() {
        let home = IpAddr::V4(String::from("127.0.0.1"));
        println!("{:?}", home);

        let loopback = IpAddr::V6(String::from("::1"));
        println!("{:?}", loopback);
        let unknown = IpAddr::UNKNOWN(UnknownIpAddr {
            name: String::from("khalid"),
            msg: String::from("tao"),
            value: 42,
        });
        println!("{:#?}", unknown);
        println!("{}", unknown);
        let some_value = Some(3);
        let res = if let Some(3) = some_value{
            println!("three");
            "aha"
        }else {
            "fail"
        };
        println!("{}",res)

    }
}
