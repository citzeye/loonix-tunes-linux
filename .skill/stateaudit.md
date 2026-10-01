URGENT CODE AUDIT: STATE PERSISTENCE & UI SYNCHRONIZATION

CONTEXT: > We have just unified the configuration into a single Arc<Mutex<AppConfig>> in src/audio/config.rs. Now, we need to audit every function in the backend that handles "Saving" or "Updating" state to ensure they follow the new architecture.

TASK:
Check every setter function and "Save" method in the following files:

src/ui/theme.rs (ThemeManager)

src/ui/core.rs (MusicModel)

src/audio/config.rs (AppConfig)

AUDIT CRITERIA (The "Smart Save" Rules):

Rule 1: Use Shared Config. > Ensure NO function uses local variables or confy. They must lock the shared Arc<Mutex<AppConfig>>, update the value, and then call cfg.save().

Rule 2: Immediate UI Refresh (Smart Apply).
In theme.rs, when set_custom_theme_colors or names are called, check if the index being edited is the Active Theme. If YES, it MUST trigger self.set_theme() immediately so the UI reflects changes without a restart.

Rule 3: Audio State Persistence.
In core.rs, ensure functions like set_volume, set_balance, and EQ fader updates are correctly writing to the shared AppConfig. Check if the AppConfig implementation of save() is actually being called after these updates.

Rule 4: Metadata & Playlists.
Ensure that adding custom_folders or updating favorites follows the same SSOT pattern.

Rule 5: Signal Notification.
Every setter must emit its corresponding qt_signal (e.g., colormap_changed, music_state_changed, etc.) after saving, so QML stays in sync.

REQUIRED OUTPUT:
Identify any functions that are still "hardcoded", missing a .save() call, or failing to notify the UI. Provide the corrected code for those specific functions.