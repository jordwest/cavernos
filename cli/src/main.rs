mod subcommands;
mod manifest;

use structopt::StructOpt;

use subcommands::new::{NewOpts, new_command};
use subcommands::package::{PackageOpts, package_command};
use subcommands::run::{RunOpts, run_command};

#[derive(StructOpt, Debug)]
#[structopt(name = "cavernos")]
enum Cli {
  #[structopt(name = "new", about = "Create a new project from a template")]
  New(NewOpts),
  
  #[structopt(name = "package", about = "Bundle up your project into a distributable package")]
  Package(PackageOpts),

  #[structopt(name = "run", about = "Run the project in the current directory")]
  Run(RunOpts),
}

fn main() {
  let subcommand = Cli::from_args();
  
  match subcommand {
    Cli::New(opts) => new_command(opts),
    Cli::Package(opts) => {
      match package_command(opts) {
        Ok(()) => (),
        Err(msg) => {
          eprintln!("Packaging error: {}", msg);
          std::process::exit(1);
        }
      }
    },
    Cli::Run(opts) => {
      match run_command(opts) {
        Ok(()) => (),
        Err(msg) => {
          eprintln!("Run error: {}", msg);
          std::process::exit(1);
        }
      }
    }
  }
}
