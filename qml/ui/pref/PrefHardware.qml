import QtQuick
import QtQuick.Layouts

ColumnLayout {
    spacing: 12

    PrefDropdown {
        label: "Audio Output Device"
        description: "Select the audio backend to route your music."
        model: (Qt.platform.os === "windows") ? ["WASAPI (Shared)", "WASAPI (Exclusive)", "ASIO"] :
               (Qt.platform.os === "android") ? ["AAudio (High-Res)", "OpenSL ES"] :
               ["PipeWire", "PulseAudio", "ALSA"]
        
        currentIndex: (Qt.platform.os === "linux") ? 2 : 0
        onOptionSelected: (index, value) => {
            console.log("OS: " + Qt.platform.os + " | User milih:", value)
            musicModel.set_output_device(index)
        }
    }

    PrefSwitch {
        label: "High-Res 64-bit Internal Processing"
        description: "Enables 64-bit floating-point processing for pristine audio quality"
        checked: musicModel.highres_enabled
        onToggled: musicModel.set_highres(checked)
    }
}