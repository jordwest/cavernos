use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "new")]
struct NewOpts {
  #[structopt(short, long)]
  name: String,

  #[structopt(short, long)]
  language: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "cavernos")]
enum Cli {
  #[structopt(name = "new")]
  New(NewOpts),
}

fn main() {
  let opts = Cli::from_args();
  println!("Hello, world! {:#?}", opts);
}
