/* --- LOONIX-TUNES src/audio/vsthost.rs --- */

use parking_lot::RwLock;
use std::collections::HashMap;
use std::fmt;
use std::os::raw::c_void;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// WAJIB import IUnknown biar query_interface jalan
use vst3_sys::base::{IPluginFactory, IPluginFactory2, IUnknown, PClassInfo2};
use vst3_sys::sys::GUID;
use vst3_sys::vst::{
    AudioBusBuffers, IAudioProcessor, IComponent, IEditController, ProcessData, ProcessSetup,
};

// Helper buat string i8 (ASCII/UTF-8 array dari C)
fn i8_array_to_string(chars: &[i8]) -> String {
    let mut u8_chars = Vec::new();
    for &c in chars {
        if c == 0 {
            break;
        }
        u8_chars.push(c as u8);
    }
    String::from_utf8_lossy(&u8_chars).to_string()
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub vendor: String,
    pub category: String,
    pub uid: [u8; 16],
}

#[derive(Debug, Clone)]
pub struct ScannedPlugin {
    pub info: PluginInfo,
    pub path: String,
}

pub struct Vst3Plugin {
    _library: Arc<libloading::Library>,
    component: vst3_sys::VstPtr<dyn IComponent>,
    pub audio_processor: Option<vst3_sys::VstPtr<dyn IAudioProcessor>>,
    controller: Option<vst3_sys::VstPtr<dyn IEditController>>,
    info: PluginInfo,
    pub path: String,
    sample_rate: f64,
    block_size: i32,
    parameters: Arc<RwLock<HashMap<i32, f32>>>,
}

unsafe impl Send for Vst3Plugin {}
unsafe impl Sync for Vst3Plugin {}

impl Vst3Plugin {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Vst3Error> {
        let path = path.as_ref();

        let library = unsafe {
            libloading::Library::new(path.as_os_str()).map_err(|_| Vst3Error::LibraryLoadFailed)?
        };
        let library = Arc::new(library);

        let factory_fn: libloading::Symbol<unsafe extern "system" fn() -> *mut c_void> = unsafe {
            library
                .get(b"GetPluginFactory\0")
                .map_err(|_| Vst3Error::FactoryNotFound)?
        };

        let factory_ptr = unsafe { factory_fn() };
        if factory_ptr.is_null() {
            return Err(Vst3Error::FactoryNotFound);
        }

        let factory: vst3_sys::VstPtr<dyn IPluginFactory> =
            unsafe { std::mem::transmute(factory_ptr) };

        let mut info = PluginInfo {
            name: String::new(),
            vendor: String::new(),
            category: String::new(),
            uid: [0; 16],
        };

        let mut class_info: PClassInfo2 = unsafe { std::mem::zeroed() };
        let factory2: vst3_sys::VstPtr<dyn IPluginFactory2> =
            unsafe { std::mem::transmute(factory_ptr) };

        let result = unsafe { factory2.get_class_info2(0, &mut class_info) };
        if result != 0 {
            return Err(Vst3Error::Vst3Error(result));
        }

        // Fix: Use i8 array helper
        info.name = i8_array_to_string(&class_info.name);
        info.vendor = i8_array_to_string(&class_info.vendor);
        info.category = i8_array_to_string(&class_info.category);

        // Fix: Convert GUID struct to [u8; 16]
        let mut uid = [0u8; 16];
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class_info.cid as *const _ as *const u8,
                uid.as_mut_ptr(),
                16,
            );
        }
        info.uid = uid;

        let mut component_ptr: *mut c_void = std::ptr::null_mut();
        // VST3 IID constants (from SDK documentation)
        // IComponent: E831FF31-F2D5-4301-928E-BBEE25697802
        // IAudioProcessor: 42043F99-B7DA-453C-A569-E79D9AAEC33D
        // IEditController: DCD7BBE3-7742-448D-A874-AACC979C759E
        let icomp_iid = GUID {
            data: [
                0xE8, 0x31, 0xFF, 0x31, 0xF2, 0xD5, 0x43, 0x01, 0x92, 0x8E, 0xBB, 0xEE, 0x25, 0x69,
                0x78, 0x02,
            ],
        };
        let result =
            unsafe { factory.create_instance(&class_info.cid, &icomp_iid, &mut component_ptr) };
        if result != 0 || component_ptr.is_null() {
            return Err(Vst3Error::InstantiationFailed);
        }
        let component: vst3_sys::VstPtr<dyn IComponent> =
            unsafe { std::mem::transmute(component_ptr) };

        let mut ap_ptr: *mut c_void = std::ptr::null_mut();
        let iap_iid = GUID {
            data: [
                0x42, 0x04, 0x3F, 0x99, 0xB7, 0xDA, 0x45, 0x3C, 0xA5, 0x69, 0xE7, 0x9D, 0x9A, 0xAE,
                0xC3, 0x3D,
            ],
        };
        let result = unsafe { component.query_interface(&iap_iid, &mut ap_ptr) };
        let audio_processor = if result == 0 && !ap_ptr.is_null() {
            Some(unsafe { std::mem::transmute(ap_ptr) })
        } else {
            None
        };

        let mut ctrl_ptr: *mut c_void = std::ptr::null_mut();
        let ictrl_iid = GUID {
            data: [
                0xDC, 0xD7, 0xBB, 0xE3, 0x77, 0x42, 0x44, 0x8D, 0xA8, 0x74, 0xAA, 0xCC, 0x97, 0x9C,
                0x75, 0x9E,
            ],
        };
        let result = unsafe { factory.create_instance(&class_info.cid, &ictrl_iid, &mut ctrl_ptr) };
        let controller = if result == 0 && !ctrl_ptr.is_null() {
            Some(unsafe { std::mem::transmute(ctrl_ptr) })
        } else {
            None
        };

        let mut plugin = Self {
            _library: library,
            component,
            audio_processor,
            controller,
            info,
            path: path.to_string_lossy().into_owned(),
            sample_rate: 44100.0,
            block_size: 512,
            parameters: Arc::new(RwLock::new(HashMap::new())),
        };

        plugin.set_sample_rate(44100.0)?;
        Ok(plugin)
    }

    pub fn set_sample_rate(&mut self, rate: f64) -> Result<(), Vst3Error> {
        self.sample_rate = rate;
        if let Some(ref ap) = self.audio_processor {
            let mut setup: ProcessSetup = unsafe { std::mem::zeroed() };
            setup.process_mode = 0;
            setup.symbolic_sample_size = 0;
            setup.max_samples_per_block = self.block_size;
            setup.sample_rate = rate;

            let result = unsafe { ap.setup_processing(&setup) };
            if result != 0 {
                return Err(Vst3Error::Vst3Error(result));
            }
        }
        Ok(())
    }

    pub fn process(
        &mut self,
        inputs: &[&[f32]],
        outputs: &mut [&mut [f32]],
    ) -> Result<(), Vst3Error> {
        if let Some(ref ap) = self.audio_processor {
            let num_samples = inputs.first().map(|s| s.len()).unwrap_or(0);

            let mut process_data: ProcessData = unsafe { std::mem::zeroed() };
            process_data.process_mode = 0;
            process_data.symbolic_sample_size = 0;
            process_data.num_samples = num_samples as i32;
            process_data.num_inputs = if inputs.is_empty() {
                0
            } else {
                inputs.len() as i32
            };
            process_data.num_outputs = if outputs.is_empty() {
                0
            } else {
                outputs.len() as i32
            };

            if !inputs.is_empty() {
                let mut input_bus: AudioBusBuffers = unsafe { std::mem::zeroed() };
                input_bus.num_channels = inputs.len() as i32;
                input_bus.silence_flags = 0;

                let buffer_ptrs: Vec<*mut c_void> =
                    inputs.iter().map(|ch| ch.as_ptr() as *mut c_void).collect();
                input_bus.buffers = buffer_ptrs.as_ptr() as *mut *mut c_void;
                process_data.inputs = &mut input_bus;
            }

            if !outputs.is_empty() {
                let mut output_bus: AudioBusBuffers = unsafe { std::mem::zeroed() };
                output_bus.num_channels = outputs.len() as i32;
                output_bus.silence_flags = 0;

                let buffer_ptrs: Vec<*mut c_void> = outputs
                    .iter_mut()
                    .map(|ch| ch.as_mut_ptr() as *mut c_void)
                    .collect();
                output_bus.buffers = buffer_ptrs.as_ptr() as *mut *mut c_void;
                process_data.outputs = &mut output_bus;
            }

            let result = unsafe { ap.process(&mut process_data) };
            if result != 0 {
                return Err(Vst3Error::Vst3Error(result));
            }
        }
        Ok(())
    }

    pub fn info(&self) -> &PluginInfo {
        &self.info
    }

    #[cfg(target_os = "linux")]
    pub fn open_editor(&self, _window_id: u32) -> Result<(), Vst3Error> {
        if let Some(ref ctrl) = self.controller {
            unsafe {
                let view_ptr = ctrl.create_view(std::ptr::null());
                if view_ptr.is_null() {
                    return Err(Vst3Error::Vst3Error(-1));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Vst3Error {
    LibraryLoadFailed,
    FactoryNotFound,
    InstantiationFailed,
    Vst3Error(i32),
}

impl fmt::Display for Vst3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Vst3Error::LibraryLoadFailed => write!(f, "Failed to load .so library"),
            Vst3Error::FactoryNotFound => write!(f, "GetPluginFactory not found"),
            Vst3Error::InstantiationFailed => write!(f, "Failed to instantiate plugin"),
            Vst3Error::Vst3Error(c) => write!(f, "VST3 error code: {}", c),
        }
    }
}
impl std::error::Error for Vst3Error {}

/// Fungsi buat nge-scan direktori dan ngebalikin list Path beserta Info Plugin-nya
/// CUMA untuk baca metadata, TIDAK instantiate plugin
pub fn scan_vst3_plugins<P: AsRef<Path>>(dir: P) -> Vec<(PathBuf, PluginInfo)> {
    let mut plugins = Vec::new();
    let dir = dir.as_ref();

    if !dir.exists() {
        println!("[VST3 Scanner] Folder {:?} tidak ditemukan. Skip.", dir);
        return plugins;
    }

    println!("[VST3 Scanner] Mulai menyisir di: {:?}", dir);

    // Jalan-jalan ke semua sub-folder
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // 1. Cari folder "Bundle" yang belakangnya .vst3
        if path.is_dir() && path.extension().and_then(|s| s.to_str()) == Some("vst3") {
            // 2. Tembak langsung ke lokasi file .so (Standar Linux)
            let mut so_path = path.to_path_buf();
            so_path.push("Contents");
            so_path.push("x86_64-linux");

            if so_path.exists() {
                if let Ok(entries) = std::fs::read_dir(&so_path) {
                    for so_entry in entries.flatten() {
                        let p = so_entry.path();

                        // 3. Pastikan itu file .so
                        if p.extension().and_then(|s| s.to_str()) == Some("so") {
                            println!(
                                "[VST3 Scanner] Ditemukan kandidat: {:?}",
                                p.file_name().unwrap()
                            );

                            // 4. Cuma baca metadata, JANGAN instantiate plugin (biar gak crash)
                            match get_plugin_info_only(&p) {
                                Ok(info) => {
                                    println!("  => Metadata OK: {} by {}", info.name, info.vendor);
                                    plugins.push((p.clone(), info));
                                }
                                Err(e) => {
                                    eprintln!(
                                        "  => [ERROR] Gagal baca metadata {:?}: {}",
                                        p.file_name().unwrap(),
                                        e
                                    );
                                }
                            }
                            break; // Cukup 1 file .so per bundle
                        }
                    }
                }
            }
        }
    }

    println!(
        "[VST3 Scanner] Selesai! Menemukan {} plugin sehat.",
        plugins.len()
    );
    plugins
}

/// Cuma baca metadata plugin TANPA instantiate
/// Ini aman karena gak perlu inisialisasi VST3 SDK
fn get_plugin_info_only<P: AsRef<Path>>(path: P) -> Result<PluginInfo, Vst3Error> {
    let path = path.as_ref();

    println!("  => Loading library...");

    // Load library
    let library = match unsafe { libloading::Library::new(path.as_os_str()) } {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("  => Failed to load library: {:?}", e);
            return Err(Vst3Error::LibraryLoadFailed);
        }
    };

    println!("  => Library loaded OK");

    // Get factory function
    println!("  => Looking for GetPluginFactory...");
    let factory_fn: libloading::Symbol<unsafe extern "system" fn() -> *mut c_void> =
        match unsafe { library.get(b"GetPluginFactory\0") } {
            Ok(f) => f,
            Err(e) => {
                eprintln!("  => Failed to get GetPluginFactory: {:?}", e);
                return Err(Vst3Error::FactoryNotFound);
            }
        };

    println!("  => GetPluginFactory found!");

    let factory_ptr = unsafe { factory_fn() };
    println!("  => Factory pointer: {:?}", factory_ptr);
    if factory_ptr.is_null() {
        return Err(Vst3Error::FactoryNotFound);
    }

    // Get factory2 interface
    println!("  => Creating factory interface...");
    let factory2: vst3_sys::VstPtr<dyn IPluginFactory2> =
        unsafe { std::mem::transmute(factory_ptr) };

    // Get class info for first class (usually the audio effect)
    println!("  => Reading class info...");
    let mut class_info: PClassInfo2 = unsafe { std::mem::zeroed() };
    let result = unsafe { factory2.get_class_info2(0, &mut class_info) };
    println!("  => get_class_info2 result: {}", result);

    if result != 0 {
        return Err(Vst3Error::Vst3Error(result));
    }

    let info = PluginInfo {
        name: i8_array_to_string(&class_info.name),
        vendor: i8_array_to_string(&class_info.vendor),
        category: i8_array_to_string(&class_info.category),
        uid: class_info.cid.data,
    };

    println!("  => Got info: {} by {}", info.name, info.vendor);

    // Library and factory pointers are dropped here automatically
    // We don't keep them alive because we don't need to instantiate

    Ok(info)
}

/* --- END --- */
