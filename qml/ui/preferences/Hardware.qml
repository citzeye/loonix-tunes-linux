import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Flickable {
    id: hardwareFlick
    contentWidth: width
    contentHeight: hardwareColumn.height
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    ColumnLayout {
        id: hardwareColumn
        width: hardwareFlick.width
        spacing: 12

        SettingHeader { title: "HARDWARE CONTROL" }

        SettingDropdown {
            label: "Audio Output Device"
            description: "Select the audio backend to route your music."
            // EDIT DI SINI: Model ganti otomatis sesuai OS
            model: (Qt.platform.os === "windows") ? ["WASAPI (Shared)", "WASAPI (Exclusive)", "ASIO"] :
                   (Qt.platform.os === "android") ? ["AAudio (High-Res)", "OpenSL ES"] :
                   ["PipeWire", "PulseAudio", "ALSA"]
            
            currentIndex: (Qt.platform.os === "linux") ? 2 : 0
            onOptionSelected: (index, value) => {
                console.log("OS: " + Qt.platform.os + " | User milih:", value)
                musicModel.set_output_device(index)
            }
        }
        
        // SOON
        // SettingSwitch {
        //     visible: Qt.platform.os !== "android"
        //     label: "Exclusive Mode"
        //     description: "Bypass system mixer. Locks DAC to track's native sample rate. Mutes other apps."
        //     checked: musicModel.exclusive_mode
        //     onToggled: musicModel.toggle_exclusive_mode()
        // }

        SettingSwitch {
            label: "High-Res 64-bit Internal Processing"
            description: "Enables 64-bit floating-point processing for pristine audio quality"
            checked: musicModel.highres_enabled
            onToggled: musicModel.set_highres(checked)
        }
    }
}