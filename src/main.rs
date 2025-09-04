use idjits;
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

  /// The pneumonics to use to generate valid idjits. Can be specified multiple
  /// times. Follows the convention '(a|b)(c|d|e)'.
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

  let branches = idjits::compute_branches(&pneumonics, &raw_idjits);
  let idjits = idjits::construct_idjits(&branches, &prefix);

  for idjit in idjits {
    println!("{}", idjit);
  }
}
