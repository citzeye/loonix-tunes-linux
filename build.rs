fn main() {
    // --- LINUX BUILD ---
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=rubberband");
        println!("cargo:rustc-link-lib=fftw3");
        println!("cargo:rustc-link-lib=samplerate");
    }

    // --- WINDOWS BUILD ---
    #[cfg(windows)]
    {
        let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        // Beritahu Rust buat nyari ke folder libs lo
        println!("cargo:rustc-link-search=native={}/packaging/windows/libs", project_dir);
        
        // Link library FFmpeg
        println!("cargo:rustc-link-lib=avcodec");
        println!("cargo:rustc-link-lib=avformat");
        println!("cargo:rustc-link-lib=avutil");
        println!("cargo:rustc-link-lib=swresample");
        
        // Link library DSP (Sesuaikan sama nama file .lib lo)
        println!("cargo:rustc-link-lib=rubberband");
        println!("cargo:rustc-link-lib=libfftw3-3"); 
        println!("cargo:rustc-link-lib=samplerate");
        println!("cargo:rustc-link-lib=sndfile"); // Tambahin ini kalau sndfile dipake

        // Metadata & Icon Windows
        let mut res = winres::WindowsResource::new();
        res.set_icon("packaging/windows/icon.ico");
        res.set_resource_file("packaging/windows/resource.rc");
        res.compile().unwrap();
    }

    println!("cargo:rerun-if-changed=packaging/windows/resource.rc");
}