
pub fn sum(a: u32, b: u32) -> u32 {
  a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(sum(2, 2), 4);
    }
}
