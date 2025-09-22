use rand::{thread_rng, Rng};

fn main() {
    let a = dbg!({
        let mut a =thread_rng().next_f32();
        a += 1000.0;
        a - 100.0
    });
    println!("{}", a)
}
