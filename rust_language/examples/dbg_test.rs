use rand::RngExt;

fn main() {
    let a = dbg!({
        let mut a = rand::rng().random::<f32>();
        a += 1000.0;
        a - 100.0
    });
    println!("{}", a)
}
