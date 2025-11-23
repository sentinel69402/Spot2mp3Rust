use regex::Regex;
use serde::Deserialize;
use std::path::{Path,PathBuf};
use csv::ReaderBuilder;
use anyhow::Result;
use std::result::Result::Ok;

pub const BASE_DIR: &str = "playlists";

#[derive(Debug,Clone,Deserialize)]
pub struct TrackRecord {
    #[serde(rename = "Track Name")]
    pub track_name: String,

    #[serde(rename = "Artist Name(s)")]
    pub artist_name: String,

    #[serde(rename = "Album Name")]
    pub album_name: String
}

pub fn load_playlists(csv_path: &PathBuf) -> Result<Vec<TrackRecord>> {
    let mut rdr = ReaderBuilder::new().from_path(csv_path)?;
    let mut records = Vec::new();

    for result in rdr.deserialize::<TrackRecord>() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => log::warn!("Skipping malformed row: {}", e),
        }
    }

    Ok(records)
}
pub fn clean_query(track: &str,artist: &str) -> String {
    let re = Regex::new(r#"[^\w\s]"#).unwrap();
    let track_clean = re.replace_all(track, "").trim().to_string();
    let artist_clean = re.replace_all(artist, "").trim().to_string();
    format!("{} - {} official audio",track_clean,artist_clean)
}
pub fn get_track_path(_artist: &str, album: &str, track: &str) -> PathBuf {
    let album_dir = Path::new(BASE_DIR).join(album);
    if let Err(e) = std::fs::create_dir_all(&album_dir) {
        log::warn!("Failed to create album dir {:?}: {}", album_dir, e);
    }
    let re_forbidden = Regex::new(r#"[\\/:"*?<>|]+"#).unwrap();
    let safe_track = re_forbidden.replace_all(track, "").to_string();
    album_dir.join(format!("{}.mp3", safe_track))
}