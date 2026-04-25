/* --- loonixtunesv2/src/core/library/metadata.rs | Metadata --- */

use ffmpeg::format::input;
use ffmpeg::media::Type;
use ffmpeg_next as ffmpeg;

#[derive(Clone, Default)]
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: String,
    pub genre: String,
    pub duration_sec: f64,
    pub bitrate_kbps: i64,
    pub sample_rate: i64,
    pub channels: i64,
    pub codec: String,
    pub file_path: String,
    pub file_size_bytes: u64,
}

pub fn read_track_metadata(path: &str) -> TrackMetadata {
    let mut meta = TrackMetadata::default();
    meta.file_path = path.to_string();

    if let Ok(file_meta) = std::fs::metadata(path) {
        meta.file_size_bytes = file_meta.len();
    }

    if let Err(e) = ffmpeg::init() {
        eprintln!("metadata: ffmpeg init failed: {}", e);
        return meta;
    }

    let ictx = match input(path) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("metadata: failed to open {}: {}", path, e);
            return meta;
        }
    };

    let duration_us = ictx.duration();
    meta.duration_sec = if duration_us > 0 {
        duration_us as f64 / 1_000_000.0
    } else {
        0.0
    };

    let format_dict = ictx.metadata();
    for (key, value) in format_dict.iter() {
        match key {
            "title" | "TLAN" => {
                if meta.title.is_empty() {
                    meta.title = value.to_string();
                }
            }
            "artist" | "TPE1" | "album_artist" | "TPE2" => {
                if meta.artist.is_empty() {
                    meta.artist = value.to_string();
                }
            }
            "album" | "TALB" => {
                if meta.album.is_empty() {
                    meta.album = value.to_string();
                }
            }
            "date" | "year" | "TYER" | "TDRC" => {
                if meta.year.is_empty() {
                    meta.year = value.to_string();
                }
            }
            "genre" | "TCON" => {
                if meta.genre.is_empty() {
                    meta.genre = value.to_string();
                }
            }
            _ => {}
        }
    }

    if meta.title.is_empty() {
        meta.title = std::path::Path::new(path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
    }

    // FALLBACK ARTIST: if empty, use parent folder name
    if meta.artist.is_empty() {
        if let Some(parent) = std::path::Path::new(path).parent() {
            if let Some(folder) = parent.file_name() {
                meta.artist = folder.to_string_lossy().into_owned();
            }
        }
    }

    if let Some(stream) = ictx.streams().best(Type::Audio) {
        let params = stream.parameters();
        meta.codec = params.id().name().to_string();

        // Get sample rate and channels from decoder
        if let Ok(ctx) = ffmpeg::codec::context::Context::from_parameters(params.clone()) {
            if let Ok(decoder) = ctx.decoder().audio() {
                meta.sample_rate = decoder.rate() as i64;
                meta.channels = decoder.channels() as i64;
            }
        }

        let stream_dict = stream.metadata();
        for (key, value) in stream_dict.iter() {
            match key {
                "title" | "TLAN" => {
                    if !value.is_empty() {
                        meta.title = value.to_string();
                    }
                }
                "artist" | "TPE1" => {
                    if !value.is_empty() {
                        meta.artist = value.to_string();
                    }
                }
                "album" | "TALB" => {
                    if !value.is_empty() {
                        meta.album = value.to_string();
                    }
                }
                "date" | "year" => {
                    if !value.is_empty() && meta.year.is_empty() {
                        meta.year = value.to_string();
                    }
                }
                "genre" | "TCON" => {
                    if !value.is_empty() {
                        meta.genre = value.to_string();
                    }
                }
                _ => {}
            }
        }
    }

    // Compute bitrate from file size and duration (most reliable for file info)
    if meta.file_size_bytes > 0 && meta.duration_sec > 0.0 {
        meta.bitrate_kbps =
            ((meta.file_size_bytes as f64 * 8.0) / meta.duration_sec / 1000.0) as i64;
    }

    meta
}
