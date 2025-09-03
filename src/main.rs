use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// The prefix to attach to the pneumonics.
  #[arg(long)]
  prefix: Option<String>,

  /// All possible valid idjits, separated by a comma.
  #[arg(short, long, value_delimiter = ',')]
  idjits: Vec<String>,

  /// The pneumonics to use to generate valid idjits. Can be specified multiple times.
  #[arg(short, long)]
  pneumonic: Vec<String>,
}

fn main() {
  let args = Args::parse();
  let prefix = args.prefix.expect("--prefix is required.");
  let pneumonics = args.pneumonic;
  let raw_idjits = args.idjits;

  if pneumonics.len() < 1 {
    panic!("At least one --pneumonic is required.");
  }

  if raw_idjits.len() < 1 {
    panic!("At least one idjit (--idjits) must be provided.");
  }

  let branches: Vec<&str> = Vec::new();
  for pneumonic in &pneumonics {
    let optional_parts: Vec<&str> = pneumonic
        .split(&['(', ')'])
        .into_iter()
        .filter(|&item| item != "")
        .collect();

    for optional_part in &optional_parts {
      let optional_idjits: Vec<&str> = optional_part.split('|').collect();
      validate_idjits(&raw_idjits, &optional_idjits);
    }
  }

  println!("Hello, world! {}, {:?}, {:?}", prefix, pneumonics, raw_idjits);
}

fn validate_idjits(idjits: &Vec<String>, possible_idjits: &Vec<&str>) {
  for possible_idjit in possible_idjits {
    if !idjits.contains(&String::from(*possible_idjit)) {
      panic!("'{}' is not a valid idjit.", possible_idjit);
    }
  }

  // otherwise all is good
}
