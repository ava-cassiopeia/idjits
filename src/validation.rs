pub fn validate_idjits(idjits: &Vec<String>, possible_idjits: &Vec<&str>) {
  for possible_idjit in possible_idjits {
    if !idjits.contains(&String::from(*possible_idjit)) {
      panic!("'{}' is not a valid idjit.", possible_idjit);
    }
  }

  // otherwise all is good
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_idjits_validates() {
    let idjits = vec![
      "a".to_string(),
      "b".to_string(),
      "c".to_string(),
    ];
    let possible_idjits = vec![
      "a",
      "b",
    ];
  
    // No need to assert - this will panic if validation fails.
    validate_idjits(&idjits, &possible_idjits);
  }

  #[test]
  #[should_panic(expected = "'d' is not a valid idjit.")]
  fn test_validate_idjits_panics_if_invalid() {
    let idjits = vec![
      "a".to_string(),
      "b".to_string(),
      "c".to_string(),
    ];
    let possible_idjits = vec![
      "a",
      "b",
      "d",
    ];
  
    validate_idjits(&idjits, &possible_idjits);
  }
}
