use rand::Rng;

fn main() {
    let a = dbg!({
        let mut a = rand::rng().random_range(1.. 101);
        a += 1000;
        a - 100
    });
    println!("{}", a)
}
