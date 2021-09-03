use libc;

// 导入c的函数
extern
{
    pub fn c_zero() -> libc::c_int;
    #[link_name = "c_one_t"]
    pub fn c_one() -> libc::c_int;

    pub fn c_get_rs_value() -> libc::c_int;
}

/// 给c 导出的函数
/// ## Example
/// ```c
/// extern int rs_zero();
/// ```
#[no_mangle]
pub extern "C" fn rs_zero() -> libc::c_int
{
    println!("c call rs func");
    0
}

// 调用c的函数的safe包装

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


pub fn c_get_pid() -> libc::c_int
{
    unsafe
        {
            libc::getpid()
        }
}