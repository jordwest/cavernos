use structopt::StructOpt;
use std::fs;
use std::path::Path;
use rust_embed::RustEmbed;
use serde_json::{Value, Map};
use crate::manifest::read_manifest;

#[derive(StructOpt, Debug)]
#[structopt(name = "package")]
pub struct PackageOpts {
  #[structopt(name = "MANIFEST")]
  manifest_path: Option<String>,
}

#[derive(RustEmbed)]
#[folder = "../runtimes/web/dist"]
#[exclude = "*.js.map"]
struct RuntimeAssets;

const MAX_SUPPORTED_MANIFEST_VERSION: i64 = 1;

/// Copies a file referenced in the source manifest to the dist directory, then updates
/// the manifest with a new relative path.
fn copy_and_update_path(map: &mut Map<String, Value>, manifest_dir: &Path, key: &str, new_path: &str) -> Result<(), String> {
  let relative_path = map.get(key)
    .ok_or(format!("Missing `{}` key in manifest", key))?
    .as_str()
    .ok_or(format!("Expected `{}` key to be a string", key))?;

  let source_path = manifest_dir.join(relative_path);
  
  let destination_path = manifest_dir
    .join("dist")
    .join(Path::new(new_path));
  
  eprintln!("Copying {:?} --> {:?}", source_path, destination_path);
  
  fs::copy(&source_path, &destination_path)
    .map_err(|_| format!("Could not copy {:?} to dist directory. Have you built the project?", &source_path))?;
  
  map.insert(key.to_string(), Value::String(new_path.to_string()));
  
  Ok(())
}

pub fn package_command(opts: PackageOpts) -> Result<(), String> {
  let manifest_path = opts.manifest_path.unwrap_or("manifest.json".to_string());
  let manifest_path = Path::new(&manifest_path);
  let manifest_directory = manifest_path.parent().unwrap();
  
  let mut manifest_dict = read_manifest(manifest_path)?;

  let dist_directory = manifest_directory.join(Path::new("dist"));
  
  fs::create_dir_all(&dist_directory);
  
  copy_and_update_path(&mut manifest_dict, manifest_directory, "program", "program.wasm")?;
  copy_and_update_path(&mut manifest_dict, manifest_directory, "palette", "palette.png")?;
  
  let fonts_dict = &mut manifest_dict
    .get_mut("fonts")
    .ok_or("Manifest missing `fonts` key")?
    .as_object_mut()
    .ok_or("Expected `fonts` key to be a JSON object")?;
  
  copy_and_update_path(fonts_dict, manifest_directory, "narrow", "font_narrow.png")?;
  copy_and_update_path(fonts_dict, manifest_directory, "square", "font_square.png")?;

  for filename in RuntimeAssets::iter() {
    let destination_path = dist_directory.join(filename.as_ref());
    let file = RuntimeAssets::get(filename.as_ref()).unwrap();
    fs::write(destination_path, file.data);
  }
  
  let manifest_data = serde_json::to_vec_pretty(&mut manifest_dict).unwrap();
  fs::write((&dist_directory).join("manifest.json"), manifest_data);
  
  Ok(())
}
