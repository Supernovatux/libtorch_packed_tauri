use tauri_build::{try_build, Attributes, WindowsAttributes};
use futures_util::StreamExt;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Builder;
use zip_extensions::zip_extract;
fn main()  -> Result<(), Box<dyn Error>>{
     
  let rel_path = Path::new("tools/Release");
  fs::create_dir_all(rel_path)?;
  let req_dll = env!("REQ_DLL");
  let target = env::var("OUT_DIR").unwrap();
  println!("{}", req_dll);
  let runtime = Builder::new_multi_thread()
  .worker_threads(4)
  .thread_name("my-custom-name")
  .thread_stack_size(3 * 1024 * 1024)
.enable_time()
.enable_io()
  .build()
  .unwrap();
runtime.block_on(do_async_stuff())?;
  for i in req_dll.split(',') {
      println!("{}", i);
      fs::copy(
          format!("tools/libtorch/lib/{}", i),
          format!("tools/Release/{}", i),
      )
      .unwrap_or(1);
      fs::copy(
          format!("C:/Windows/System32/{}", i),
          format!("tools/Release/{}", i),
      )
      .unwrap_or(1);
      fs::copy(format!("tools/libtorch/lib/{}",i), format!("target/debug/{}",i)).unwrap_or(1);
      fs::copy(format!("C:/Windows/System32/{}",i), format!("target/debug/{}",i)).unwrap_or(1);
      fs::copy(format!("tools/libtorch/lib/{}",i), format!("target/release/{}",i)).unwrap_or(1);
      fs::copy(format!("C:/Windows/System32/{}",i), format!("target/release/{}",i)).unwrap_or(1);
      fs::copy(
          format!("tools/libtorch/lib/{}", i),
          format!("{}/{}", target, i),
      )
      .unwrap_or(1);
      fs::copy(
          format!("C:/Windows/System32/{}", i),
          format!("{}/{}", target, i),
      )
      .unwrap_or(1);
  }
  if let Err(error) = try_build(
    Attributes::new()
      .windows_attributes(WindowsAttributes::new().window_icon_path("../.icons/icon.ico")),
  ) {
    panic!("error found during tauri-build: {}", error);
  }
  Ok(())
}
async fn do_async_stuff() -> Result<(), Box<dyn Error>> {
  let tool_path = Path::new("tools");
  fs::create_dir_all(tool_path)?;
  println!("Downloading the libtorch");
  if !Path::new("tools/libtorch").exists() {
      download_files("https://download.pytorch.org/libtorch/cpu/libtorch-win-shared-with-deps-2.1.0%2Bcpu.zip", "tools/libtorch.zip").await?;
      println!("Extracting the file");
      if !Path::new("tools/libtorch").exists() {
      zip_extract(
          &PathBuf::from("tools/libtorch.zip"),
          &PathBuf::from("tools/"),
      )?;
  }
  };
  Ok(())
}

async fn download_files(url: &str, path: &str) -> Result<(), Box<dyn Error>> {
  let mut file = File::create(path).await?;
  println!("Downloading {}...", url);

  let mut stream = reqwest::get(url).await?.bytes_stream();

  while let Some(chunk_result) = stream.next().await {
      let chunk = chunk_result?;
      file.write_all(&chunk).await?;
  }

  file.flush().await?;

  println!("Downloaded {}", url);
  Ok(())
}
