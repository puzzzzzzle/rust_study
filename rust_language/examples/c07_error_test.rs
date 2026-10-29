#![cfg_attr(not(test), allow(dead_code))]

use std;
use std::fs::File;
use std::io;
#[cfg(test)]
use std::io::ErrorKind;
use std::io::Read;

#[test]
#[should_panic]
fn panic_test() {
    panic!("crash here")
}

#[test]
#[allow(unused_variables)]
#[should_panic]
fn result_match_test() {
    let dir = tempfile::tempdir().unwrap();
    let file_name = dir.path().join("hello.txt");
    let f = File::open(&file_name);
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
}

#[test]
#[allow(unused_variables)]
fn result_match_resume_test() {
    let dir = tempfile::tempdir().unwrap();
    let file_name = dir.path().join("hello.txt");
    let f = File::open(&file_name);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

#[test]
#[allow(unused_variables)]
#[should_panic]
fn unwrap_test() {
    let dir = tempfile::tempdir().unwrap();
    let f = File::open(dir.path().join("hello.txt")).unwrap();
}

#[test]
#[allow(unused_variables)]
#[should_panic]
fn expect_test() {
    let dir = tempfile::tempdir().unwrap();
    let f = File::open(dir.path().join("hello.txt")).expect("Failed to open");
}

fn read_username_from_file(name: &str) -> Result<String, io::Error> {
    let f = File::open(name);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_simple(name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(name)?.read_to_string(&mut s)?;
    Ok(s)
}

#[test]
#[allow(unused_variables)]
fn test_read_file() {
    let dir = tempfile::tempdir().unwrap();
    let file_name = dir.path().join("hello.txt");
    std::fs::write(&file_name, "hello").unwrap();
    let file_name = file_name.to_str().unwrap();
    let s = read_username_from_file(file_name).unwrap();
    println!("{}", s);
    let s = read_username_from_file_simple(file_name).unwrap();
    println!("{}", s);
    let s = std::fs::read_to_string(file_name).unwrap();
    println!("{}", s);
}

fn main() {}
