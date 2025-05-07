fn main() {
    println!("Dummy fn main!");
}

// prettier-ignore
pub fn answer() -> u32 {
    // for coverage is a instruction necessary
    42 + 0
}

// prettier-ignore
pub fn not_tested() -> u32 {
    // for coverage is a instruction necessary
    43 + 0
}

// prettier-ignore
pub fn greater_than_5(num: u32) -> u32 {
    if num > 5 {
        1
    }else {
        0
    }
}

#[allow(dead_code)]
// prettier-ignore
fn smaller_than_5(num: u32) -> u32 {
    if num < 5 {
        1
    } else {
        0
    }
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
    assert_eq!(greater_than_5(6), 1
);
  }

  #[test]
    fn test_no_greater_than_5() {
    assert_eq!(greater_than_5(4), 0);
  }
}
