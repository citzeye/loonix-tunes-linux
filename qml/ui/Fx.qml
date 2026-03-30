/* --- LOONIX-TUNES qml/ui/Fx.qml --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    id: fx
    anchors.fill: parent

    // Properti State - reverbEnabled now bound to backend state
    property bool compEnabled: false

    // --- RAHASIA OCD: Definisi Lebar Konten ---
    readonly property bool isWide: width > 500
    // Kita tentukan lebar maksimal konten supaya gak "meluber" di monitor ultrawide
    readonly property int maxContentWidth: 600
    readonly property int activeWidth: Math.min(width - 40, maxContentWidth)

    ScrollView {
        anchors.fill: parent
        contentWidth: parent.width
        clip: true

        ColumnLayout {
            // Kontainer utama sekarang punya lebar yang dikunci
            width: activeWidth
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: 12
            anchors.topMargin: 20
            anchors.bottomMargin: 20

            // --- 1. REVERB (Sekarang Lebarnya 100% dari Kontainer) ---
            Rectangle {
                id: reverbCard
                Layout.fillWidth: true // SEJAJAR DENGAN GRID
                Layout.preferredHeight: 65
                color: theme.colormap.bgoverlay
                radius: 4
                border.color: musicModel.reverb_active ? theme.colormap.playeraccent : theme.colormap.tabborder
                antialiasing: false

                ColumnLayout {
                    anchors.centerIn: parent
                    spacing: 4
                    RowLayout {
                        Layout.alignment: Qt.AlignHCenter
                        spacing: 6
                        Text {
                            text: "REVERB"
                            font.family: kodeMono.name
                            font.pixelSize: 11
                            font.bold: true
                            color: musicModel.reverb_active ? theme.colormap.playeraccent : theme.colormap.playersubtext
                        }
                        Text {
                            text: musicModel.reverb_active ? '󰔡' : '󰨙'
                            font.family: symbols.name
                            font.pixelSize: 16
                            color: musicModel.reverb_active ? theme.colormap.playerhover : theme.colormap.playersubtext
                            MouseArea {
                                anchors.fill: parent
                                onClicked: musicModel.toggle_reverb_master()
                            }
                        }
                    }
                    RowLayout {
                        spacing: 8
                        Repeater {
                            model: ["STAGE", "HALL", "STADIUM"]
                            delegate: Button {
                                id: rBtn
                                checkable: true
                                checked: musicModel.current_reverb.toLowerCase() === modelData.toLowerCase()
                                Layout.preferredWidth: 80 // Tombol tetap fixed biar rapi
                                Layout.preferredHeight: 18
                                contentItem: Text {
                                    text: modelData
                                    font.family: kodeMono.name
                                    font.pixelSize: 9
                                    color: rBtn.checked ? "black" : "white"
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                }
                                background: Rectangle {
                                    color: rBtn.checked ? theme.colormap.playeraccent : "transparent"
                                    border.color: theme.colormap.tabborder
                                    radius: 3
                                    antialiasing: false
                                }
                                onClicked: musicModel.set_reverb(checked ? modelData.toLowerCase() : "off")
                            }
                        }
                    }
                }
            }

            // --- 2. GRID EFFECTS (50% + 50%) ---
            // --- GRID EFFECTS (3 Baris x 2 Kolom = 6 Efek) ---
            GridLayout {
                Layout.fillWidth: true
                columns: isWide ? 2 : 1
                columnSpacing: 12
                rowSpacing: 12

                FxSliderItem {
                    title: "COMPRESSOR"
                    enabledState: musicModel.compressor_active
                    controlValue: musicModel.get_compressor_threshold()
                    defaultValue: 0.9
                    onToggled: musicModel.toggle_compressor()
                    onSliderMoved: (val) => musicModel.set_compressor_threshold(val)
                }
                // PITCH SHIFTER - Bipolar Center-Aligned Slider
                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 55
                    color: theme.colormap.bgoverlay
                    radius: 4
                    opacity: musicModel.pitch_active ? 1.0 : 0.5
                    border.color: musicModel.pitch_active ? theme.colormap.playeraccent : "transparent"
                    antialiasing: false

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 12
                        anchors.rightMargin: 12
                        spacing: 2

                        RowLayout {
                            Layout.fillWidth: true
                            Text {
                                text: "PITCH SHIFTER"
                                font.family: kodeMono.name
                                font.pixelSize: 10
                                color: musicModel.pitch_active ? theme.colormap.playeraccent : theme.colormap.playersubtext
                                Layout.fillWidth: true
                            }
                            Text {
                                text: musicModel.pitch_active ? '󰔡' : '󰨙'
                                font.family: symbols.name
                                font.pixelSize: 16
                                color: musicModel.pitch_active ? theme.colormap.playerhover : theme.colormap.playersubtext
                                MouseArea {
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    onClicked: musicModel.toggle_pitch()
                                }
                            }
                        }

                        RowLayout {
                            spacing: 10
                            Slider {
                                id: pitchSld
                                Layout.fillWidth: true
                                Layout.preferredHeight: 20
                                enabled: musicModel.pitch_active
                                from: -12.0
                                to: 12.0
                                stepSize: 1.0

                                // Snap to 0 deadzone
                                value: musicModel.pitch_semitones

                                onMoved: {
                                    var v = pitchSld.value
                                    if (Math.abs(v) < 0.5) v = 0.0
                                    musicModel.set_pitch_semitones(v)
                                }

                                WheelHandler {
                                    target: pitchSld
                                    acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                                    property: "position"
                                    orientation: Qt.Vertical
                                    onWheel: function(event) {
                                        var step = 1.0
                                        var delta = event.angleDelta.y > 0 ? step : -step
                                        var newVal = Math.max(-12.0, Math.min(12.0, pitchSld.value + delta))
                                        if (Math.abs(newVal) < 0.5) newVal = 0.0
                                        pitchSld.value = newVal
                                        musicModel.set_pitch_semitones(newVal)
                                    }
                                }

                                background: Rectangle {
                                    height: 3
                                    radius: 1.5
                                    color: "#111"
                                    y: (parent.height - height) / 2

                                    // Center tick mark
                                    Rectangle {
                                        width: 2
                                        height: 8
                                        anchors.centerIn: parent
                                        color: theme.colormap.playersubtext
                                        opacity: 0.5
                                    }

                                    // Bipolar fill: from center to handle
                                    Rectangle {
                                        anchors.verticalCenter: parent.verticalCenter
                                        height: parent.height
                                        radius: 1.5
                                        color: theme.colormap.playeraccent
                                        x: pitchSld.visualPosition >= 0.5 ? parent.width / 2 : pitchSld.visualPosition * parent.width
                                        width: Math.abs(pitchSld.visualPosition - 0.5) * parent.width
                                    }
                                }

                                handle: Rectangle {
                                    x: pitchSld.leftPadding + pitchSld.visualPosition * (pitchSld.availableWidth - width)
                                    y: (pitchSld.availableHeight - height) / 2
                                    width: 10
                                    height: 10
                                    radius: 5
                                    color: pitchSld.value === 0 ? "#ffffff" : (pitchSld.pressed ? theme.colormap.playerhover : theme.colormap.playeraccent)
                                }
                            }

                            Text {
                                text: pitchSld.value === 0 ? "ORIGINAL" : ((pitchSld.value > 0 ? "+" : "") + pitchSld.value.toFixed(1) + " ST")
                                font.family: sansSerif.name
                                font.pixelSize: 9
                                color: theme.colormap.playersubtext
                                Layout.preferredWidth: 70
                            }

                            Text {
                                text: '󰜉'
                                font.family: symbols.name
                                font.pixelSize: 14
                                color: theme.colormap.playersubtext
                                MouseArea {
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    onClicked: {
                                        pitchSld.value = 0
                                        musicModel.set_pitch_semitones(0)
                                    }
                                }
                            }
                        }
                    }
                }
                FxSliderItem {
                    title: "MIDDLE CLARITY"
                    enabledState: musicModel.middle_active
                    controlValue: musicModel.middle_amount
                    onToggled: musicModel.toggle_middle()
                    onSliderMoved: function(val) { musicModel.set_middle_amount(val) }
                }
                FxSliderItem {
                    title: "STEREO WIDTH"
                    enabledState: musicModel.mono_active
                    controlValue: musicModel.mono_width
                    defaultValue: 1.0
                    leftLabel: "mono"
                    rightLabel: "stereo"
                    onToggled: musicModel.toggle_mono()
                    onSliderMoved: function(val) { musicModel.set_mono_width(val) }
                }

                // Dua slot terakhir: Stereo dan Saturation
                FxSliderItem {
                    title: "STEREO ENHANCE"
                    enabledState: musicModel.stereo_active
                    controlValue: musicModel.stereo_amount
                    onToggled: musicModel.toggle_stereo()
                    onSliderMoved: function(val) { musicModel.set_stereo_amount(val) }
                }
                FxSliderItem {
                    title: "HEADPHONE CROSSFEED"
                    enabledState: musicModel.crossfeed_active
                    controlValue: musicModel.crossfeed_amount
                    defaultValue: 0.0
                    onToggled: musicModel.toggle_crossfeed()
                    onSliderMoved: function(val) { musicModel.set_crossfeed_amount(val) }
                }
            }
        }
    }

    // --- COMPONENT: SEJAJAR PIXEL-PERFECT (Fixed Edition) ---
    component FxSliderItem: Rectangle {
        id: rootItem
        property string title: ""
        property bool enabledState: false
        property real controlValue: 0.0
        property real defaultValue: 0.0
        property string leftLabel: ""
        property string rightLabel: ""
        signal toggled
        signal sliderMoved(real val)

        Layout.fillWidth: true
        Layout.preferredHeight: 55
        color: theme.colormap.bgoverlay
        radius: 4
        opacity: enabledState ? 1.0 : 0.5
        border.color: enabledState ? theme.colormap.playeraccent : "transparent"
        antialiasing: false

        ColumnLayout {
            anchors.fill: parent
            anchors.leftMargin: 12
            anchors.rightMargin: 12
            spacing: 2

            RowLayout {
                Layout.fillWidth: true
                Text {
                    text: title
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    color: enabledState ? theme.colormap.playeraccent : theme.colormap.playersubtext
                    Layout.fillWidth: true
                }
                Text {
                    text: enabledState ? '󰔡' : '󰨙'
                    font.family: symbols.name
                    font.pixelSize: 16
                    color: enabledState ? theme.colormap.playerhover : theme.colormap.playersubtext
                    MouseArea {
                        anchors.fill: parent
                        onClicked: rootItem.toggled()
                    }
                }
            }

            RowLayout {
                spacing: 6
                Text {
                    text: rootItem.leftLabel
                    font.family: kodeMono.name
                    font.pixelSize: 9
                    color: theme.colormap.playersubtext
                    visible: rootItem.leftLabel !== ""
                }
                Slider {
                    id: sld
                    Layout.fillWidth: true
                    Layout.preferredHeight: 20
                    enabled: enabledState
                    from: 0.0
                    to: 1.0
                    stepSize: 0.01

                    value: rootItem.controlValue
                    onMoved: rootItem.sliderMoved(sld.value)

                    WheelHandler {
                        target: sld
                        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                        property: "position"
                        orientation: Qt.Vertical
                        onWheel: function(event) {
                            if (!enabledState) return
                            var step = 0.05
                            var delta = event.angleDelta.y > 0 ? step : -step
                            var newVal = Math.max(0.0, Math.min(1.0, sld.value + delta))
                            sld.value = newVal
                            rootItem.sliderMoved(newVal)
                        }
                    }

                    background: Rectangle {
                        height: 3
                        radius: 1.5
                        color: "#111"
                        y: (parent.height - height) / 2
                        Rectangle {
                            width: sld.visualPosition * parent.width
                            height: parent.height
                            color: theme.colormap.playeraccent
                            radius: 1.5
                        }
                    }
                    handle: Rectangle {
                        x: sld.leftPadding + sld.visualPosition * (sld.availableWidth - width)
                        y: (sld.availableHeight - height) / 2
                        width: 10
                        height: 10
                        radius: 5
                        color: sld.pressed ? theme.colormap.playerhover : theme.colormap.playeraccent
                    }
                }
                Text {
                    text: rootItem.rightLabel
                    font.family: kodeMono.name
                    font.pixelSize: 9
                    color: theme.colormap.playersubtext
                    visible: rootItem.rightLabel !== ""
                }
                Text {
                    text: Math.round(sld.value * 100) + "%"
                    font.family: sansSerif.name
                    font.pixelSize: 9
                    color: theme.colormap.playersubtext
                    Layout.preferredWidth: 25
                }
                Text {
                    text: '󰜉'
                    font.family: symbols.name
                    font.pixelSize: 14
                    color: theme.colormap.playersubtext
                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            sld.value = defaultValue
                            rootItem.sliderMoved(defaultValue)
                        }
                    }
                }
            }
        }
    }
}
