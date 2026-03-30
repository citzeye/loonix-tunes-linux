/* --- LOONIX-TUNES src/ui/playerbridge.rs --- */

use qmetaobject::*;

/// PlayerBridge - bridge between Rust audio engine and QML UI
/// This prevents UI from directly accessing the audio engine
#[derive(QObject, Default)]
pub struct PlayerBridge {
    base: qt_base_class!(trait QObject),

    // Properties
    pub position: qt_property!(i64; NOTIFY position_changed),
    pub position_changed: qt_signal!(),

    pub duration: qt_property!(i64; NOTIFY duration_changed),
    pub duration_changed: qt_signal!(),

    pub is_playing: qt_property!(bool; NOTIFY playing_state_changed),
    pub playing_state_changed: qt_signal!(),

    pub volume: qt_property!(f32; NOTIFY volume_changed),
    pub volume_changed: qt_signal!(),

    // Methods
    pub play: qt_method!(fn(&mut self)),
    pub pause: qt_method!(fn(&mut self)),
    pub stop: qt_method!(fn(&mut self)),
    pub seek: qt_method!(fn(&mut self, position_ms: i64)),
    pub set_volume: qt_method!(fn(&mut self, volume: f32)),
    pub set_position: qt_method!(fn(&mut self, position_ms: i64)),
    pub set_duration: qt_method!(fn(&mut self, duration_ms: i64)),
    pub set_playing: qt_method!(fn(&mut self, playing: bool)),

    // Internal state
    internal_position_ms: i64,
    internal_duration_ms: i64,
    internal_playing: bool,
    internal_volume: f32,
}

impl PlayerBridge {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn play(&mut self) {
        self.internal_playing = true;
        self.is_playing = true;
        self.playing_state_changed();
    }

    pub fn pause(&mut self) {
        self.internal_playing = false;
        self.is_playing = false;
        self.playing_state_changed();
    }

    pub fn stop(&mut self) {
        self.internal_playing = false;
        self.internal_position_ms = 0;
        self.is_playing = false;
        self.position = 0;
        self.playing_state_changed();
        self.position_changed();
    }

    pub fn seek(&mut self, position_ms: i64) {
        self.internal_position_ms = position_ms;
        self.position = position_ms;
        self.position_changed();
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.internal_volume = volume;
        self.volume = volume;
        self.volume_changed();
    }

    pub fn set_position(&mut self, position_ms: i64) {
        self.internal_position_ms = position_ms;
        self.position = position_ms;
        self.position_changed();
    }

    pub fn set_duration(&mut self, duration_ms: i64) {
        self.internal_duration_ms = duration_ms;
        self.duration = duration_ms;
        self.duration_changed();
    }

    pub fn set_playing(&mut self, playing: bool) {
        self.internal_playing = playing;
        self.is_playing = playing;
        self.playing_state_changed();
    }
}
