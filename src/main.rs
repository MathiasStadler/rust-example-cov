fn main() {
    println!("Hello, world!");
	println!("{} {} ", answer(), not_tested());
	
   
    
}

pub fn answer() -> u32 {
    42
}

pub fn not_tested() -> u32 {
    43
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(answer(), 42);
    }

    #[test]
    fn test_not_tested() {
        assert_eq!(not_tested(), 43);
    }
}
