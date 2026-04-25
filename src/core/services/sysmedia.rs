/* --- loonixtunesv2/src/audio/sysmedia.rs | System Media --- */
/* MPRIS Media Controls via souvlaki */

use qmetaobject::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "linux")]
use souvlaki::{MediaControlEvent, MediaControls, PlatformConfig};

thread_local! {
    static EVENT_RX: std::cell::RefCell<Option<mpsc::Receiver<MediaControlEvent>>> = const { std::cell::RefCell::new(None) };
}

#[derive(QObject, Default)]
#[allow(non_snake_case)]
pub struct SysMediaManager {
    base: qt_base_class!(trait QObject),

    pub title: qt_property!(QString; NOTIFY metadata_changed),
    pub artist: qt_property!(QString; NOTIFY metadata_changed),
    pub album: qt_property!(QString; NOTIFY metadata_changed),
    pub duration_ms: qt_property!(i64; NOTIFY metadata_changed),
    pub position_ms: qt_property!(i64; NOTIFY position_updated),
    pub is_playing: qt_property!(bool; NOTIFY playback_state_changed),

    pub playRequested: qt_signal!(),
    pub pauseRequested: qt_signal!(),
    pub stopRequested: qt_signal!(),
    pub nextRequested: qt_signal!(),
    pub prevRequested: qt_signal!(),
    pub seekPosition: qt_signal!(position_ms: i64),
    pub openUri: qt_signal!(uri: QString),

    metadata_changed: qt_signal!(),
    playback_state_changed: qt_signal!(),
    position_updated: qt_signal!(),

    poll_events: qt_method!(fn(&mut self)),

    #[cfg(target_os = "linux")]
    controls: Option<MediaControls>,
}

impl SysMediaManager {
    pub fn new() -> Self {
        let mut manager = Self::default();

        #[cfg(target_os = "linux")]
        {
            manager.spawn_mpris_listener();
        }

        #[cfg(not(target_os = "linux"))]
        {}

        manager
    }

    #[cfg(target_os = "linux")]
    fn spawn_mpris_listener(&mut self) {
        let (event_tx, event_rx) = mpsc::channel::<MediaControlEvent>();

        EVENT_RX.with(|cell| {
            *cell.borrow_mut() = Some(event_rx);
        });

        thread::spawn(move || {
            let config = PlatformConfig {
                display_name: "Loonix Tunes",
                dbus_name: "loonix-tunes",
                hwnd: None,
            };

            let mut controls = match MediaControls::new(config) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[SysMedia] Failed to create MediaControls: {:?}", e);
                    return;
                }
            };

            let result = controls.attach(move |event| {
                if let Err(e) = event_tx.send(event) {
                    eprintln!("[SysMedia] Failed to send event: {:?}", e);
                }
            });

            match result {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("[SysMedia] Failed to attach MediaControls: {:?}", e);
                    return;
                }
            }

            loop {
                thread::sleep(Duration::from_secs(60));
            }
        });
    }

    #[cfg(not(target_os = "linux"))]
    fn spawn_mpris_listener(&mut self) {}

    pub fn poll_events(&mut self) {
        #[cfg(target_os = "linux")]
        {
            EVENT_RX.with(|cell| {
                if let Some(ref rx) = *cell.borrow() {
                    while let Ok(event) = rx.try_recv() {
                        self.handle_event(event);
                    }
                }
            });
        }
    }

    #[cfg(target_os = "linux")]
    fn handle_event(&mut self, event: MediaControlEvent) {
        use souvlaki::MediaControlEvent;
        match event {
            MediaControlEvent::Play => {
                self.playRequested();
            }
            MediaControlEvent::Pause => {
                self.pauseRequested();
            }
            MediaControlEvent::Toggle => {
                self.playRequested();
            }
            MediaControlEvent::Next => {
                self.nextRequested();
            }
            MediaControlEvent::Previous => {
                self.prevRequested();
            }
            MediaControlEvent::Seek(direction) => {
                let delta_ms = match direction {
                    souvlaki::SeekDirection::Forward => 5000,
                    souvlaki::SeekDirection::Backward => -5000,
                };
                self.seekPosition(delta_ms);
            }
            MediaControlEvent::SetPosition(position) => {
                let pos_ms = position.0.as_millis() as i64;
                self.seekPosition(pos_ms);
            }
            MediaControlEvent::OpenUri(uri) => {
                self.openUri(QString::from(uri.as_str()));
            }
            _ => {}
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn handle_event(&mut self, _event: MediaControlEvent) {}

    pub fn update_metadata(&mut self, title: &str, artist: &str, album: &str, duration_ms: i64) {
        self.title = QString::from(title);
        self.artist = QString::from(artist);
        self.album = QString::from(album);
        self.duration_ms = duration_ms;
        self.metadata_changed();

        #[cfg(target_os = "linux")]
        {
            if let Some(ref mut controls) = self.controls {
                use souvlaki::MediaMetadata;
                let metadata = MediaMetadata {
                    title: Some(title),
                    album: Some(album),
                    artist: Some(artist),
                    cover_url: None,
                    duration: Some(Duration::from_millis(duration_ms as u64)),
                };
                if let Err(e) = controls.set_metadata(metadata) {
                    eprintln!("[SysMedia] Failed to set metadata: {:?}", e);
                }
            }
        }
    }

    pub fn update_playback_state(&mut self, playing: bool) {
        self.is_playing = playing;
        self.playback_state_changed();

        #[cfg(target_os = "linux")]
        {
            println!(
                "[SysMedia] Playback state: {}",
                if playing { "Playing" } else { "Paused" }
            );
        }
    }

    pub fn update_position(&mut self, position_ms: i64) {
        self.position_ms = position_ms;
        self.position_updated();
    }

    pub fn get_title(&self) -> QString {
        self.title.clone()
    }

    pub fn get_artist(&self) -> QString {
        self.artist.clone()
    }

    pub fn get_album(&self) -> QString {
        self.album.clone()
    }

    pub fn get_duration(&self) -> i64 {
        self.duration_ms
    }

    pub fn get_is_playing(&self) -> bool {
        self.is_playing
    }
}
