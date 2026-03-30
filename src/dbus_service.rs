/* --- LOONIX-TUNES src/dbus_service.rs --- */
/* D-Bus service is currently disabled due to Send/Sync issues with MusicModel */

pub fn init_dbus() -> Result<(), String> {
    println!("[DBUS] Service disabled - needs redesign for Send/Sync compatibility");
    Ok(())
}

pub async fn start_dbus_service<T>(_music_model: std::sync::Arc<std::sync::Mutex<T>>) -> Result<(), String> {
    Ok(())
}
