extern crate libc;

// 导入c的函数
extern
{
    pub fn c_zero() -> libc::c_int;
    pub fn c_get_rs_value() -> libc::c_int;
}

// 给c 导出的函数
#[no_mangle]
pub extern "C" fn rs_zero() -> libc::c_int
{
    println!("c call rs func");
    let ret: libc::c_int = 0;
    ret
}

pub fn c_zero_safe() -> libc::c_int {
    unsafe
        {
            c_zero()
        }
}

pub fn get_rs_value_safe() -> libc::c_int
{
    unsafe
        {
            c_get_rs_value()
        }
}