/* --- LOONIX-TUNES src/core/playback.rs --- */

#![allow(non_snake_case)]

use crate::audio::dsp::abrepeat::ABRepeat;
use crate::audio::engine::{AudioState, FfmpegEngine, MusicItem, PlaybackState};
use qmetaobject::prelude::*;
use qmetaobject::QString;
use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};

pub struct PlaybackController {
    pub(crate) ffmpeg: Arc<Mutex<FfmpegEngine>>,
    pub(crate) audio: Arc<Mutex<AudioState>>,

    pub current_title: QString,
    pub current_index: i32,
    pub position: i32,
    pub duration: i32,
    pub volume: f64,
    pub muted: bool,

    pub shuffle_active: bool,
    pub loop_active: bool,
    pub shuffle_queue: Vec<i32>,
    pub queue_index: usize,

    pub abrepeat: ABRepeat,
    pub tick_counter: u32,
}

impl Default for PlaybackController {
    fn default() -> Self {
        Self {
            ffmpeg: Arc::new(Mutex::new(FfmpegEngine::new())),
            audio: Arc::new(Mutex::new(AudioState::default())),
            current_title: QString::default(),
            current_index: -1,
            position: 0,
            duration: 0,
            volume: 1.0,
            muted: false,
            shuffle_active: false,
            loop_active: false,
            shuffle_queue: Vec::new(),
            queue_index: 0,
            abrepeat: ABRepeat::default(),
            tick_counter: 0,
        }
    }
}

impl PlaybackController {
    pub fn new(ffmpeg: Arc<Mutex<FfmpegEngine>>, audio: Arc<Mutex<AudioState>>) -> Self {
        Self {
            ffmpeg,
            audio,
            current_title: QString::default(),
            current_index: -1,
            position: 0,
            duration: 0,
            volume: 1.0,
            muted: false,
            shuffle_active: false,
            loop_active: false,
            shuffle_queue: Vec::new(),
            queue_index: 0,
            abrepeat: ABRepeat::default(),
            tick_counter: 0,
        }
    }
}

impl PlaybackController {
    pub fn play_at(&mut self, item: &MusicItem) {
        self.current_title = QString::from(item.name.clone());
        self.position = 0;
        self.duration = 0;

        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.load(&item.path);
            ff.play();
            let dur = (ff.get_duration() * 1000.0) as i32;
            if dur > 0 {
                self.duration = dur;
            }
        }
    }

    pub fn stop(&mut self) {
        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.stop();
        }
    }

    pub fn pause(&mut self) {
        if let Ok(mut ff) = self.ffmpeg.lock() {
            if matches!(ff.get_playback_state(), PlaybackState::Playing) {
                ff.pause();
            }
        }
    }

    pub fn resume(&mut self) {
        if let Ok(mut ff) = self.ffmpeg.lock() {
            if matches!(ff.get_playback_state(), PlaybackState::Paused) {
                ff.resume();
            }
        }
    }

    pub fn toggle(&mut self) {
        if let Ok(mut ff) = self.ffmpeg.lock() {
            match ff.get_playback_state() {
                PlaybackState::Playing => {
                    ff.pause();
                }
                PlaybackState::Paused => {
                    ff.resume();
                }
                PlaybackState::Stopped | PlaybackState::Loading | PlaybackState::Priming => {
                    if let Some(ref path) = ff.get_current_path() {
                        ff.load(path);
                        ff.play();
                    }
                }
            }
        }
    }

    pub fn get_playback_state(&self) -> PlaybackState {
        if let Ok(ff) = self.ffmpeg.lock() {
            ff.get_playback_state()
        } else {
            PlaybackState::Stopped
        }
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.get_playback_state(), PlaybackState::Playing)
    }

    pub fn seek_to(&mut self, pos: i32) {
        self.position = pos;
        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.seek(pos as f64 / 1000.0);
        }
    }

    pub fn set_volume(&mut self, vol: f64) {
        self.volume = vol;
        let actual = if self.muted { 0.0_f32 } else { vol as f32 };
        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.set_volume(actual);
        }
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        let vol = if self.muted {
            0.0_f32
        } else {
            self.volume as f32
        };
        if let Ok(mut ff) = self.ffmpeg.lock() {
            ff.set_volume(vol);
        }
    }

    pub fn toggle_shuffle(&mut self, display_list: &[MusicItem]) {
        self.shuffle_active = !self.shuffle_active;
        self.queue_index = 0;

        if self.shuffle_active && !display_list.is_empty() {
            self.shuffle_queue.clear();
            let total = display_list.len();
            let mut indices: Vec<i32> = (0..total as i32).collect();
            indices.shuffle(&mut rand::rng());
            self.shuffle_queue = indices;
        }
    }

    pub fn toggle_repeat(&mut self) {
        self.loop_active = !self.loop_active;
    }

    pub fn play_next(
        &mut self,
        display_list: &[MusicItem],
        current_idx: i32,
    ) -> Option<(usize, MusicItem)> {
        if display_list.is_empty() {
            return None;
        }

        let total = display_list.len();
        let next_idx = if self.shuffle_active {
            self.queue_index += 1;
            if self.queue_index >= self.shuffle_queue.len() {
                if self.loop_active {
                    self.queue_index = 0;
                } else {
                    return None;
                }
            }
            self.shuffle_queue[self.queue_index] as usize
        } else {
            let next = current_idx + 1;
            if next >= total as i32 {
                if self.loop_active {
                    0
                } else {
                    return None;
                }
            } else {
                next as usize
            }
        };

        Some((next_idx, display_list[next_idx].clone()))
    }

    pub fn play_prev(
        &mut self,
        display_list: &[MusicItem],
        current_idx: i32,
    ) -> Option<(usize, MusicItem)> {
        if display_list.is_empty() {
            return None;
        }

        let total = display_list.len();
        let prev_idx = if self.shuffle_active {
            if self.queue_index > 0 {
                self.queue_index -= 1;
            }
            self.shuffle_queue[self.queue_index] as usize
        } else {
            let prev = current_idx - 1;
            if prev < 0 {
                if self.loop_active {
                    total - 1
                } else {
                    return None;
                }
            } else {
                prev as usize
            }
        };

        Some((prev_idx, display_list[prev_idx].clone()))
    }

    pub fn toggle_abrepeat(&mut self) {
        let current_position = self.position as f64 / 1000.0;
        self.abrepeat.toggle(current_position);
    }
}
