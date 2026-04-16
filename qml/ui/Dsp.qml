/* --- loonix-tunes/qml/ui/Dsp.qml --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Popup {
    id: dspRoot
    width: 500
    height: 520
    modal: true
    closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside

    background: Rectangle {
        color: theme.colormap.dspeqbg
        border.color: theme.colormap.dspborder
        border.width: 1
        radius: 4
    }

    contentItem: ColumnLayout {
        id: dspContent
        anchors.margins: 8
        spacing: 3

        // EQ Properties & Functions
        readonly property var freqLabels: ["31", "62", "125", "250", "500", "1k", "2k", "4k", "8k", "16k"]
        property var defaultPresets: []
        property var defaultPresetValues: []
        property var userPresets: ["USER 1", "USER 2", "USER 3", "USER 4", "USER 5", "USER 6"]

        function loadDefaultPresets() {
            var count = musicModel.get_eq_preset_count()
            var names = []
            var values = []
            for (var i = 0; i < count; i++) {
                names.push(musicModel.get_eq_preset_name(i))
                values.push(musicModel.get_eq_preset_gains(i))
            }
            defaultPresets = names
            defaultPresetValues = values
        }

        function refreshUserPresetNames() {
            var newNames = []
            for (var i = 0; i < 6; i++) {
                let name = musicModel.get_user_preset_name(i);
                newNames.push(name !== "" ? name : "User " + (i+1))
            }
            dspContent.userPresets = newNames;
        }

        Component.onCompleted: {
            loadDefaultPresets()
            refreshUserPresetNames()
        }

        property int activePresetIndex: -1

        onActivePresetIndexChanged: {
            musicModel.set_active_preset_index(activePresetIndex)
        }

        function loadPresetByIndex(index) {
            activePresetIndex = index
            var gains
            var macroVal = 0
            if (index >= 0 && index < 6) {
                gains = dspContent.defaultPresetValues[index]
            } else if (index >= 6 && index < 12) {
                var preset = index - 6
                gains = musicModel.get_user_eq_gains(preset)
                macroVal = musicModel.get_user_eq_macro(preset)
            } else {
                return
            }
            musicModel.set_eq_instant_apply()
            for (var i = 0; i < 10; ++i) {
                var slider = eqRepeater.itemAt(i).children[0].children[1]
                slider.value = gains[i]
                musicModel.set_eq_band(i, gains[i])
            }
            gainSlider.value = macroVal
        }

        function resetEQ() {
            activePresetIndex = -1
            musicModel.set_eq_instant_apply()
            for (var i = 0; i < 10; ++i) {
                var slider = eqRepeater.itemAt(i).children[0].children[1]
                slider.value = 0
                musicModel.set_eq_band(i, 0)
            }
            gainSlider.value = 0
        }


        GridLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: 20
            Layout.maximumHeight: 20
            columns: 3
            columnSpacing: 3
            rowSpacing: 3

            // TOGGLE BYPASS DSP
            Button {
                id: ampBtn
                Layout.preferredWidth: 157
                Layout.preferredHeight: 20

                onClicked: {
                    musicModel.set_preamp_gain(musicModel.get_preamp_gain() === 0 ? 3 : 0)
                }

                property bool isOn: musicModel.get_preamp_gain() !== 0

                background: Rectangle {
                    color: ampBtn.isOn ? theme.colormap.dspeqpresetactive : (ampBtn.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspeqbg)
                    border.color: ampBtn.isOn ? theme.colormap.dspfxtext : theme.colormap.dspborder
                    radius: 2
                }

                contentItem: Text {
                    text: "BYPASS"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    font.bold: true
                    color: ampBtn.isOn ? "black" : (ampBtn.hovered ? "black" : theme.colormap.dspfxsubtext)
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
            }

            // NORMALIZER - always ON
            Button {
                id: normBtn
                Layout.preferredWidth: 157
                Layout.preferredHeight: 20
                property bool isOn: true

                onClicked: {
                    // TODO: show normalizer dialog
                }

                background: Rectangle {
                    color: normBtn.isOn ? theme.colormap.dspeqpresetactive : (normBtn.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspeqbg)
                    border.color: normBtn.isOn ? theme.colormap.dspfxtext : theme.colormap.dspborder
                    radius: 2
                }

                contentItem: Text {
                    text: "NORMALIZER"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    font.bold: true
                    color: normBtn.isOn ? "black" : (normBtn.hovered ? "black" : theme.colormap.dspfxsubtext)
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
            }

            // EQ ON OFF
            Button {
                id: eqOnOffBtn
                Layout.preferredWidth: 157
                Layout.preferredHeight: 20
                property bool isOn: musicModel.eq_enabled

                onClicked: {
                    musicModel.set_eq_enabled(!musicModel.eq_enabled)
                }

                background: Rectangle {
                    color: eqOnOffBtn.isOn ? theme.colormap.dspeqpresetactive : (eqOnOffBtn.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspeqbg)
                    border.color: eqOnOffBtn.isOn ? theme.colormap.dspfxtext : theme.colormap.dspborder
                    radius: 2
                }

                contentItem: Text {
                    text: "EQ"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    font.bold: true
                    color: musicModel.eq_enabled ? "black" : (eqOnOffBtn.hovered ? "black" : theme.colormap.dspfxsubtext)
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
            }
        }

        

        // EQ Sliders (Rectangle)
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 110
            color: theme.colormap.dspeqbg
            radius: 4
            border.color: theme.colormap.dspborder

            RowLayout {
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                anchors.topMargin: 4
                anchors.bottomMargin: 4
                anchors.horizontalCenter: parent.horizontalCenter
                spacing: 3

                // AMP SLIDER
                ColumnLayout {
                    Layout.preferredWidth: 28
                    Layout.fillHeight: true
                    spacing: 3

                    Text {
                        Layout.alignment: Qt.AlignHCenter
                        Layout.fillHeight: true
                        text: ampSlider.pressed ? (ampSlider.value > 0 ? "+" + ampSlider.value + " dB" : ampSlider.value + " dB") : (ampSlider.value > 0 ? "+" + ampSlider.value : ampSlider.value)
                        color: ampSlider.pressed ? theme.colormap.dspeqpresetactive : theme.colormap.dspfxsubtext
                        font.family: sansSerif.name
                        font.pixelSize: 11
                    }

                    Slider {
                        id: ampSlider
                        Layout.alignment: Qt.AlignHCenter
                        Layout.fillHeight: true
                        Layout.preferredWidth: 28
                        orientation: Qt.Vertical
                        from: -20; to: 20
                        value: musicModel.get_preamp_gain()
                        stepSize: 1
                        padding: 0

                        onValueChanged: {
                            if (pressed || hovered) {
                                musicModel.set_preamp_gain(ampSlider.value)
                            }
                        }

                        background: Rectangle {
                            anchors.horizontalCenter: parent.horizontalCenter
                            width: 3; height: parent.height; radius: 1.5; color: theme.colormap.dspeqfaderbg
                            Rectangle {
                                width: parent.width; y: ampSlider.visualPosition * parent.height
                                height: parent.height - y; color: theme.colormap.dspeqfaderslider; radius: 1.5; opacity: 0.6
                            }
                        }
                        handle: Rectangle {
                            anchors.horizontalCenter: parent.horizontalCenter
                            y: ampSlider.topPadding + ampSlider.visualPosition * (ampSlider.availableHeight - height)
                            width: 10; height: 10; radius: 5; color: ampSlider.pressed ? theme.colormap.dspeqfaderslider : theme.colormap.dspeqfaderhandle
                            border.color: theme.colormap.dspfxborder; border.width: 1
                        }
                    }

                    Text {
                        Layout.alignment: Qt.AlignHCenter
                        text: "PREAMP"
                        color: theme.colormap.dspfxsubtext
                        font.family: kodeMono.name
                        font.pixelSize: 11
                    }

                    MouseArea {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        acceptedButtons: Qt.NoButton
                        onWheel: function(wheel) {
                            let delta = wheel.angleDelta.y > 0 ? 1 : -1
                            let newVal = Math.max(-20, Math.min(20, ampSlider.value + delta))
                            ampSlider.value = newVal
                            musicModel.set_preamp_gain(newVal)
                        }
                    }
                }

                // 10-BAND EQ
                RowLayout {
                    Layout.fillHeight: true
                    spacing: 3

                    Repeater {
                        id: eqRepeater
                        model: 10
                        delegate: Item {
                            Layout.preferredWidth: 28
                            Layout.fillHeight: true

                            ColumnLayout {
                                anchors.fill: parent
                                spacing: 5

                                Text {
                                    Layout.alignment: Qt.AlignHCenter
                                    text: Math.round(innerSlider.value)
                                    color: innerSlider.pressed ? theme.colormap.dspeqpresetactive : theme.colormap.dspfxsubtext
                                    font.family: sansSerif.name
                                    font.pixelSize: 11
                                }

                                Slider {
                                    id: innerSlider
                                    Layout.alignment: Qt.AlignHCenter
                                    Layout.fillHeight: true
                                    Layout.preferredWidth: 20
                                    orientation: Qt.Vertical
                                    from: -20; to: 20
                                    value: musicModel.get_eq_band_value(index)
                                    stepSize: 1
                                    padding: 0

                                    onValueChanged: {
                                        if (innerSlider.pressed || innerSlider.hovered) {
                                            musicModel.set_eq_band(index, innerSlider.value)
                                        }
                                    }

                                    background: Rectangle {
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        width: 3
                                        height: parent.height
                                        radius: 1.5
                                        color: theme.colormap.dspeq10bg

                                        Rectangle {
                                            width: parent.width
                                            y: innerSlider.visualPosition * parent.height
                                            height: parent.height - y
                                            color: theme.colormap.dspeq10slider
                                            radius: 1.5
                                            opacity: 0.6
                                        }
                                    }

                                    handle: Rectangle {
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        y: innerSlider.topPadding + innerSlider.visualPosition * (innerSlider.availableHeight - height)
                                        width: 10; height: 10; radius: 5
                                        color: innerSlider.pressed ? theme.colormap.dspeq10slider : theme.colormap.dspeq10handle
                                        border.color: theme.colormap.dspfxborder
                                        border.width: 1
                                    }
                                }

                                Text {
                                    Layout.alignment: Qt.AlignHCenter
                                    text: dspContent.freqLabels[index]
                                    color: theme.colormap.dspfxsubtext
                                    font.family: sansSerif.name
                                    font.pixelSize: 11
                                }
                            }

                            MouseArea {
                                anchors.fill: parent
                                hoverEnabled: true
                                acceptedButtons: Qt.NoButton
                                onWheel: function(wheel) {
                                    let delta = wheel.angleDelta.y > 0 ? 1 : -1
                                    let newVal = Math.max(-20, Math.min(20, innerSlider.value + delta))
                                    innerSlider.value = newVal
                                    musicModel.set_eq_band(index, newVal)
                                }
                            }
                        }
                    }
                }

                // FADER MACRO
                ColumnLayout {
                    Layout.preferredWidth: 28
                    Layout.fillHeight: true
                    spacing: 3

                    Text {
                        Layout.alignment: Qt.AlignHCenter
                        Layout.fillHeight: true
                        text: gainSlider.pressed ? (Math.round((gainSlider.value + 20) * 2.5) - 100) + "%" : ""
                        color: gainSlider.pressed ? theme.colormap.dspeqpresetactive : "transparent"
                        font.family: kodeMono.name
                        font.pixelSize: 9
                    }

                    Slider {
                        id: gainSlider
                        Layout.alignment: Qt.AlignHCenter
                        Layout.fillHeight: true
                        Layout.preferredWidth: 20
                        orientation: Qt.Vertical
                        from: -20; to: 20
                        value: 0
                        stepSize: 1
                        padding: 0

                        property real previousValue: 0

                        onPressedChanged: {
                            if (pressed) {
                                previousValue = value
                            }
                            if (!pressed && value !== 0) {
                                Timer.singleShot(100, function() {
                                    gainSlider.value = 0
                                })
                            }
                        }

                        onValueChanged: {
                            if (pressed) {
                                let delta = value - previousValue
                                if (delta !== 0) {
                                    for (let i = 0; i < 10; ++i) {
                                        let slider = eqRepeater.itemAt(i).children[0].children[1]
                                        let newVal = Math.max(-20, Math.min(20, slider.value + delta))
                                        slider.value = newVal
                                        musicModel.set_eq_band(i, newVal)
                                    }
                                    previousValue = value
                                }
                            }
                        }

                        background: Rectangle {
                            anchors.horizontalCenter: parent.horizontalCenter
                            width: 3; height: parent.height; radius: 1.5; color: theme.colormap.dspeqfaderbg
                            Rectangle {
                                width: parent.width; y: gainSlider.visualPosition * parent.height
                                height: parent.height - y; color: theme.colormap.dspeqfaderslider; radius: 1.5; opacity: 0.6
                            }
                        }
                        handle: Rectangle {
                            anchors.horizontalCenter: parent.horizontalCenter
                            y: gainSlider.topPadding + gainSlider.visualPosition * (gainSlider.availableHeight - height)
                            width: 10; height: 10; radius: 5; color: gainSlider.pressed ? theme.colormap.dspeqfaderslider : theme.colormap.dspeqfaderhandle
                            border.color: theme.colormap.dspfxborder; border.width: 1
                        }
                    }

                    Text {
                        Layout.alignment: Qt.AlignHCenter
                        text: "FADER"
                        color: theme.colormap.dspfxsubtext
                        font.family: kodeMono.name
                        font.pixelSize: 9
                    }

                    MouseArea {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 1
                        acceptedButtons: Qt.NoButton
                        onWheel: function(wheel) {
                            let delta = wheel.angleDelta.y > 0 ? 1 : -1
                            let newVal = Math.max(-20, Math.min(20, gainSlider.value + delta))
                            gainSlider.value = newVal
                            for (let i = 0; i < 10; ++i) {
                                let slider = eqRepeater.itemAt(i).children[0].children[1]
                                slider.value = Math.max(-20, Math.min(20, slider.value + delta))
                                musicModel.set_eq_band(i, slider.value)
                            }
                        }
                    }
}
        }
        }

        // EQ Controls (RESET - SAVE AS)
        RowLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: 20
            spacing: 3

            Button {
                id: resetBtn
                Layout.fillWidth: true
                Layout.preferredWidth: 1
                Layout.preferredHeight: 20

                onClicked: {
                    dspContent.resetEQ()
                }

                background: Rectangle {
                    color: resetBtn.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspeqbg
                    border.color: theme.colormap.dspborder
                    radius: 2
                }

                contentItem: Text {
                    text: "RESET"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    font.bold: true
                    color: resetBtn.hovered ? "black" : theme.colormap.dspfxsubtext
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
            }

            Button {
                id: saveBtn
                Layout.fillWidth: true
                Layout.preferredWidth: 1
                Layout.preferredHeight: 20

                onClicked: {
                    nameInput.text = ""
                    saveEqDialog.open()
                }

                background: Rectangle {
                    color: saveBtn.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspeqbg
                    border.color: theme.colormap.dspborder
                    radius: 2
                }

                contentItem: Text {
                    text: "SAVE AS"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    font.bold: true
                    color: saveBtn.hovered ? "black" : theme.colormap.dspfxsubtext
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
            }
        }

        // Default Presets Grid
        RowLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: 20
            spacing: 3

            Repeater {
                model: dspContent.defaultPresets
                delegate: Button {
                    id: defBtn
                    property bool isActive: dspContent.activePresetIndex === index
                    Layout.fillWidth: true
                    Layout.preferredHeight: 20
                    contentItem: Text {
                        text: modelData
                        font.family: kodeMono.name
                        font.pixelSize: 10
                        color: defBtn.isActive ? "black" : (defBtn.hovered ? "black" : theme.colormap.dspfxsubtext)
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        color: defBtn.isActive ? theme.colormap.dspeqpresetactive : (defBtn.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspeqbg)
                        border.color: defBtn.isActive ? theme.colormap.dspfxtext : theme.colormap.dspborder
                        radius: 2
                    }

                    onClicked: {
                        dspContent.loadPresetByIndex(index)
                    }
                }
            }
        }

        // EQ User Presets Grid
        RowLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: 20
            spacing: 3

            Repeater {
                model: dspContent.userPresets
                delegate: Button {
                    id: pBtn
                    property bool isActive: dspContent.activePresetIndex === index + 6
                    Layout.fillWidth: true
                    Layout.preferredHeight: 20
                    contentItem: Text {
                        text: modelData
                        font.family: kodeMono.name
                        font.pixelSize: 10
                        color: pBtn.isActive ? "black" : (pBtn.hovered ? "black" : theme.colormap.dspfxsubtext)
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        color: pBtn.isActive ? theme.colormap.dspeqpresetactive : (pBtn.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspeqbg)
                        border.color: pBtn.isActive ? theme.colormap.dspfxtext : theme.colormap.dspborder
                        radius: 2
                    }

                    onClicked: {
                        dspContent.loadPresetByIndex(index + 6)
                    }
                }
            }
        }

        // FX Section
        ColumnLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: 200
            anchors.margins: 6
            spacing: 3

            // COMPRESSOR
            RowLayout {
                Layout.fillWidth: true
                spacing: 3
                

                FxToggleBox {
                    id: compToggle
                    title: "COMPRESSOR"
                    isOn: musicModel.compressor_active
                    onToggled: musicModel.toggleStdCompressor()
                }

                FxSliderBox {
                    id: compSlider
                    enabled: compToggle.isOn && musicModel.dsp_enabled
                    controlValue: musicModel.compressor_threshold
                    onSliderChanged: val => musicModel.setStdCompressorThreshold(val)
                }
                FxValueBox {
                    enabled: compToggle.isOn && musicModel.dsp_enabled
                    sliderValue: compSlider.currentValue
                }
                FxResetButton {
                    enabled: compToggle.isOn && musicModel.dsp_enabled
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_compressor()
                }
            }

            // SURROUND
            RowLayout {
                Layout.fillWidth: true
                spacing: 3

                FxToggleBox {
                    id: surrToggle
                    title: "SURROUND"
                    isOn: musicModel.surround_magic_active
                    onToggled: musicModel.toggleStdSurround()
                }

                FxSliderBox {
                    id: surrSlider
                    enabled: surrToggle.isOn && musicModel.dsp_enabled
                    controlValue: musicModel.surround_width
                    onSliderChanged: val => musicModel.setStdSurroundWidth(val * 2.0)
                }
                FxValueBox {
                    enabled: surrToggle.isOn && musicModel.dsp_enabled
                    sliderValue: surrSlider.currentValue
                }
                FxResetButton {
                    enabled: surrToggle.isOn && musicModel.dsp_enabled
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_surround()
                }
            }

            // MONO - STEREO
            RowLayout {
                Layout.fillWidth: true
                spacing: 3

                FxToggleBox {
                    id: monoToggle
                    title: "MONO - STEREO"
                    isOn: musicModel.mono_active
                    onToggled: musicModel.toggleStdStereoWidth()
                }

                FxSliderBox {
                    id: monoSlider
                    enabled: monoToggle.isOn && musicModel.dsp_enabled
                    controlValue: musicModel.mono_width
                    onSliderChanged: val => musicModel.setStdStereoWidthAmount(val)
                }
                FxValueBox {
                    enabled: monoToggle.isOn && musicModel.dsp_enabled
                    sliderValue: monoSlider.controlValue
                }
                FxResetButton {
                    enabled: monoToggle.isOn && musicModel.dsp_enabled
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_stereo_width()
                }
            }

            // MIDDLE CLARITY
            RowLayout {
                Layout.fillWidth: true
                spacing: 3

                FxToggleBox {
                    id: midToggle
                    title: "MIDDLE CLARITY"
                    isOn: musicModel.middle_active
                    onToggled: musicModel.toggleStdMiddleClarity()
                }

                FxSliderBox {
                    id: midSlider
                    enabled: midToggle.isOn
                    controlValue: musicModel.middle_amount
                    onSliderChanged: val => musicModel.setStdMiddleClarityAmount(val)
                }
                FxValueBox {
                    enabled: midToggle.isOn
                    sliderValue: midSlider.controlValue
                }
                FxResetButton {
                    enabled: midToggle.isOn
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_middle_clarity()
                }
            }

            // STEREO ENHANCE
            RowLayout {
                Layout.fillWidth: true
                spacing: 3

                FxToggleBox {
                    id: stereoEnhToggle
                    title: "STEREO ENHANCER"
                    isOn: musicModel.stereo_active
                    onToggled: musicModel.toggleStdStereoEnhance()
                }

                FxSliderBox {
                    id: stereoSlider
                    enabled: stereoEnhToggle.isOn
                    controlValue: musicModel.stereo_amount
                    onSliderChanged: val => musicModel.setStdStereoEnhanceAmount(val)
                }
                FxValueBox {
                    enabled: stereoEnhToggle.isOn
                    sliderValue: stereoSlider.controlValue
                }
                FxResetButton {
                    enabled: stereoEnhToggle.isOn
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_stereo_enhance()
                }
            }

            // HEADPHONE CROSSFEED
            RowLayout {
                Layout.fillWidth: true
                spacing: 3

                FxToggleBox {
                    id: crossfeedToggle
                    title: "CROSSFEED"
                    isOn: musicModel.crossfeed_active
                    onToggled: musicModel.toggleStdCrossfeed()
                }

                FxSliderBox {
                    id: crossfeedSlider
                    enabled: crossfeedToggle.isOn
                    controlValue: musicModel.crossfeed_amount
                    onSliderChanged: val => musicModel.setStdCrossfeedAmount(val)
                }
                FxValueBox {
                    enabled: crossfeedToggle.isOn
                    sliderValue: crossfeedSlider.controlValue
                }
                FxResetButton {
                    enabled: crossfeedToggle.isOn
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_crossfeed()
                }
            }

            // CRYSTALIZER - 3 box layout
            RowLayout {
                Layout.fillWidth: true
                spacing: 3

                FxToggleBox {
                    id: crystalToggle
                    title: "CRYSTALIZER"
                    isOn: musicModel.crystal_magic_active
                    onToggled: musicModel.toggleStdCrystalizer()
                }

                FxSliderBox {
                    id: crystalAmtSlider
                    enabled: crystalToggle.isOn
                    controlValue: musicModel.crystal_amount
                    onSliderChanged: val => musicModel.set_crystalizer_amount(val)
                }
                FxValueBox {
                    enabled: crystalToggle.isOn
                    sliderValue: crystalAmtSlider.controlValue
                }
                FxResetButton {
                    enabled: crystalToggle.isOn
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_crystalizer()
                }
            }

            // BASS BOOSTER - mode buttons with amount
            RowLayout {
                Layout.fillWidth: true
                spacing: 3

                FxToggleBox {
                    id: bassToggle
                    title: "BASS BOOSTER"
                    isOn: musicModel.bass_magic_active
                    onToggled: musicModel.toggleStdBassBooster()
                }

                BassModeSelector {
                    id: bassModeSelector
                    boxEnabled: bassToggle.isOn && musicModel.dsp_enabled
                    Layout.fillWidth: true
                    onModeChanged: mode => {
                        var freqs = [50, 60, 90, 150];
                        musicModel.setStdBassCutoff(freqs[mode]);
                    }
                }

                FxBassAmountBox {
                    boxEnabled: bassToggle.isOn && musicModel.dsp_enabled
                    currentValue: musicModel.bass_gain
                    onValueChanged: val => musicModel.setStdBassGain(val)
                }

                FxResetButton {
                    enabled: bassToggle.isOn && musicModel.dsp_enabled
                    useNoArgReset: true
                    onResetNoArg: musicModel.reset_std_bass()
                }
            }

            // PITCH SHIFTER
            RowLayout {
                Layout.fillWidth: true
                spacing: 3

                FxToggleBox {
                    id: pitchToggle
                    title: "PITCH SHIFTER"
                    isOn: musicModel.pitch_active
                    onToggled: !musicModel.dsp_enabled ? {} : musicModel.toggleStdPitch()
                }

                FxPitchSliderBox {
                    id: pitchSlider
                    enabled: pitchToggle.isOn
                    controlValue: musicModel.pitch_semitones
                    onSliderChanged: val => musicModel.setStdPitchSemitones(val)
                }
                FxValueBox {
                    enabled: pitchToggle.isOn
                    sliderValue: pitchSlider.controlValue
                    showSemitones: true
                }
                FxResetButton {
                    enabled: pitchToggle.isOn
                    defaultValue: 0.0
                    sliderValue: pitchSlider.controlValue
                    onReset: val => musicModel.setStdPitchSemitones(val)
                }
            }
        }
    }

    // Toggle box - name with toggle at beginning
    component FxToggleBox: Rectangle {
        id: rootItem
        property string title: ""
        property bool isOn: false
        signal toggled

        Layout.fillWidth: false
        Layout.preferredWidth: 150
        Layout.preferredHeight: 20
        color: theme.colormap.dspfxbg
        radius: 4
        antialiasing: false

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 4
            anchors.rightMargin: 4
            spacing: 0

            Text {
                text: isOn ? '󰔡' : '󰨙'
                font.family: symbols.name
                font.pixelSize: 16
                color: isOn ? theme.colormap.dspfxhover : theme.colormap.dspfxsubtext
                Layout.preferredWidth: 30
                MouseArea {
                    id: toggleIconArea
                    anchors.fill: parent
                    onClicked: rootItem.toggled()
                }
            }

            Text {
                text: title
                font.family: kodeMono.name
                font.pixelSize: 11
                color: isOn ? theme.colormap.dspfxtext : theme.colormap.dspfxsubtext
                Layout.preferredWidth: 160
                elide: Text.ElideRight
                MouseArea {
                    anchors.fill: parent
                    onClicked: rootItem.toggled()
                }
            }
        }
    }

    // Slider content - label + slider only
    component FxSliderBox: Rectangle {
        id: rootItem
        property real controlValue: 0.0
        property real currentValue: controlValue
        property string leftLabel: ""
        signal sliderChanged(real val)

        onControlValueChanged: {
            if (sld && !sld.pressed) {
                sld.value = controlValue;
                rootItem.currentValue = controlValue;
            }
        }

        Layout.fillWidth: true
        Layout.preferredHeight: 20
        color: theme.colormap.dspfxbg
        radius: 4
        antialiasing: false

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 6
            anchors.rightMargin: 6
            spacing: 3

            Text {
                text: leftLabel
                font.family: kodeMono.name
                font.pixelSize: 11
                color: theme.colormap.dspfxsubtext
                visible: leftLabel !== ""
            }

            Slider {
                id: sld
                Layout.fillWidth: true
                Layout.fillHeight: true
                from: 0.0
                to: 1.0
                stepSize: 0.01
                value: rootItem.controlValue
                onValueChanged: rootItem.currentValue = sld.value
                onMoved: rootItem.sliderChanged(sld.value)

                WheelHandler {
                    target: sld
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                    orientation: Qt.Vertical
                    onWheel: function (event) {
                        var step = 0.05;
                        var delta = event.angleDelta.y > 0 ? step : -step;
                        var newVal = Math.max(0.0, Math.min(1.0, sld.value + delta));
                        sld.value = newVal;
                        rootItem.sliderChanged(newVal);
                    }
                }

                background: Rectangle {
                    x: sld.leftPadding
                    y: sld.topPadding + sld.availableHeight / 2 - height / 2
                    width: sld.availableWidth
                    height: 4
                    radius: 2
                    color: theme.colormap.dspfxsliderbg
                    Rectangle {
                        width: sld.visualPosition * parent.width
                        height: 4
                        radius: 2
                        color: theme.colormap.dspfxslider
                    }
                }
                handle: Rectangle {
                    x: sld.leftPadding + sld.visualPosition * (sld.availableWidth - 10)
                    y: sld.topPadding + sld.availableHeight / 2 - 5
                    width: 10
                    height: 10
                    radius: 5
                    color: theme.colormap.dspfxhandle
                }
            }
        }
    }

    // Slider with value combined - 4 box layout
    component FxSliderValueBox: Rectangle {
        id: rootItem
        property real controlValue: 0.0
        property real currentValue: controlValue
        property bool showHz: false
        property bool showKhz: false
        property real hzMin: 0.0
        property real hzMax: 10000.0
        signal sliderChanged(real val)

        onControlValueChanged: {
            if (svdSld && !svdSld.pressed) {
                svdSld.value = controlValue;
                rootItem.currentValue = controlValue;
            }
        }

        Layout.fillWidth: true
        Layout.preferredHeight: 20
        color: theme.colormap.dspfxbg
        radius: 4
        antialiasing: false

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 6
            anchors.rightMargin: 6
            spacing: 3

            Slider {
                id: svdSld
                Layout.fillWidth: true
                Layout.fillHeight: true
                from: 0.0
                to: 1.0
                stepSize: 0.01
                value: rootItem.controlValue
                onValueChanged: rootItem.currentValue = svdSld.value
                onMoved: rootItem.sliderChanged(svdSld.value)

                WheelHandler {
                    target: svdSld
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                    orientation: Qt.Vertical
                    onWheel: function (event) {
                        var step = 0.05;
                        var delta = event.angleDelta.y > 0 ? step : -step;
                        var newVal = Math.max(0.0, Math.min(1.0, svdSld.value + delta));
                        svdSld.value = newVal;
                        rootItem.sliderChanged(newVal);
                    }
                }

                background: Rectangle {
                    x: svdSld.leftPadding
                    y: svdSld.topPadding + svdSld.availableHeight / 2 - height / 2
                    width: svdSld.availableWidth
                    height: 4
                    radius: 2
                    color: theme.colormap.dspfxsliderbg
                    Rectangle {
                        width: svdSld.visualPosition * parent.width
                        height: 4
                        radius: 2
                        color: theme.colormap.dspfxslider
                    }
                }
                handle: Rectangle {
                    x: svdSld.leftPadding + svdSld.visualPosition * (svdSld.availableWidth - 10)
                    y: svdSld.topPadding + svdSld.availableHeight / 2 - 5
                    width: 10
                    height: 10
                    radius: 5
                    color: theme.colormap.dspfxhandle
                }
            }

            Text {
                text: {
                    if (showHz) {
                        var freq = hzMin + (controlValue * (hzMax - hzMin));
                        return Math.round(freq) + " Hz";
                    } else if (showKhz) {
                        var freq = (hzMin + (controlValue * (hzMax - hzMin))) / 1000;
                        return freq.toFixed(1) + " kHz";
                    } else {
                        return Math.round(controlValue * 100) + "%";
                    }
                }
                font.family: sansSerif.name
                font.pixelSize: 11
                color: theme.colormap.dspfxsubtext
                Layout.preferredWidth: 60
            }
        }
    }

    // Bass mode button - just label
    component FxBassModeButton: Rectangle {
        id: rootItem
        property string modeLabel: ""
        property bool isActive: false
        signal clicked

        Layout.fillWidth: true
        Layout.preferredHeight: 20
        color: theme.colormap.dspfxbg
        radius: 4
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: modeLabel
            font.family: kodeMono.name
            font.pixelSize: 11
            font.bold: isActive
            color: isActive ? theme.colormap.dspfxtext : theme.colormap.dspfxsubtext
        }

        MouseArea {
            anchors.fill: parent
            onClicked: rootItem.clicked()
        }
    }

    // Bass mode selector with state
    component BassModeSelector: Item {
        id: bassModeRoot
        property int selectedMode: 2
        property bool boxEnabled: true
        signal modeChanged(int mode)

        Layout.fillWidth: true
        Layout.preferredHeight: 20

        RowLayout {
            anchors.fill: parent
            spacing: 3
            enabled: bassModeRoot.boxEnabled

            FxBassModeButton {
                modeLabel: "Deep"
                isActive: bassModeRoot.selectedMode === 0
                onClicked: {
                    bassModeRoot.selectedMode = 0;
                    bassModeRoot.modeChanged(0);
                }
            }
            FxBassModeButton {
                modeLabel: "Soft"
                isActive: bassModeRoot.selectedMode === 1
                onClicked: {
                    bassModeRoot.selectedMode = 1;
                    bassModeRoot.modeChanged(1);
                }
            }
            FxBassModeButton {
                modeLabel: "Punch"
                isActive: bassModeRoot.selectedMode === 2
                onClicked: {
                    bassModeRoot.selectedMode = 2;
                    bassModeRoot.modeChanged(2);
                }
            }
            FxBassModeButton {
                modeLabel: "Warm"
                isActive: bassModeRoot.selectedMode === 3
                onClicked: {
                    bassModeRoot.selectedMode = 3;
                    bassModeRoot.modeChanged(3);
                }
            }
        }
    }

    // Editable amount box for bass
    component FxBassAmountBox: Rectangle {
        id: rootItem
        property real currentValue: 0.0
        property real minValue: 0.0
        property real maxValue: 12.0
        property bool boxEnabled: true
        signal valueChanged(real val)

        Layout.preferredWidth: 60
        Layout.preferredHeight: 20
        color: theme.colormap.dspfxbg
        radius: 4
        antialiasing: false
        opacity: boxEnabled ? 1.0 : 0.5

        state: "display"

        Text {
            id: displayText
            anchors.centerIn: parent
            text: Math.round(rootItem.currentValue / rootItem.maxValue * 100) + "%"
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.dspfxsubtext
            visible: rootItem.state === "display"
        }

        TextInput {
            id: inputField
            anchors.centerIn: parent
            width: 35
            horizontalAlignment: TextInput.AlignHCenter
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.dspfxtext
            visible: rootItem.state === "edit"
            validator: IntValidator {
                bottom: 0
                top: 100
            }
            onAccepted: {
                var val = parseInt(text);
                if (!isNaN(val)) {
                    val = Math.max(0, Math.min(100, val));
                    rootItem.currentValue = val / 100 * rootItem.maxValue;
                    rootItem.valueChanged(rootItem.currentValue);
                }
                rootItem.state = "display";
            }
            onActiveFocusChanged: {
                if (!activeFocus) {
                    rootItem.state = "display";
                }
            }
        }

        MouseArea {
            id: hoverArea
            anchors.fill: parent
            hoverEnabled: true
            onEntered: displayText.color = theme.colormap.dspfxtext
            onExited: displayText.color = theme.colormap.dspfxsubtext
            onClicked: rootItem.state = "display"
            onDoubleClicked: {
                inputField.text = Math.round(rootItem.currentValue / rootItem.maxValue * 100);
                rootItem.state = "edit";
                inputField.forceActiveFocus();
                inputField.selectAll();
            }
            onWheel: event => {
                var delta = event.angleDelta.y > 0 ? 0.5 : -0.5;
                var newVal = Math.max(rootItem.minValue, Math.min(rootItem.maxValue, rootItem.currentValue + delta));
                rootItem.currentValue = newVal;
                rootItem.valueChanged(newVal);
            }
        }
    }

    // Dual value box: "X% | YkHz"
    component FxValueBox2: Rectangle {
        id: rootItem
        property real percentValue: 0.0
        property real freqValue: 0.0
        property real hzMin: 0.0
        property real hzMax: 10000.0
        property bool showKhz: false

        Layout.preferredWidth: 60
        Layout.preferredHeight: 20
        color: theme.colormap.dspfxbg
        radius: 4
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: {
                var pct = Math.round(percentValue * 100) + "%";
                var freq = hzMin + (freqValue * (hzMax - hzMin));
                if (showKhz) {
                    freq = (freq / 1000).toFixed(1) + " kHz";
                } else {
                    freq = Math.round(freq) + " Hz";
                }
                return pct + " | " + freq;
            }
            font.family: sansSerif.name
            font.pixelSize: 10
            color: theme.colormap.dspfxsubtext
        }
    }

    // Value display box
    component FxValueBox: Rectangle {
        id: rootItem
        property real sliderValue: 0.0
        property bool showHz: false
        property real hzMin: 0.0
        property real hzMax: 10000.0
        property bool showSemitones: false

        Layout.preferredWidth: 60
        Layout.preferredHeight: 20
        color: theme.colormap.dspfxbg
        radius: 4
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: {
                if (showHz) {
                    var freq = hzMin + (sliderValue * (hzMax - hzMin));
                    return Math.round(freq) + " Hz";
                } else if (showSemitones) {
                    if (sliderValue === 0)
                        return "0 ST";
                    return (sliderValue > 0 ? "+" : "") + Math.round(sliderValue) + " ST";
                } else {
                    return Math.round(sliderValue * 100) + "%";
                }
            }
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.dspfxsubtext
        }
    }

    // Reset button box
    component FxResetButton: Rectangle {
        id: rootItem
        property real defaultValue: 0.0
        property real sliderValue: 0.0
        property bool showHz: false
        property real hzMin: 0.0
        property real hzMax: 10000.0
        property bool useNoArgReset: false
        signal reset(real val)
        signal resetNoArg

        Layout.preferredWidth: 24
        Layout.preferredHeight: 20
        color: theme.colormap.dspfxbg
        radius: 4
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: '󰜉'
            font.family: symbols.name
            font.pixelSize: 12
            color: theme.colormap.dspfxsubtext
        }

        MouseArea {
            anchors.fill: parent
            onClicked: {
                if (rootItem.useNoArgReset) {
                    rootItem.resetNoArg();
                } else {
                    var resetVal = rootItem.defaultValue;
                    rootItem.reset(resetVal);
                }
            }
        }
    }

    // Pitch slider box - special with center marker
    component FxPitchSliderBox: Rectangle {
        id: rootItem
        property real controlValue: 0.0
        property real currentValue: controlValue
        signal sliderChanged(real val)

        onControlValueChanged: {
            if (pitchSld && !pitchSld.pressed) {
                pitchSld.value = controlValue;
                rootItem.currentValue = controlValue;
            }
        }

        Layout.fillWidth: true
        Layout.preferredHeight: 20
        color: theme.colormap.dspfxbg
        radius: 4
        antialiasing: false

        Slider {
            id: pitchSld
            anchors.fill: parent
            anchors.margins: 6
            from: -12.0
            to: 12.0
            stepSize: 1.0
            value: rootItem.controlValue
            onValueChanged: rootItem.currentValue = pitchSld.value
            onMoved: {
                var v = pitchSld.value;
                if (Math.abs(v) < 0.5)
                    v = 0.0;
                rootItem.sliderChanged(v);
            }

            WheelHandler {
                target: pitchSld
                acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                orientation: Qt.Vertical
                onWheel: function (event) {
                    var step = 1.0;
                    var delta = event.angleDelta.y > 0 ? step : -step;
                    var newVal = Math.max(-12.0, Math.min(12.0, pitchSld.value + delta));
                    if (Math.abs(newVal) < 0.5)
                        newVal = 0.0;
                    pitchSld.value = newVal;
                    rootItem.sliderChanged(newVal);
                }
            }

            background: Rectangle {
                x: pitchSld.leftPadding
                y: pitchSld.topPadding + pitchSld.availableHeight / 2 - height / 2
                width: pitchSld.availableWidth
                height: 4
                radius: 2
                color: theme.colormap.dspfxsliderbg

                Rectangle {
                    width: 2
                    height: 8
                    anchors.centerIn: parent
                    color: theme.colormap.dspfxsubtext
                    opacity: 0.5
                }

                Rectangle {
                    anchors.verticalCenter: parent.verticalCenter
                    height: 4
                    radius: 2
                    color: theme.colormap.dspfxslider
                    x: pitchSld.visualPosition >= 0.5 ? parent.width / 2 : pitchSld.visualPosition * parent.width
                    width: Math.abs(pitchSld.visualPosition - 0.5) * parent.width
                }
            }
            handle: Rectangle {
                x: pitchSld.leftPadding + pitchSld.visualPosition * (pitchSld.availableWidth - 10)
                y: pitchSld.topPadding + pitchSld.availableHeight / 2 - 5
                width: 10
                height: 10
                radius: 5
                color: theme.colormap.dspfxhandle
            }
        }
    }
}
