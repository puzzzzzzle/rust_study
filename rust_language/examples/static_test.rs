use std::ops::Deref;
use std::sync::Mutex;
use lazy_static::lazy::Lazy;
use lazy_static::lazy_static;

#[derive(Default, Debug)]
struct StaticData
{
    i1: i32,
    d1: f64,
    s1: String,
}

pub static mut GLB_DATA: Option<&'static StaticData> = None;

pub fn unsafe_save_gbl_data(data: & StaticData)
{
    unsafe {
        let data_ptr = data as *const StaticData as *mut StaticData;
        let boxed_data = Box::from_raw(data_ptr);
        GLB_DATA = Some(Box::leak(boxed_data))
    }
}


fn main() {
    let mut data = StaticData {
        i1: 42,
        d1: 3.14,
        s1: String::from("Hello, world!"),
    };

    unsafe_save_gbl_data(&mut data);
    unsafe {
        if let Some(gbl_data) = &GLB_DATA {
            println!("{:?}", gbl_data);
        }
    }
}