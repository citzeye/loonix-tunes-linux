$bytes = [System.IO.File]::ReadAllBytes("target\release\loonix-tunes.exe")
$peOffset = [BitConverter]::ToInt32($bytes[0x3C..0x3F], 0)
$machine = [BitConverter]::ToInt16($bytes[$peOffset+4..$peOffset+5], 0)
$arch = switch ($machine) {
    0x014c { "x86 (32-bit)" }
    0x8664 { "x64 (64-bit)" }
    0xAA64 { "ARM64" }
    default { "Unknown: 0x" + $machine.ToString("X") }
}
Write-Host "Machine: $arch"