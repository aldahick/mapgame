use std::{error::Error, path::Path};

use tokio::{
  fs::{create_dir, File},
  io::AsyncWriteExt,
};

pub async fn download_file(url: &str, path_str: &str) -> Result<(), Box<dyn Error>> {
  let resp = reqwest::get(url).await?.bytes().await?;
  let path = Path::new(path_str);
  if path.exists() && (resp.len() as u64) == path.metadata()?.len() {
    return Ok(());
  }
  println!("Downloading {} to {}", url, path_str);
  create_dir(path.parent().unwrap()).await?;
  let mut file = File::create(&path).await?;
  file.write_all(&resp).await?;
  println!("Downloaded to {} successfully", path_str);
  Ok(())
}
