mod validation;

pub fn construct_idjits(branches: &Vec<String>, prefix: &String) -> Vec<String> {
  let mut output_idjits: Vec<String> = Vec::new();

  for branch in branches {
    let mut output_alias = prefix.clone().to_string();
    output_alias.push_str(branch);

    let mut output_idjit_pipeline_list: Vec<String> = Vec::new();
  
    for c in branch.chars() {
      let mut idjit_part = prefix.clone();
      idjit_part.push(c);
      output_idjit_pipeline_list.push(idjit_part);
    }

    let output_idjit_pipeline = output_idjit_pipeline_list.join(" && ");
    if output_idjit_pipeline == output_alias {
      continue;
    }

    output_idjits.push(format!("alias {}='{}'", output_alias, output_idjit_pipeline));
  }

  return output_idjits;
}

pub fn compute_branches(pneumonics: &Vec<String>, valid_idjits: &Vec<String>) -> Vec<String> {
  let mut branches: Vec<String> = Vec::new();
  for pneumonic in pneumonics {
    let mut pneumonic_branches: Vec<String> = Vec::new();
    let optional_parts: Vec<&str> = pneumonic
        .split(&['(', ')'])
        .into_iter()
        .filter(|&item| item != "")
        .collect();

    for optional_part in &optional_parts {
      let optional_idjits: Vec<&str> = optional_part.split('|').collect();
      validation::validate_idjits(&valid_idjits, &optional_idjits);

      // Fill out the branches
      let mut new_branches: Vec<String> = Vec::new();
      for existing_branch in &pneumonic_branches {
        for idjit in &optional_idjits {
          let mut new_branch = existing_branch.clone();
          new_branch.push_str(*idjit);
          new_branches.push(new_branch);
        }
      }
      pneumonic_branches.extend(new_branches);
      for idjit in &optional_idjits {
        pneumonic_branches.push(idjit.to_string());
      }
    }

    branches.extend(pneumonic_branches);
  }

  return branches.into_iter().filter(|b| b.len() > 1).collect();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compute_branches() {
    let pneumonics = vec![
      "(a|b)(c)(d|e)".to_string(),
    ];
    let valid_idjits = vec![
      "a".to_string(),
      "b".to_string(),
      "c".to_string(),
      "d".to_string(),
      "e".to_string(),
    ];
  
    let result = compute_branches(&pneumonics, &valid_idjits);

    assert_eq!(result, vec![
      "ac",
      "bc",
      "ad",
      "ae",
      "bd",
      "be",
      "acd",
      "ace",
      "bcd",
      "bce",
      "cd",
      "ce",
    ]);
  }

  #[test]
  fn test_compute_branches_with_multiple_pneumonics() {
    let pneumonics = vec![
      "(a|b)(c)".to_string(),
      "(d|e)(a)".to_string(),
    ];
    let valid_idjits = vec![
      "a".to_string(),
      "b".to_string(),
      "c".to_string(),
      "d".to_string(),
      "e".to_string(),
    ];
  
    let result = compute_branches(&pneumonics, &valid_idjits);

    assert_eq!(result, vec![
      "ac",
      "bc",
      "da",
      "ea",
    ]);
  }

  #[test]
  fn test_construct_idjits() {
    let branches = vec![
      "ab".to_string(),
      "abc".to_string(),
    ];

    let idjits = construct_idjits(&branches, &"pre".to_string());

    assert_eq!(idjits, vec![
      "alias preab='prea && preb'".to_string(),
      "alias preabc='prea && preb && prec'".to_string(),
    ]);
  }
}
