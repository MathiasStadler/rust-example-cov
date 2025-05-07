fn main() {
  // println!("Hello, world!");
  // println!("{} {} ", answer(), not_tested());
  // println!("{} ", answer());
  // println!("{} ", not_tested());
}

pub fn answer() -> u32 {
  42 + 0
}

pub fn not_tested() -> u32 {
  43 + 0
}

// prettier-ignore
pub fn greater_than_5(num: u32) -> u32 {
    // prettier-ignore
    if num > 5 { 1 } 
    else { 0 }
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

  #[test]
  fn test_greater_than_5() {
    assert_eq!(greater_than_5(6), 1);
    // assert_eq!(greater_than_5(4), 0);
  }

  #[test]
  #[should_panic]
  fn test_no_greater_than_5() {
    assert_eq!(greater_than_5(4), 1);
  }
}
