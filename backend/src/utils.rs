use std::{fs, io::Write, path};

use futures::StreamExt;
use reqwest::IntoUrl;

pub async fn download<T: IntoUrl>(
    url: T,
    on_progress: impl Fn(u64, u64) -> (),
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path::Path::new("/Users/wayne/Downloads/temp");
    fs::remove_file(path).ok();
    let mut f = fs::OpenOptions::new().write(true).create(true).open(path)?;
    let res = reqwest::get(url).await?;
    let total = res.content_length().unwrap_or(0);
    let mut stream = res.bytes_stream();
    let mut progress: u64 = 0;
    while let Some(chunk) = stream.next().await {
        if let Ok(chunk) = chunk {
            progress = progress + chunk.len() as u64;
            f.write(&chunk)?;
            on_progress(progress, total);
        } else {
            break;
        }
    }
    Ok(())
}
