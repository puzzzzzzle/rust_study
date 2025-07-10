use std::cell::{OnceCell, RefCell};
#[derive(Default, Debug)]
pub struct StaticData
{
    i1: i32,
    d1: f64,
    s1: String,
}

pub static mut GLB_DATA: Option<&'static StaticData> = None;

pub fn unsafe_save_gbl_data(data: &StaticData)
{
    unsafe {
        let data_ptr = data as *const StaticData as *mut StaticData;
        let boxed_data = Box::from_raw(data_ptr);
        GLB_DATA = Some(Box::leak(boxed_data))
    }
}

thread_local! {
    pub static GLB_DATA_REF_CELL: RefCell<Option<&'static StaticData>> = RefCell::new(None);
}
pub fn unsafe_save_gbl_data_ref_cell(data: &StaticData) {
    unsafe {
        let data_ptr = data as *const StaticData as *mut StaticData;
        let boxed_data = Box::from_raw(data_ptr);
        let static_data: &'static StaticData = Box::leak(boxed_data);
        GLB_DATA_REF_CELL.replace(Some(static_data));
    }
}

fn main() {
    let data = StaticData {
        i1: 42,
        d1: 3.14,
        s1: String::from("Hello, world!"),
    };

    unsafe_save_gbl_data(&data);
    unsafe {
        let gbl_data_ptr = &raw const GLB_DATA;
        if let Some(gbl_data) = &*gbl_data_ptr {
            println!("{:?}", gbl_data);
        }
    }
    unsafe_save_gbl_data_ref_cell(&data);
    println!("new 2 {:?}", GLB_DATA_REF_CELL.take().unwrap());

}