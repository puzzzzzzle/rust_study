mod ex2;

use anyhow::Result;

fn main() -> Result<()> {
    let s = ex2::hello()?;
    println!("{}", s);
    Ok(())
}
