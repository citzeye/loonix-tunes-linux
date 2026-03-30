/* --- LOONIX-TUNES src/ui/updater.rs --- */

use std::sync::mpsc::Receiver;

pub struct UpdateChecker {
    pub update_rx: Option<Receiver<String>>,
    pub update_status: String,
    pub update_available: bool,
}

impl Default for UpdateChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdateChecker {
    pub fn new() -> Self {
        Self {
            update_rx: None,
            update_status: String::new(),
            update_available: false,
        }
    }

    pub fn check_for_updates(&mut self) {
        self.update_status = "Checking for updates...".to_string();
        self.update_available = false;

        let (tx, rx) = std::sync::mpsc::channel();
        self.update_rx = Some(rx);

        std::thread::spawn(move || {
            let client = match reqwest::blocking::Client::builder()
                .user_agent("loonix-tunes")
                .timeout(std::time::Duration::from_secs(10))
                .build()
            {
                Ok(c) => c,
                Err(_) => {
                    let _ = tx.send("error".to_string());
                    return;
                }
            };

            let url = "https://api.github.com/repos/citz/loonix-tunes/releases/latest";
            match client.get(url).send() {
                Ok(res) => match res.json::<serde_json::Value>() {
                    Ok(json) => {
                        let latest = json["tag_name"].as_str().unwrap_or("").replace('v', "");
                        let _ = tx.send(latest);
                    }
                    Err(_) => {
                        let _ = tx.send("error".to_string());
                    }
                },
                Err(_) => {
                    let _ = tx.send("error".to_string());
                }
            }
        });
    }

    pub fn poll_result(&mut self) {
        if let Some(ref rx) = self.update_rx {
            if let Ok(latest) = rx.try_recv() {
                self.update_rx = None;
                let current = env!("CARGO_PKG_VERSION");
                if latest == "error" {
                    self.update_status = "Failed to reach GitHub".to_string();
                    self.update_available = false;
                } else if latest > current.to_string() {
                    self.update_status = format!("New version available: v{}", latest);
                    self.update_available = true;
                } else {
                    self.update_status = "You are up to date!".to_string();
                    self.update_available = false;
                }
            }
        }
    }
}
