use serde_json::{Map, Value};
use std::path::{PathBuf, Path};
use std::fs;

const MAX_SUPPORTED_MANIFEST_VERSION: i64 = 1;

pub fn read_manifest(manifest_path: &Path) -> Result<Map<String, Value>, String> {
  let manifest_data = fs::read(manifest_path)
    .map_err(|_| format!("Error reading manifest file. Are you in the right directory?"))?;

  let mut v: Value = serde_json::from_slice(&manifest_data)
    .map_err(|e| format!("Error parsing manifest: {}", e))?;

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
  
  Ok(manifest_dict.clone())
}