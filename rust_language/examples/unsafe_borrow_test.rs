use std::cell::RefCell;
use std::rc::Rc;
use futures::future::Lazy;
use rand::Rng;

#[derive(Default, Debug)]
struct Point(i32, i32);

#[derive(Default, Debug)]
struct Test {
    p: Point,
    i: i32,
}

impl Test
{
    pub fn test(&mut self, p: &mut Point)
    {
        p.1 += 42;
        self.i += 111;
    }
    pub fn test_self(&mut self)
    {
        let p_ptr = &mut self.p as *mut Point;
        unsafe
            {
                self.test(&mut *p_ptr)
            }
    }
}
struct StaticData
{

}

fn main() {
    let mut t = Test::default();
    let mut p = Point::default();
    t.test(&mut p);
    println!("{:?} p {:?}", t, p);
    t.test_self();
    println!("{:?}", t);
}