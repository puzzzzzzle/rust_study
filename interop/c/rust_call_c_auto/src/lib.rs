pub mod c_libs;

#[cfg(test)]
mod tests {
    use crate::c_libs::zero;

    #[test]
    fn test_c_func()
    {
        unsafe {
            println!("zero from c auto {:?}", zero());
        }
    }
}