# Spotify2Media

<div align="center">

![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Status](https://img.shields.io/badge/status-active-success.svg)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey.svg)

A fast, console-based Rust tool to download songs from a Spotify playlist via YouTube.

**Optimized for speed** with parallel async downloads, batch searches, and automatic query cleaning.

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Configuration](#configuration)

</div>

---

> **⚠️ Disclaimer:** This tool is for educational purposes only. Respect copyright laws when using downloaded content.

---

## Features

- Works with **Spotify playlist CSVs** (exportable via [Exportify](https://exportify.net/))
- **Console-based** interface, cancelable at any time with `Ctrl+C`
- **Parallel async downloads** using Tokio for blazing-fast performance
- **Batch YouTube searches** to reduce HTTP requests
- **Query cleaning** to improve search results (removes special characters, adds "official audio")
- Converts audio to **MP3** using `yt-dlp` + `ffmpeg`
- Organizes downloads by **Album** folder structure
- **Smart skip** - avoids re-downloading existing tracks
- **Progress bars** for each download with real-time status

---

## Installation

### Prerequisites

![Rust](https://img.shields.io/badge/Rust-1.70+-CE422B?logo=rust&logoColor=white)
![FFmpeg](https://img.shields.io/badge/FFmpeg-required-007808?logo=ffmpeg&logoColor=white)
![yt-dlp](https://img.shields.io/badge/yt--dlp-required-FF0000?logo=youtube&logoColor=white)

- **Rust 1.70+** - [Install here](https://rustup.rs/)
- **ffmpeg** - [Download here](https://ffmpeg.org/download.html)
- **yt-dlp** - [Install guide](https://github.com/yt-dlp/yt-dlp#installation)

### Install Dependencies

**Install yt-dlp:**
```bash
# Using pip
pip install yt-dlp

# Or using your package manager
# macOS: brew install yt-dlp
# Linux: sudo apt install yt-dlp
```

**Install FFmpeg:**
```bash
# Linux
sudo apt install ffmpeg

# macOS
brew install ffmpeg

# Windows: Download from ffmpeg.org and add to PATH
```

### Build from Source

```bash
git clone https://github.com/sentinel69402/Spot2mp3.git
cd Spot2mp3
cargo build --release
```

The compiled binary will be in `target/release/Spot2mp3` (or `Spot2mp3.exe` on Windows).

---

## Usage

### 1. Export Your Spotify Playlist

Visit [Exportify](https://exportify.net/) and export your playlist as a CSV file.

### 2. Run the Downloader

**Download all tracks automatically:**
```bash
./target/release/Spot2mp3.exe playlist.csv --all
```

**Download with confirmation prompts:**
```bash
./target/release/Spot2mp3.exe playlist.csv
```

**Specify number of parallel downloads:**
```bash
./target/release/Spot2mp3.exe playlist.csv --all -j 8
```

### 3. Find Your Music

Downloaded tracks are organized in:
```
playlists/
├── Album Name/
│   ├── Track Name 1.mp3
│   └── Track Name 2.mp3
```

---

## Configuration

### Command Line Arguments

| Argument | Short | Description | Default |
|----------|-------|-------------|---------|
| `csv` | - | Path to Spotify playlist CSV file (required) | - |
| `--all` | `-a` | Download all tracks without prompting | false |
| `--jobs` | `-j` | Number of parallel downloads | 4 |

### Examples

```bash
# Download with prompts
./target/release/Spot2mp3.exe my_playlist.csv

# Download all automatically with 8 parallel workers
./target/release/Spot2mp3.exe my_playlist.csv --all -j 8

# If CSV is not in current directory, you'll be prompted to enter the path
./target/release/Spot2mp3.exe
```

### Adjust Download Settings

The number of parallel downloads can be adjusted via the `-j` flag:

```bash
# Conservative (good for slower connections)
Spot2mp3.exe playlist.csv --all -j 2

# Balanced (default)
Spot2mp3.exe playlist.csv --all -j 4

# Aggressive (requires good CPU and internet)
Spot2mp3.exe playlist.csv --all -j 12
```

**Search settings** are configured in `utils.rs`:
- YouTube search returns top 10 results (`ytsearch10`)
- Query cleaning removes special characters and adds "official audio"

---

## Dependencies

This project uses the following Rust crates:

- **tokio** - Async runtime for parallel downloads
- **clap** - Command-line argument parsing
- **indicatif** - Progress bars and status display
- **serde** / **serde_json** - CSV parsing and JSON handling
- **csv** - CSV file reading
- **regex** - Query cleaning and path sanitization
- **anyhow** - Error handling
- **env_logger** - Logging support

---

## Troubleshooting

**Issue: "yt-dlp: command not found"**
- Install yt-dlp using pip: `pip install yt-dlp`
- Or install via package manager (see Installation section)
- Ensure yt-dlp is in your system PATH

**Issue: "FFmpeg not found"**
- Install FFmpeg and ensure it's in your system PATH
- Windows: Download from [ffmpeg.org](https://ffmpeg.org/) and add to PATH
- Linux: `sudo apt install ffmpeg`
- macOS: `brew install ffmpeg`

**Issue: Downloads are slow**
- Increase parallel jobs with `-j` flag (e.g., `-j 8` or `-j 12`)
- Check your internet connection
- Note: Too many parallel jobs may cause rate limiting

**Issue: Wrong songs downloaded**
- The tool searches YouTube automatically
- Results depend on YouTube's search algorithm
- Query cleaning adds "official audio" to improve accuracy
- Consider manually verifying important tracks

**Issue: Build fails**
- Ensure you have Rust 1.70+ installed: `rustc --version`
- Update Rust: `rustup update`
- Clean build cache: `cargo clean && cargo build --release`

**Issue: Progress bars not displaying correctly**
- This may happen on some terminals
- Try setting `RUST_LOG=info` for text-based logging
- Run with: `RUST_LOG=info ./target/release/spotify2media playlist.csv --all`

---

## License

![License](https://img.shields.io/badge/license-MIT-green.svg)

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## Acknowledgments

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - YouTube downloader
- [Exportify](https://exportify.net/) - Spotify playlist exporter
- [FFmpeg](https://ffmpeg.org/) - Audio processing
- [Tokio](https://tokio.rs/) - Async runtime for Rust

---

## Why Rust?

This is a complete rewrite of the original Python version with several advantages:

- **Faster execution** - Native compiled code with zero-cost abstractions
- **Better concurrency** - Tokio's async runtime handles parallel downloads efficiently
- **Memory safety** - Rust's ownership system prevents common bugs
- **Single binary** - No Python interpreter or pip dependencies needed
- **Cross-platform** - Compile once, run anywhere

---

<div align="center">

Made with ❤️ for music lovers

**Star this repo if you find it useful!**

</div>