use rand::Rng;

fn main() {
    let _ = dbg!({
        let mut a = rand::thread_rng().gen_range(1, 101);
        a += 1000;
        a - 100
    });
}
