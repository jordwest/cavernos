mod subcommands;

use structopt::StructOpt;

use subcommands::new::{NewOpts, new_command};
use subcommands::package::{PackageOpts, package_command};

#[derive(StructOpt, Debug)]
#[structopt(name = "cavernos")]
enum Cli {
  #[structopt(name = "new")]
  New(NewOpts),
  
  #[structopt(name = "package")]
  Package(PackageOpts),
}

fn main() {
  let subcommand = Cli::from_args();
  
  match subcommand {
    Cli::New(opts) => new_command(opts),
    Cli::Package(opts) => {
      match package_command(opts) {
        Ok(()) => (),
        Err(msg) => eprintln!("Packaging error: {}", msg),
      }
    },
  }
}
