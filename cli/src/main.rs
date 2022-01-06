use structopt::StructOpt;
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;

#[derive(StructOpt, Debug)]
#[structopt(name = "new")]
struct NewOpts {
  #[structopt(name = "NAME")]
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

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

const SUPPORTED_LANGUAGES: [&'static str; 1] = ["rust"];

fn new_command(opts: NewOpts) {
  let project_name = opts.name;
  
  fs::create_dir(&project_name);
  
  for filename in Asset::iter() {
    let path = Path::new(filename.as_ref());
    
    // Extract common assets
    if let Ok(path) = path.strip_prefix("common") {
      let path_prefix = format!("{}/assets", &project_name);
      let path = Path::new(&path_prefix).join(path);
      
      let containing_dir = path.parent().unwrap();
      fs::create_dir(containing_dir);

      let embedded_file = Asset::get(&filename).unwrap();
      fs::write(path, embedded_file.data);
    }

    // Extract language specific files
    if let Ok(path) = path.strip_prefix("templates") {
      // Check it's for the language we want
      if let Ok(path) = path.strip_prefix(&opts.language) {
        let embedded_file = Asset::get(&filename).unwrap();
        
        let path = Path::new(&project_name).join(path);

        let containing_dir = path.parent().unwrap();
        fs::create_dir(containing_dir);

        fs::write(path, embedded_file.data);
      }
    }
  }
}

fn main() {
  let subcommand = Cli::from_args();
  
  match subcommand {
    Cli::New(opts) => new_command(opts),
  }
}
