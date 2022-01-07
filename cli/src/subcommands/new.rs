use structopt::StructOpt;
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;

#[derive(StructOpt, Debug)]
#[structopt(name = "new")]
pub struct NewOpts {
  #[structopt(name = "NAME")]
  name: String,

  #[structopt(short, long)]
  template: String,
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub fn new_command(opts: NewOpts) {
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

    // Extract template specific files
    if let Ok(path) = path.strip_prefix("templates") {
      // Check it's for the template we want
      if let Ok(path) = path.strip_prefix(&opts.template) {
        let embedded_file = Asset::get(&filename).unwrap();

        let path = Path::new(&project_name).join(path);

        let containing_dir = path.parent().unwrap();
        fs::create_dir(containing_dir);

        fs::write(path, embedded_file.data);
      }
    }
  }
}

