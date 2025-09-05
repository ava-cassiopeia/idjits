pub fn validate_pneumonics(pneumonics: &Vec<String>, maybe_pneumonics: &Vec<&str>) {
  for maybe_pneumonic in maybe_pneumonics {
    if !pneumonics.contains(&String::from(*maybe_pneumonic)) {
      panic!("'{}' is not a valid pneumonic. Options are: {:?}", maybe_pneumonic, pneumonics);
    }
  }

  // otherwise all is good
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_pneumonics_validates() {
    let pneumonics = vec![
      "a".to_string(),
      "b".to_string(),
      "c".to_string(),
    ];
    let possible_pneumonics = vec![
      "a",
      "b",
    ];
  
    // No need to assert - this will panic if validation fails.
    validate_pneumonics(&pneumonics, &possible_pneumonics);
  }

  #[test]
  #[should_panic(expected = "'d' is not a valid pneumonic.")]
  fn test_validate_pneumonics_panics_if_invalid() {
    let pneumonics = vec![
      "a".to_string(),
      "b".to_string(),
      "c".to_string(),
    ];
    let possible_pneumonics = vec![
      "a",
      "b",
      "d",
    ];
  
    validate_pneumonics(&pneumonics, &possible_pneumonics);
  }
}
