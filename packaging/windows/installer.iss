[Setup]
AppName=Loonix-Tunes
AppVersion=1.0.0
DefaultDirName={autopf}\LoonixTunes
DefaultGroupName=Loonix-Tunes
UninstallDisplayIcon={app}\LoonixTunes.exe
Compression=lzma2
SolidCompression=yes
OutputDir=..\..\target\release\installer
OutputBaseFilename=LoonixTunes_v1.0.0_Windows_x64
SetupIconFile=icon.ico

[Files]
; Program Utama
Source: "..\..\target\release\loonix-tunes.exe"; DestDir: "{app}"; Flags: ignoreversion
; Icon Assets
Source: "icon.ico"; DestDir: "{app}"; Flags: ignoreversion
; Semua DLL FFmpeg & Dependency dari folder windows lo
Source: "*.dll"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Loonix-Tunes"; Filename: "{app}\Loonix-tunes.exe"
Name: "{autodesktop}\Loonix-Tunes"; Filename: "{app}\Loonix-tunes.exe"; IconFilename: "{app}\icon.ico"

[Run]
Filename: "{app}\Loonix-tunes.exe"; Description: "Launch Loonix-Tunes"; Flags: nowait postinstall skipifsilent