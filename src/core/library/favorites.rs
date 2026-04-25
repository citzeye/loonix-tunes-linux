use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct FavoriteItem {
    pub path: String,
    pub name: String,
}

pub struct FavoritesManager {
    favorites: Arc<Mutex<Vec<FavoriteItem>>>,
}

impl FavoritesManager {
    pub fn new() -> Self {
        Self {
            favorites: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, path: String, name: String) -> bool {
        let mut fav = self.favorites.lock().unwrap();
        if !fav.iter().any(|item| item.path == path) {
            fav.push(FavoriteItem { path, name });
            true
        } else {
            false
        }
    }

    pub fn remove(&self, path: &str) -> bool {
        let mut fav = self.favorites.lock().unwrap();
        let len_before = fav.len();
        fav.retain(|item| item.path != path);
        fav.len() < len_before
    }

    pub fn toggle(&self, path: String, name: String) -> bool {
        let mut fav = self.favorites.lock().unwrap();
        if let Some(pos) = fav.iter().position(|item| item.path == path) {
            fav.remove(pos);
            false
        } else {
            fav.push(FavoriteItem { path, name });
            true
        }
    }

    pub fn contains(&self, path: &str) -> bool {
        self.favorites.lock().unwrap().iter().any(|item| item.path == path)
    }

    pub fn get_all(&self) -> Vec<FavoriteItem> {
        self.favorites.lock().unwrap().clone()
    }

    pub fn set_all(&self, items: Vec<FavoriteItem>) {
        *self.favorites.lock().unwrap() = items;
    }

    pub fn clear(&self) {
        self.favorites.lock().unwrap().clear();
    }
}

impl Default for FavoritesManager {
    fn default() -> Self {
        Self::new()
    }
}
pub type Favorites = FavoritesManager;
