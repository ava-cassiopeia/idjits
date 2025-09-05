mod validation;

pub fn construct_idjits(branches: &[String], prefix: &str) -> Vec<String> {
  let mut output_idjits: Vec<String> = Vec::new();

  for branch in branches {
    let mut output_alias = prefix.to_string();
    output_alias.push_str(branch);

    let mut output_idjit_pipeline_list: Vec<String> = Vec::new();
  
    for c in branch.chars() {
      let mut idjit_part = prefix.to_string();
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

pub fn compute_branches(phrases: &[String], pneumonics: &[String]) -> Vec<String> {
  let mut branches: Vec<String> = Vec::new();
  for phrase in phrases {
    let mut phrase_branches: Vec<String> = Vec::new();
    let optional_parts: Vec<&str> = phrase
        .split(&['(', ')'])
        .into_iter()
        .filter(|&item| item != "")
        .collect();

    for optional_part in &optional_parts {
      let optional_pneumonics: Vec<String> = optional_part.split('|').map(|i| i.to_string()).collect();
      validation::validate_pneumonics(&pneumonics, &optional_pneumonics);

      // Fill out the branches
      let mut new_branches: Vec<String> = Vec::new();
      for existing_branch in &phrase_branches {
        for phrase_pneumonic in &optional_pneumonics {
          let mut new_branch = existing_branch.clone();
          new_branch.push_str(phrase_pneumonic);
          new_branches.push(new_branch);
        }
      }
      phrase_branches.extend(new_branches);
      for phrase_pneumonic in &optional_pneumonics {
        phrase_branches.push(phrase_pneumonic.to_string());
      }
    }

    branches.extend(phrase_branches);
  }

  return branches.into_iter().filter(|b| b.len() > 1).collect();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compute_branches() {
    let phrases = vec![
      "(a|b)(c)(d|e)".to_string(),
    ];
    let pneumonics = vec![
      "a".to_string(),
      "b".to_string(),
      "c".to_string(),
      "d".to_string(),
      "e".to_string(),
    ];
  
    let result = compute_branches(&phrases, &pneumonics);

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
  fn test_compute_branches_with_multiple_phrases() {
    let phrases = vec![
      "(a|b)(c)".to_string(),
      "(d|e)(a)".to_string(),
    ];
    let pneumonics = vec![
      "a".to_string(),
      "b".to_string(),
      "c".to_string(),
      "d".to_string(),
      "e".to_string(),
    ];
  
    let result = compute_branches(&phrases, &pneumonics);

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
