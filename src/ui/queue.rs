/* --- LOONIX-TUNES src/ui/queuecontroller.rs | Queue Controller --- */

use crate::audio::engine::MusicItem;
use qmetaobject::prelude::*;
use qmetaobject::{QString, QVariant, QVariantMap};

#[derive(Default, Clone)]
pub struct QueueController {
    pub queue: Vec<MusicItem>,
}

impl QueueController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, path: String, name: String) {
        self.queue.push(MusicItem {
            name,
            path,
            is_folder: false,
            parent_folder: None,
        });
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.queue.len() {
            self.queue.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn pop_front(&mut self) -> Option<MusicItem> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn get_item(&self, index: usize) -> Option<&MusicItem> {
        self.queue.get(index)
    }

    pub fn get_item_map(&self, index: i32) -> QVariantMap {
        let idx = index as usize;
        if idx >= self.queue.len() {
            return QVariantMap::default();
        }
        let item = &self.queue[idx];
        let mut map = QVariantMap::default();
        map.insert("name".into(), QString::from(item.name.as_str()).into());
        map.insert("path".into(), QString::from(item.path.as_str()).into());
        map
    }

    pub fn get_all(&self) -> Vec<MusicItem> {
        self.queue.clone()
    }

    pub fn set_all(&mut self, items: Vec<MusicItem>) {
        self.queue = items;
    }
}
