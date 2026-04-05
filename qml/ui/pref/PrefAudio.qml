
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Flickable {
    id: audioFlick
    contentHeight: audioColumn.height
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    ScrollBar.vertical: ScrollBar {
        policy: ScrollBar.AsNeeded
        width: 4
        z: 1
        background: Rectangle { implicitWidth: 4; implicitHeight: 20; color: theme.colormap.bgmain; opacity: 0.0 }
        contentItem: Rectangle {
            implicitWidth: 4
            implicitHeight: 30
            radius: 2
            color: theme.colormap.playeraccent
            Behavior on color { ColorAnimation { duration: 200 } }
        }
    }

    ColumnLayout {
        id: audioColumn
        width: audioFlick.width - 15
        spacing: 24

        // ==========================================
        // KELOMPOK 1: MASTER
        // ==========================================
        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8

             PrefSwitch {
                 label: "Bypass All DSP processing"
                 checked: !musicModel.dsp_enabled
                 onToggled: musicModel.toggle_dsp()
             }

             // Smart Loudness Normalizer (collapsible)
            ColumnLayout {
                id: normalizerSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    // Chevron + Label (clickable to toggle dropdown only)
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: chevronText.implicitWidth + labelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: normalizerSection.expanded = !normalizerSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: chevronText
                                    text: normalizerSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: labelText
                                    text: "Smart Loudness Normalizer"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    // Switch (separate - toggle on/off only)
                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.normalizer_enabled ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]

                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.normalizer_enabled ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_normalizer()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: normalizerSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Target LUFS"
                        valueText: musicModel.normalizer_target_lufs.toFixed(1)
                        fromValue: -24.0; toValue: -10.0; stepValue: 0.5
                        currentValue: musicModel.normalizer_target_lufs
                        defaultValue: -14.0
                        onMoved: (v) => musicModel.set_normalizer_target_lufs(v)
                        onResetToDefault: musicModel.set_normalizer_target_lufs(-14.0)
                    }
                    PrefSlider {
                        label: "True Peak Ceiling"
                        valueText: musicModel.normalizer_true_peak_dbtp.toFixed(1) + " dBTP"
                        fromValue: -3.0; toValue: 0.0; stepValue: 0.1
                        currentValue: musicModel.normalizer_true_peak_dbtp
                        defaultValue: -1.5
                        onMoved: (v) => musicModel.set_normalizer_true_peak_dbtp(v)
                        onResetToDefault: musicModel.set_normalizer_true_peak_dbtp(-1.5)
                    }
                    PrefSlider {
                        label: "Max Gain Boost"
                        valueText: "+" + musicModel.normalizer_max_gain_db.toFixed(1) + " dB"
                        fromValue: 0.0; toValue: 12.0; stepValue: 0.5
                        currentValue: musicModel.normalizer_max_gain_db
                        defaultValue: 12.0
                        onMoved: (v) => musicModel.set_normalizer_max_gain_db(v)
                        onResetToDefault: musicModel.set_normalizer_max_gain_db(12.0)
                    }
                }
            }
        }

        // ==========================================
        // KELOMPOK 2: DYNAMIC RANGE (Normalizer dkk)
        // ==========================================
        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8
            enabled: musicModel.dsp_enabled
            opacity: enabled ? 1.0 : 0.4
            Behavior on opacity { NumberAnimation { duration: 150 } }

            // === Reverb (collapsible) ===
            ColumnLayout {
                id: reverbSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: reverbChevronText.implicitWidth + reverbLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: reverbSection.expanded = !reverbSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: reverbChevronText
                                    text: reverbSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: reverbLabelText
                                    text: "Reverb"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.reverb_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.reverb_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_reverb_master()
                        }
                    }
                }

                RowLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: reverbSection.expanded
                    spacing: 8
                    Repeater {
                        model: ["STAGE", "HALL", "STADIUM"]
                        delegate: Rectangle {
                            Layout.preferredWidth: reverbBtnText.implicitWidth + 20
                            Layout.preferredHeight: 24
                            radius: 4
                            color: musicModel.current_reverb.toLowerCase() === modelData.toLowerCase() ? theme.colormap["playeraccent"] : theme.colormap["bgoverlay"]
                            border.color: theme.colormap["graysolid"]
                            border.width: 1
                            Text {
                                id: reverbBtnText
                                anchors.centerIn: parent
                                text: modelData
                                font.family: kodeMono.name
                                font.pixelSize: 10
                                color: musicModel.current_reverb.toLowerCase() === modelData.toLowerCase() ? theme.colormap["bgmain"] : theme.colormap["playlisttext"]
                            }
                            MouseArea {
                                anchors.fill: parent
                                cursorShape: Qt.PointingHandCursor
                                onClicked: musicModel.set_reverb(modelData.toLowerCase())
                            }
                        }
                    }
                    Item { Layout.fillWidth: true }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: reverbSection.expanded && musicModel.reverb_active
                    spacing: 6

                    PrefSlider {
                        label: "Room Size"
                        valueText: Math.round(musicModel.reverb_room_size * 100) + "%"
                        fromValue: 0.0; toValue: 1.0; stepValue: 0.01
                        currentValue: musicModel.reverb_room_size
                        defaultValue: 0.55
                        onMoved: (v) => musicModel.set_reverb_room_size(v)
                        onResetToDefault: musicModel.set_reverb_room_size(0.55)
                    }
                    PrefSlider {
                        label: "Dampening"
                        valueText: Math.round(musicModel.reverb_damp * 100) + "%"
                        fromValue: 0.0; toValue: 1.0; stepValue: 0.01
                        currentValue: musicModel.reverb_damp
                        defaultValue: 0.5
                        onMoved: (v) => musicModel.set_reverb_damp(v)
                        onResetToDefault: musicModel.set_reverb_damp(0.5)
                    }
                }
            }

            // === Compressor (collapsible) ===
            ColumnLayout {
                id: compressorSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: compChevronText.implicitWidth + compLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: compressorSection.expanded = !compressorSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: compChevronText
                                    text: compressorSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: compLabelText
                                    text: "Compressor"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.compressor_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.compressor_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_compressor()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: compressorSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Threshold"
                        valueText: musicModel.get_compressor_threshold().toFixed(1) + " dB"
                        fromValue: -30.0; toValue: 0.0; stepValue: 0.5
                        currentValue: musicModel.get_compressor_threshold()
                        defaultValue: -18.0
                        onMoved: (v) => musicModel.set_compressor_threshold(v)
                        onResetToDefault: musicModel.set_compressor_threshold(-18.0)
                    }
                }
            }

            // === Pitch Shifter (collapsible) ===
            ColumnLayout {
                id: pitchSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: pitchChevronText.implicitWidth + pitchLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: pitchSection.expanded = !pitchSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: pitchChevronText
                                    text: pitchSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: pitchLabelText
                                    text: "Pitch Shifter"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.pitch_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.pitch_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_pitch()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: pitchSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Semitones"
                        valueText: musicModel.pitch_semitones === 0 ? "ORIGINAL" : ((musicModel.pitch_semitones > 0 ? "+" : "") + musicModel.pitch_semitones.toFixed(0) + " ST")
                        fromValue: -12.0; toValue: 12.0; stepValue: 1.0
                        currentValue: musicModel.pitch_semitones
                        defaultValue: 0.0
                        onMoved: (v) => musicModel.set_pitch_semitones(v)
                        onResetToDefault: musicModel.set_pitch_semitones(0.0)
                    }
                }
            }

            // === Middle Clarity (collapsible) ===
            ColumnLayout {
                id: middleSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: middleChevronText.implicitWidth + middleLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: middleSection.expanded = !middleSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: middleChevronText
                                    text: middleSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: middleLabelText
                                    text: "Middle Clarity"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.middle_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.middle_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_middle()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: middleSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Amount"
                        valueText: Math.round(musicModel.middle_amount * 100) + "%"
                        fromValue: 0.0; toValue: 1.0; stepValue: 0.01
                        currentValue: musicModel.middle_amount
                        defaultValue: 0.0
                        onMoved: (v) => musicModel.set_middle_amount(v)
                        onResetToDefault: musicModel.set_middle_amount(0.0)
                    }
                }
            }

            // === Stereo Width (collapsible) ===
            ColumnLayout {
                id: stereoWidthSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: stereoWidthChevronText.implicitWidth + stereoWidthLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: stereoWidthSection.expanded = !stereoWidthSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: stereoWidthChevronText
                                    text: stereoWidthSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: stereoWidthLabelText
                                    text: "Stereo Width"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.mono_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.mono_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_mono()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: stereoWidthSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Width"
                        valueText: musicModel.mono_width.toFixed(1)
                        fromValue: 0.0; toValue: 2.0; stepValue: 0.1
                        currentValue: musicModel.mono_width
                        defaultValue: 1.0
                        onMoved: (v) => musicModel.set_mono_width(v)
                        onResetToDefault: musicModel.set_mono_width(1.0)
                    }
                }
            }

            // === Stereo Enhance (collapsible) ===
            ColumnLayout {
                id: stereoEnhanceSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: stereoEnhanceChevronText.implicitWidth + stereoEnhanceLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: stereoEnhanceSection.expanded = !stereoEnhanceSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: stereoEnhanceChevronText
                                    text: stereoEnhanceSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: stereoEnhanceLabelText
                                    text: "Stereo Enhance"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.stereo_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.stereo_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_stereo()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: stereoEnhanceSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Amount"
                        valueText: Math.round(musicModel.stereo_amount * 100) + "%"
                        fromValue: 0.0; toValue: 1.0; stepValue: 0.01
                        currentValue: musicModel.stereo_amount
                        defaultValue: 0.0
                        onMoved: (v) => musicModel.set_stereo_amount(v)
                        onResetToDefault: musicModel.set_stereo_amount(0.0)
                    }
                }
            }

            // === Headphone Crossfeed (collapsible) ===
            ColumnLayout {
                id: crossfeedSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: crossfeedChevronText.implicitWidth + crossfeedLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: crossfeedSection.expanded = !crossfeedSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: crossfeedChevronText
                                    text: crossfeedSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: crossfeedLabelText
                                    text: "Headphone Crossfeed"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.crossfeed_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.crossfeed_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_crossfeed()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: crossfeedSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Amount"
                        valueText: Math.round(musicModel.crossfeed_amount * 100) + "%"
                        fromValue: 0.0; toValue: 1.0; stepValue: 0.01
                        currentValue: musicModel.crossfeed_amount
                        defaultValue: 0.0
                        onMoved: (v) => musicModel.set_crossfeed_amount(v)
                        onResetToDefault: musicModel.set_crossfeed_amount(0.0)
                    }
                }
            }
        }

        // ==========================================
        // KELOMPOK 3: TONAL & SPATIAL
        // ==========================================
        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8
            enabled: musicModel.dsp_enabled
            opacity: enabled ? 1.0 : 0.4

            // === Bass Booster (collapsible) ===
            ColumnLayout {
                id: bassBoosterSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: bassChevronText.implicitWidth + bassLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: bassBoosterSection.expanded = !bassBoosterSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: bassChevronText
                                    text: bassBoosterSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: bassLabelText
                                    text: "Bass Booster"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.bassbooster_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.bassbooster_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_bassbooster()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: bassBoosterSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Intensity"
                        valueText: "+" + musicModel.bass_gain.toFixed(1) + " dB"
                        fromValue: 0.0; toValue: 12.0; stepValue: 0.5
                        currentValue: musicModel.bass_gain
                        defaultValue: 6.0
                        onMoved: (v) => musicModel.set_bass_gain(v)
                        onResetToDefault: musicModel.set_bass_gain(6.0)
                    }
                    PrefSlider {
                        label: "Cutoff Frequency"
                        valueText: musicModel.bass_cutoff.toFixed(0) + " Hz"
                        fromValue: 20.0; toValue: 500.0; stepValue: 5.0
                        currentValue: musicModel.bass_cutoff
                        defaultValue: 180.0
                        onMoved: (v) => musicModel.set_bass_cutoff(v)
                        onResetToDefault: musicModel.set_bass_cutoff(180.0)
                    }
                }
            }

            // === Surround (collapsible) ===
            ColumnLayout {
                id: surroundSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: surChevronText.implicitWidth + surLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: surroundSection.expanded = !surroundSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: surChevronText
                                    text: surroundSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: surLabelText
                                    text: "Surround"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.surround_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.surround_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_surround()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: surroundSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Width"
                        valueText: musicModel.surround_width.toFixed(1)
                        fromValue: 0.0; toValue: 2.0; stepValue: 0.1
                        currentValue: musicModel.surround_width
                        defaultValue: 1.8
                        onMoved: (v) => musicModel.set_surround_width(v)
                        onResetToDefault: musicModel.set_surround_width(1.8)
                    }
                }
            }

            // === Crystalizer (collapsible) ===
            ColumnLayout {
                id: crystalizerSection
                property bool expanded: false
                Layout.fillWidth: true
                spacing: 0

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        MouseArea {
                            width: cryChevronText.implicitWidth + cryLabelText.implicitWidth + 12
                            height: 22
                            cursorShape: Qt.PointingHandCursor
                            onClicked: crystalizerSection.expanded = !crystalizerSection.expanded
                            RowLayout {
                                spacing: 6
                                Text {
                                    id: cryChevronText
                                    text: crystalizerSection.expanded ? "󰅀" : "󰅂"
                                    font.family: symbols.name
                                    font.pixelSize: 11
                                    color: theme.colormap["playersubtext"]
                                }
                                Text {
                                    id: cryLabelText
                                    text: "Crystalizer"
                                    font.family: kodeMono.name
                                    font.pixelSize: 13
                                    color: theme.colormap["playlisttext"]
                                }
                            }
                        }
                        Item { Layout.fillWidth: true }
                    }

                    Rectangle {
                        Layout.alignment: Qt.AlignVCenter
                        width: 30; height: 16
                        radius: 8
                        color: musicModel.crystalizer_active ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        Rectangle {
                            width: 12; height: 12
                            radius: 6
                            color: theme.colormap["bgmain"]
                            y: 2
                            x: musicModel.crystalizer_active ? parent.width - width - 2 : 2
                            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: musicModel.toggle_crystalizer()
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.leftMargin: 24
                    Layout.topMargin: 4
                    visible: crystalizerSection.expanded
                    spacing: 6

                    PrefSlider {
                        label: "Amount"
                        valueText: Math.round(musicModel.crystal_amount * 100) + "%"
                        fromValue: 0.0; toValue: 1.0; stepValue: 0.01
                        currentValue: musicModel.crystal_amount
                        defaultValue: 0.2
                        onMoved: (v) => musicModel.set_crystalizer_amount(v)
                        onResetToDefault: musicModel.set_crystalizer_amount(0.2)
                    }
                }
            }

        }

        Item { Layout.fillHeight: true; Layout.minimumHeight: 20 }
    }
}
