[Setup]
AppName=Loonix Tunes
AppVersion=1.0.1
DefaultDirName={autopf}\Loonix Tunes
DefaultGroupName=Loonix Tunes
OutputDir=..\..\Output
OutputBaseFilename=loonix-tunes-1.0.1-x86_64
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
SetupIconFile=icon.ico
UninstallDisplayIcon={app}\loonix-tunes.exe

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

#define ProjectRoot "..\.."

[Files]
; Main executable
Source: "{#ProjectRoot}\target\release\loonix-tunes.exe"; DestDir: "{app}"; Flags: ignoreversion

; All DLLs (Qt, FFmpeg, Rubberband, VCRuntime, etc) from target/release
Source: "{#ProjectRoot}\target\release\*.dll"; DestDir: "{app}"; Flags: ignoreversion

; vcpkg audio DLLs from packaging/windows/bin
Source: "bin\*.dll"; DestDir: "{app}"; Flags: ignoreversion

; Qt platform plugins
Source: "{#ProjectRoot}\target\release\platforms\*"; DestDir: "{app}\platforms"; Flags: ignoreversion recursesubdirs

; Qt QML modules
Source: "{#ProjectRoot}\target\release\qml\*"; DestDir: "{app}\qml"; Flags: ignoreversion recursesubdirs

; Qt image formats
Source: "{#ProjectRoot}\target\release\imageformats\*"; DestDir: "{app}\imageformats"; Flags: ignoreversion recursesubdirs

; Qt icon engines
Source: "{#ProjectRoot}\target\release\iconengines\*"; DestDir: "{app}\iconengines"; Flags: ignoreversion recursesubdirs

; Qt styles
Source: "{#ProjectRoot}\target\release\styles\*"; DestDir: "{app}\styles"; Flags: ignoreversion recursesubdirs

; Qt TLS backends
Source: "{#ProjectRoot}\target\release\tls\*"; DestDir: "{app}\tls"; Flags: ignoreversion recursesubdirs

; Qt generic plugins
Source: "{#ProjectRoot}\target\release\generic\*"; DestDir: "{app}\generic"; Flags: ignoreversion recursesubdirs

; Qt network information
Source: "{#ProjectRoot}\target\release\networkinformation\*"; DestDir: "{app}\networkinformation"; Flags: ignoreversion recursesubdirs

; Qt QML tooling
Source: "{#ProjectRoot}\target\release\qmltooling\*"; DestDir: "{app}\qmltooling"; Flags: ignoreversion recursesubdirs

; Assets (EQ presets, FX presets, fonts, icons, Qt config)
Source: "{#ProjectRoot}\assets\eqpreset.json"; DestDir: "{app}\assets"; Flags: ignoreversion
Source: "{#ProjectRoot}\assets\fxpreset.json"; DestDir: "{app}\assets"; Flags: ignoreversion
Source: "{#ProjectRoot}\assets\LoonixTunes.png"; DestDir: "{app}\assets"; Flags: ignoreversion
Source: "{#ProjectRoot}\assets\fonts\*"; DestDir: "{app}\assets\fonts"; Flags: ignoreversion recursesubdirs

; Qt Quick Controls config (must be next to .exe)
Source: "{#ProjectRoot}\assets\qtquickcontrols2.conf"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Loonix Tunes"; Filename: "{app}\loonix-tunes.exe"; IconFilename: "{app}\loonix-tunes.exe"
Name: "{autodesktop}\Loonix Tunes"; Filename: "{app}\loonix-tunes.exe"; IconFilename: "{app}\loonix-tunes.exe"; Tasks: desktopicon
Name: "{group}\Uninstall Loonix Tunes"; Filename: "{uninstallexe}"

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut"; GroupDescription: "Additional icons:"; Flags: unchecked

[Run]
Filename: "{app}\loonix-tunes.exe"; Description: "Launch Loonix Tunes"; Flags: nowait postinstall skipifsilent
