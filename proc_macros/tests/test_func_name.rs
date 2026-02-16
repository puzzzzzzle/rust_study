use proc_macros::func_name;

#[func_name]
fn default_macro_name() -> &'static str {
    func_name!()
}

#[func_name(name = internal_name)]
fn avoid_name_conflict() -> (&'static str, &'static str) {
    macro_rules! func_name {
        () => {
            "local_macro"
        };
    }

    (func_name!(), internal_name!())
}

#[test]
fn test_default_macro_name() {
    assert_eq!("default_macro_name", default_macro_name());
}

#[test]
fn test_avoid_name_conflict() {
    let (local, generated) = avoid_name_conflict();
    assert_eq!("local_macro", local);
    assert_eq!("avoid_name_conflict", generated);
}
