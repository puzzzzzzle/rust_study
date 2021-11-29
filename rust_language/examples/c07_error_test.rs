use std;
use std::fs::remove_file;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

static FILE_NAME: &str = "hello.txt";

#[test]
#[should_panic]
fn panic_test() {
    panic!("crash here")
}

#[test]
#[allow(unused_variables)]
#[should_panic]
fn result_match_test() {
    let rm = remove_file(FILE_NAME);
    let f = File::open(FILE_NAME);
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
    let rm = remove_file(FILE_NAME);

    let f = File::open(FILE_NAME);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_NAME) {
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
    let rm = remove_file(FILE_NAME);
    let f = File::open(FILE_NAME).unwrap();
}

#[test]
#[allow(unused_variables)]
#[should_panic]
fn expect_test() {
    let rm = remove_file(FILE_NAME);
    let f = File::open(FILE_NAME).expect("Failed to open");
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
    let r = std::fs::write(FILE_NAME, "hello");
    let s = read_username_from_file(FILE_NAME).unwrap();
    println!("{}", s);
    let s = read_username_from_file_simple(FILE_NAME).unwrap();
    println!("{}", s);
    let s = std::fs::read_to_string(FILE_NAME).unwrap();
    println!("{}", s);
}

fn main() {}
