use std::collections::HashMap;
use std::future::Future;
use std::net::ToSocketAddrs;
use structopt::StructOpt;
use rust_embed::RustEmbed;
use warp::{Filter, Rejection, Reply};
use warp::filters::BoxedFilter;
use std::rc::Rc;
use std::path::{PathBuf, Path};
use tokio::runtime::Runtime;
use warp::http::StatusCode;
use serde_json::Value;
use warp::http::header::SERVER;
use warp::hyper::Body;
use crate::manifest::read_manifest;

#[derive(StructOpt, Debug)]
#[structopt(name = "run")]
pub struct RunOpts {
  #[structopt(name = "MANIFEST")]
  manifest_path: Option<String>,
}

#[derive(RustEmbed)]
#[folder = "../runtimes/web/dist"]
#[exclude = "*.js.map"]
struct RuntimeAssets;

/// Manifest held in memory with paths updated for serving the files correctly
struct ServerManifest {
  content: Vec<u8>,
  original_file_locations: HashMap<String, PathBuf>,
}

fn serve_embedded_asset(name: &str, mime: &str) -> impl warp::Reply {
  let asset = RuntimeAssets::get(name).unwrap();
  
  warp::http::Response::builder()
    .header("content-type", mime)
    .body(asset.data)
}

fn serve_file(path: &Path) -> impl warp::Reply {
  let data = std::fs::read(path).unwrap();
  
  let extension = path
    .extension()
    .map(|s| s.to_str())
    .flatten()
    .unwrap_or(".png");
  
  let mime = match extension {
    "png" => "image/png",
    "jpg" => "image/jpeg",
    "jpeg" => "image/jpeg",
    "bmp" => "image/bmp",
    "js" => "text/javascript",
    "json" => "application/json",
    "wasm" => "application/wasm",
    _ => "application/octet-stream",
  };
  
  warp::http::Response::builder()
    .header("content-type", mime)
    .body(data)
}

fn update_manifest_path(manifest_dir: &Path, dict: &mut serde_json::Map<String, Value>, paths: &mut HashMap<String, PathBuf>, key: &str, new_path: &str) {
  let rel_path_val = dict
    .insert(key.to_string(), Value::String(new_path.to_string())).unwrap();
  
  let rel_path = rel_path_val.as_str().unwrap();
  
  paths.insert(new_path.to_string(), manifest_dir.join(rel_path).to_path_buf());
}

fn get_server_manifest(manifest_path: &Path) -> ServerManifest {
  let mut paths: HashMap<String, PathBuf> = HashMap::new();
  let manifest_dir = manifest_path.parent().expect("Invalid manifest path");
  
  let mut manifest_dict = read_manifest(manifest_path).unwrap();
  
  update_manifest_path(manifest_dir, &mut manifest_dict, &mut paths, "palette", "/assets/palette.png");
  update_manifest_path(manifest_dir, &mut manifest_dict, &mut paths, "program", "/assets/program.wasm");
  
  let mut fonts_dict = manifest_dict.get_mut("fonts").unwrap().as_object_mut().unwrap();

  update_manifest_path(manifest_dir, &mut fonts_dict, &mut paths, "narrow", "/assets/font_narrow.png");
  update_manifest_path(manifest_dir, &mut fonts_dict, &mut paths, "square", "/assets/font_square.png");
  
  let content = serde_json::to_vec_pretty(&Value::Object(manifest_dict)).unwrap();
  
  ServerManifest {
    content,
    original_file_locations: paths,
  }
}

#[derive(Debug)]
struct PathMatchError;
impl warp::reject::Reject for PathMatchError {}

fn serve_assets(assets: &HashMap<String, PathBuf>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  let my_assets = assets.clone();

  let get_assets = warp::any().map(move || my_assets.clone());
  
  warp::get()
    .and(warp::path::full())
    .and(get_assets)
    .and_then(move |path: warp::path::FullPath, assets: HashMap<String, PathBuf>| async move {
      println!("Path is {}", path.as_str());
      match assets.get(path.as_str()) {
        Some(real_file_path) => Ok(serve_file(real_file_path)),
        None => Err(warp::reject::custom(PathMatchError))
      }
    })
}

async fn start_server(opts: RunOpts) {
  let index_route = warp::get()
    .and(warp::path::end())
    .map(move || serve_embedded_asset("index.html", "text/html"));
  
  let js_route = warp::get()
    .and(warp::path("main.js"))
    .map(move || serve_embedded_asset("main.js", "text/javascript"));
  
  let manifest_path_string = opts.manifest_path.unwrap_or("manifest.json".to_string());
  let manifest_path = PathBuf::from(manifest_path_string);
  
  let source_dir = manifest_path.parent().unwrap().to_path_buf();
  println!("Manifest path {:?} in {:?}", &manifest_path, source_dir);

  let server_manifest = get_server_manifest(&manifest_path);
  println!("Linked assets: {:#?}", server_manifest.original_file_locations);

  let manifest_route = warp::get()
    .and(warp::path("manifest.json"))
    .map(move || {
      let data = server_manifest.content.clone();
      let response = warp::http::Response::builder()
        .header("content-type", "application/json")
        .body(data);
      Ok(response)
    });

  let assets_route = serve_assets(&server_manifest.original_file_locations);

  let mut routes = index_route
    .or(js_route)
    .or(manifest_route)
    .or(assets_route);

  warp::serve(routes).run(([127, 0, 0, 1], 8088)).await;
}

pub fn run_command(opts: RunOpts) -> Result<(), String> {
  
  let future = start_server(opts);
  
  // Start async context
  let result = Runtime::new().unwrap().block_on(future);
  
  println!("Run");
  Ok(())
}
