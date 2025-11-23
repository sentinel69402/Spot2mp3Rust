use anyhow::{Ok, Result};
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::io::Write;
use std::{fs, io};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Semaphore;
use serde_json::Value;

mod utils;
use utils::{clean_query,get_track_path,TrackRecord};

#[derive(Parser,Debug)]
#[command(author,version,about="Download Spotify playlist tracks via yt-dlp (Rust rewrite)")]
struct Args {
    csv: PathBuf,

    #[arg(short,long)]
    all: bool,

    #[arg(short = 'j',long,default_value_t=4)]
    jobs: usize
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let csv_path = if !args.csv.exists() {
        println!("CSV not found. Please enter path to CSV: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        std::path::PathBuf::from(input.trim())
    } else {
        args.csv.clone()
    };

    let tracks = utils::load_playlists(&csv_path)
        .expect("Failed to load playlist CSV");
    process_playlists(tracks, args.all, args.jobs).await?;

    Ok(())
}


async fn process_playlists(tracks: Vec<TrackRecord>, auto_confirm: bool,max_workers: usize) -> Result<()> {
    let mp = Arc::new(MultiProgress::new());
    let sem = Arc::new(Semaphore::new(max_workers));

    let mut handles = Vec::new();

    for t in tracks {
        if t.track_name.is_empty() || t.artist_name.is_empty() {
            continue;
        }

        if !auto_confirm {
            println!("Download '{}' by {}? [y/N]: ", t.track_name, t.artist_name);
            use std::io::{self,Write};
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() != "y" {
                continue;
            }
        }

        let permit = sem.clone().acquire_owned().await.unwrap();
        let mp_clone = mp.clone();
        let record = t.clone();

        let handle = tokio::spawn(async move{
            let pb = mp_clone.add(ProgressBar::new(100));
            pb.set_style(
                ProgressStyle::with_template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {pos}%")
                    .unwrap()
                    .progress_chars("=>-")  
            );
            pb.set_message(format!("{} - {}",record.artist_name,record.track_name));

            let res = download_track(&record.track_name, &record.artist_name, &record.album_name, &pb).await;

            pb.finish_with_message(format!("Done: {} - {}",record.artist_name,record.track_name));

            drop(permit);

            if let Err(e) = res {
                log::error!("Error downloading {} - {}: {}",record.artist_name,record.track_name,e);
            }
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.await;
    }

    Ok(())
}



async fn download_track(track: &str, artist: &str, album: &str, pb: &ProgressBar) -> Result<()> {
   let path = get_track_path(artist, album, track);

   if path.exists() {
    log::info!("Skipping '{}': already downloaded",track);
    pb.set_position(100);
    return  Ok(());
   }

   if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
   }

   let query = clean_query(track, artist);
   let search_url = format!("ytsearch10:{}",query);

   let output = Command::new("yt-dlp")
        .arg(&search_url)
        .arg("--dump-json")
        .arg("--skip-download")
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()
        .await?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("yt-dlp probe failed with: {}",output.status));
    }

    let out = String::from_utf8_lossy(&output.stdout);
    let first_line = out
        .lines()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No results from yt-dlp"))?;

    let v: Value = serde_json::from_str(first_line)?;
    let id = v["id"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse id from yt-dlp output"))?;

    let url = format!("https://youtube.com/watch?v={}",id);
    log::info!("Found: {} -> {}",track,url);

    let outtmpl = path.with_extension("");
    let outtmpl_str = outtmpl.to_string_lossy();

    let mut cmd = Command::new("yt-dlp");
    cmd.arg("-f").arg("bestaudio/best")
        .arg("--extract-audio")
        .arg("--audio-format").arg("mp3")
        .arg("--audio-quality").arg("192K")
        .arg("-o").arg(outtmpl_str.as_ref())
        .arg(&url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn()?;
    let mut progress = 0u64;

    loop {
        if let Some(status) = child.try_wait()? {
            if status.success() {
                pb.set_position(100);
                return Ok(());
            } else {
                let code = status.code().unwrap_or(-1);
                return Err(anyhow::anyhow!("yt-dlp failed with exit code {}",code));
            }
        }

        if progress < 90 {
            progress += 2;
            pb.set_position(progress as u64);
        }

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }
}