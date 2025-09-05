use idjits;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// The prefix to attach to the pneumonics.
  #[arg(long)]
  prefix: Option<String>,

  /// All possible valid pneumonics, separated by commas.
  #[arg(long, value_delimiter = ',')]
  pneumonics: Vec<String>,

  /// The phrases to use to generate valid idjits. Can be specified multiple
  /// times. Follows the convention '(a|b)(c|d|e)'.
  #[arg(short, long)]
  phrase: Vec<String>,
}

fn main() {
  let args = Args::parse();
  let prefix = args.prefix.expect("--prefix is required.");
  let pneumonics = args.pneumonics;
  let phrases = args.phrase;

  if pneumonics.len() < 1 {
    panic!("At least one --pneumonic is required.");
  }

  if phrases.len() < 1 {
    panic!("At least one phrase (--phrase) must be provided.");
  }

  let branches = idjits::compute_branches(&phrases, &pneumonics);
  let idjits = idjits::construct_idjits(&branches, &prefix);

  for idjit in idjits {
    println!("{}", idjit);
  }
}
