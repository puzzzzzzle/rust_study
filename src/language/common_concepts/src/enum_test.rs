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
    enum IpAddr {
        V4(String),
        V6(String),
    }
    #[test]
    fn emun_test() {
        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
    }
}
