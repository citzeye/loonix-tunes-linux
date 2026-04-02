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
        println!(
            "cargo:rustc-link-search=native={}/packaging/windows/libs",
            project_dir
        );

        // FFmpeg di-handle oleh ffmpeg-sys-next (set FFMPEG_DIR)
        // Link library DSP lain
        println!("cargo:rustc-link-lib=rubberband");
        println!("cargo:rustc-link-lib=libfftw3-3");
        println!("cargo:rustc-link-lib=samplerate");
        println!("cargo:rustc-link-lib=sndfile");

        // Metadata & Icon Windows
        let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let icon_path = format!("{}/packaging/windows/icon.ico", project_dir);
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let rc_path = format!("{}/resource.rc", out_dir);
        let res_path = format!("{}/resource.res", out_dir);

        // Copy icon to OUT_DIR so rc.exe can find it with relative path
        let icon_in_out = format!("{}/icon.ico", out_dir);
        std::fs::copy(&icon_path, &icon_in_out).expect("Failed to copy icon to OUT_DIR");

        // Write .rc file with relative icon path (no #include needed)
        let rc_content = r#"
1 ICON "icon.ico"

1 VERSIONINFO
FILEVERSION     1,0,0,0
PRODUCTVERSION  1,0,0,0
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904E4"
        BEGIN
            VALUE "CompanyName",      "citz"
            VALUE "FileDescription",  "Loonix-Tunes: High-Res Cross-Platform Player"
            VALUE "FileVersion",      "1.0.0"
            VALUE "InternalName",     "loonix-tunes"
            VALUE "LegalCopyright",   "GPLv3"
            VALUE "OriginalFilename", "loonix-tunes.exe"
            VALUE "ProductName",      "Loonix-Tunes"
            VALUE "ProductVersion",   "1.0.0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x409, 1252
    END
END
"#;
        std::fs::write(&rc_path, rc_content).unwrap();

        // Compile .rc to .res using rc.exe from Windows SDK
        let rc_path_str = "C:/Program Files (x86)/Windows Kits/10/bin/10.0.26100.0/x64/rc.exe";
        let rc = std::process::Command::new(rc_path_str)
            .args(["/fo", &res_path, &rc_path])
            .output()
            .expect("Failed to run rc.exe");
        if !rc.status.success() {
            eprintln!("rc.exe stderr: {}", String::from_utf8_lossy(&rc.stderr));
            eprintln!("rc.exe stdout: {}", String::from_utf8_lossy(&rc.stdout));
        }

        // Link the .res file
        println!("cargo:rustc-link-arg={}", res_path);
    }

    println!("cargo:rerun-if-changed=packaging/windows/resource.rc");
}
