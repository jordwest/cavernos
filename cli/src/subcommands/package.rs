use structopt::StructOpt;
use std::fs;
use std::path::Path;
use rust_embed::RustEmbed;
use serde_json::{Value, Map};

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

  let manifest_data = fs::read(manifest_path)
    .map_err(|_| format!("Error reading manifest file. Are you in the right directory?"))?;
  
  let mut v: Value = serde_json::from_slice(&manifest_data)
    .map_err(|e| format!("Error parsing manifest: {}", e))?;
  
  let dist_directory = manifest_directory.join(Path::new("dist"));
  
  fs::create_dir_all(&dist_directory);
  
  let manifest_dict = v.as_object_mut().ok_or("Invalid manifest. Expected a JSON object at the root.")?;
  
  let manifest_version = manifest_dict.get("manifestVersion")
    .ok_or("Missing `manifestVersion` key")?
    .as_i64()
    .ok_or("Expected `manifestVersion` to be an integer")?;
  
  if manifest_version > MAX_SUPPORTED_MANIFEST_VERSION {
    let cli_path = std::env::current_exe().unwrap();
    let cli_name = cli_path.file_name().unwrap().to_str().unwrap();
    eprintln!(
      "WARNING: Manifest version {} not supported, things may not behave as expected. Maximum version supported is {}. Do you need to upgrade {}?",
      manifest_version, MAX_SUPPORTED_MANIFEST_VERSION, cli_name
    )
  }
  
  copy_and_update_path(manifest_dict, manifest_directory, "program", "program.wasm")?;
  copy_and_update_path(manifest_dict, manifest_directory, "palette", "palette.png")?;
  
  let fonts_dict = manifest_dict
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
  
  let manifest_data = serde_json::to_vec_pretty(manifest_dict).unwrap();
  fs::write((&dist_directory).join("manifest.json"), manifest_data);
  
  Ok(())
}
