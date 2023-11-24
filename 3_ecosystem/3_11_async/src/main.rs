use clap::Parser;
use reqwest;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::thread::available_parallelism;
use tokio;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::runtime;

/// Simple program to download specified web pages
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Maximum number of simultaneously running threads
    #[arg(long, default_value_t = NonZeroUsize::new(available_parallelism().unwrap().get()).unwrap())]
    max_threads: NonZeroUsize,

    file: PathBuf,
}

async fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).await?;
    Ok(BufReader::new(file).lines())
}

fn filename_from_url(url: &reqwest::Url) -> String {
    let filename = match url
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|filename| {
            if filename.is_empty() {
                None
            } else {
                Some(filename)
            }
        }) {
        None => url.host_str().unwrap(),
        Some(filename) => filename,
    };
    match filename.ends_with(".htm") || filename.ends_with(".html") {
        true => filename.to_string(),
        false => filename.to_owned() + ".html",
    }
}

enum DownloadError {
    RequestError(reqwest::Error),
    FileError(std::io::Error),
}

async fn download_html(link: String) -> Result<(), DownloadError> {
    let response = reqwest::get(link.clone())
        .await
        .map_err(DownloadError::RequestError)?;
    File::create(filename_from_url(response.url()))
        .await
        .map_err(DownloadError::FileError)?
        .write_all(
            response
                .text()
                .await
                .map_err(DownloadError::RequestError)?
                .as_bytes(),
        )
        .await
        .map_err(DownloadError::FileError)
}

fn main() {
    let args = Args::parse();

    let threaded_rt = runtime::Builder::new_multi_thread()
        .worker_threads(args.max_threads.into())
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    threaded_rt.block_on(async {
        let mut file = read_lines(&args.file).await.expect("failed to open file:");

        let mut handlers = Vec::new();
        while let Some(link) = file
            .next_line()
            .await
            .expect("failed to read line with error:")
        {
            handlers.push(threaded_rt.spawn(async move {
                match download_html(link.clone()).await {
                    Ok(_) => (),
                    Err(DownloadError::RequestError(e)) => {
                        println!("request error for \"{}\": {:?}", link, e)
                    }
                    Err(DownloadError::FileError(e)) => {
                        println!("file error for \"{}\": {:?}", link, e)
                    }
                }
            }));
        }
        for handler in handlers {
            handler.await.unwrap();
        }
    });
}
