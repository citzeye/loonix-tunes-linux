/* --- loonix-tunes/qml/ui/Dsp.qml --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Popup {
    id: dspRoot
    width: 500
    height: 442
    modal: true
    closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside

    background: Rectangle {
        color: theme.colormap.dspbg
        border.color: theme.colormap.dspborder
        border.width: 1
        radius: 2
    }

    contentItem: ColumnLayout {
        id: dspContent
        anchors.margins: 0
        spacing: 3

        // EQ Properties & Functions
        readonly property var freqLabels: ["31", "62", "125", "250", "500", "1k", "2k", "4k", "8k", "16k"]
        property var defaultPresets: []
        property var defaultPresetValues: []
        property var userPresets: ["USER 1", "USER 2", "USER 3", "USER 4", "USER 5", "USER 6"]

        function loadDefaultPresets() {
            var count = musicModel.get_eq_preset_count();
            var names = [];
            var values = [];
            for (var i = 0; i < count; i++) {
                names.push(musicModel.get_eq_preset_name(i));
                values.push(musicModel.get_eq_preset_gains(i));
            }
            defaultPresets = names;
            defaultPresetValues = values;
        }

        function refreshUserPresetNames() {
            var newNames = [];
            for (var i = 0; i < 6; i++) {
                let name = musicModel.get_user_preset_name(i);
                newNames.push(name !== "" ? name : "User " + (i + 1));
            }
            dspContent.userPresets = newNames;
        }

        Component.onCompleted: {
            loadDefaultPresets();
            refreshUserPresetNames();
        }

        function loadPresetByIndex(index) {
            if (index < 0 || index >= 12) {
                return;
            }
            musicModel.load_preset(index);
        }

        // EQ Section
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 100
            color: theme.colormap.dspeqbg

            RowLayout {
                anchors.fill: parent

                Item {
                    Layout.fillWidth: true
                }

                GridLayout {
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter 
                    width: 280
                    columns: 12
                    rows: 3
                    rowSpacing: 3
                    columnSpacing: 2

                    // Row 1: Numbers (atas) - connected to sliders
                    EqNumberBox {
                        id: numPreamp
                        displayText: eqPreamp.currentValue > 0 ? "+" + Math.round(eqPreamp.currentValue) : "" + Math.round(eqPreamp.currentValue)
                    }
                    EqNumberBox {
                        id: num31
                        displayText: Math.round(eq31.currentValue)
                    }
                    EqNumberBox {
                        id: num62
                        displayText: Math.round(eq62.currentValue)
                    }
                    EqNumberBox {
                        id: num125
                        displayText: Math.round(eq125.currentValue)
                    }
                    EqNumberBox {
                        id: num250
                        displayText: Math.round(eq250.currentValue)
                    }
                    EqNumberBox {
                        id: num500
                        displayText: Math.round(eq500.currentValue)
                    }
                    EqNumberBox {
                        id: num1k
                        displayText: Math.round(eq1k.currentValue)
                    }
                    EqNumberBox {
                        id: num2k
                        displayText: Math.round(eq2k.currentValue)
                    }
                    EqNumberBox {
                        id: num4k
                        displayText: Math.round(eq4k.currentValue)
                    }
                    EqNumberBox {
                        id: num8k
                        displayText: Math.round(eq8k.currentValue)
                    }
                    EqNumberBox {
                        id: num16k
                        displayText: Math.round(eq16k.currentValue)
                    }
                    EqNumberBox {
                        id: numFader
                        displayText: Math.round((eqFader.currentValue + 20) * 2.5) + "%"
                    }

                    // Row 2: Sliders (tengah) - bound to musicModel.eqBands via reactive property
                    EqSliderBox {
                        id: eqPreamp
                        controlValue: musicModel.get_preamp_gain()
                        onSliderChanged: val => musicModel.set_preamp_gain(val)
                    }
                    EqSliderBox {
                        id: eq31
                        controlValue: musicModel.eqBands[0]
                        onSliderChanged: val => musicModel.set_eq_band(0, val)
                    }
                    EqSliderBox {
                        id: eq62
                        controlValue: musicModel.eqBands[1]
                        onSliderChanged: val => musicModel.set_eq_band(1, val)
                    }
                    EqSliderBox {
                        id: eq125
                        controlValue: musicModel.eqBands[2]
                        onSliderChanged: val => musicModel.set_eq_band(2, val)
                    }
                    EqSliderBox {
                        id: eq250
                        controlValue: musicModel.eqBands[3]
                        onSliderChanged: val => musicModel.set_eq_band(3, val)
                    }
                    EqSliderBox {
                        id: eq500
                        controlValue: musicModel.eqBands[4]
                        onSliderChanged: val => musicModel.set_eq_band(4, val)
                    }
                    EqSliderBox {
                        id: eq1k
                        controlValue: musicModel.eqBands[5]
                        onSliderChanged: val => musicModel.set_eq_band(5, val)
                    }
                    EqSliderBox {
                        id: eq2k
                        controlValue: musicModel.eqBands[6]
                        onSliderChanged: val => musicModel.set_eq_band(6, val)
                    }
                    EqSliderBox {
                        id: eq4k
                        controlValue: musicModel.eqBands[7]
                        onSliderChanged: val => musicModel.set_eq_band(7, val)
                    }
                    EqSliderBox {
                        id: eq8k
                        controlValue: musicModel.eqBands[8]
                        onSliderChanged: val => musicModel.set_eq_band(8, val)
                    }
                    EqSliderBox {
                        id: eq16k
                        controlValue: musicModel.eqBands[9]
                        onSliderChanged: val => musicModel.set_eq_band(9, val)
                    }
                    EqSliderBox {
                        id: eqFader
                        controlValue: musicModel.fader_offset
                        onSliderChanged: val => musicModel.set_fader(val)
                    }

                    // Row 3: Names (bawah)
                    EqNameBox {
                        nameLabel: "AMP"
                    }
                    EqNameBox {
                        nameLabel: "31"
                    }
                    EqNameBox {
                        nameLabel: "62"
                    }
                    EqNameBox {
                        nameLabel: "125"
                    }
                    EqNameBox {
                        nameLabel: "250"
                    }
                    EqNameBox {
                        nameLabel: "500"
                    }
                    EqNameBox {
                        nameLabel: "1k"
                    }
                    EqNameBox {
                        nameLabel: "2k"
                    }
                    EqNameBox {
                        nameLabel: "4k"
                    }
                    EqNameBox {
                        nameLabel: "8k"
                    }
                    EqNameBox {
                        nameLabel: "16k"
                    }
                    EqNameBox {
                        nameLabel: "FADER"
                    }
                }

                Item {
                    Layout.fillWidth: true
                }
            }
        }

        // FX Section
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 250
            color: theme.colormap.dspfxbg

            ColumnLayout {
                anchors.fill: parent
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
                        showDbCompressor: true
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
                        controlValue: musicModel.surround_width / 2.0
                        onSliderChanged: val => {
                            musicModel.setStdSurroundWidth(val * 2.0);
                        }
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
                        sliderValue: monoSlider.currentValue
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
                        sliderValue: midSlider.currentValue
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
                        sliderValue: stereoSlider.currentValue
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
                        sliderValue: crossfeedSlider.currentValue
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
                        sliderValue: crystalAmtSlider.currentValue
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
                        boxEnabled: musicModel.dsp_enabled
                        onToggled: musicModel.toggleStdPitch()
                    }

                    FxPitchSliderBox {
                        id: pitchSlider
                        enabled: pitchToggle.isOn
                        controlValue: musicModel.pitch_semitones
                        onSliderChanged: val => musicModel.setStdPitchSemitones(val)
                    }
                    FxValueBox {
                        enabled: pitchToggle.isOn
                        sliderValue: pitchSlider.currentValue
                        showSemitones: true
                    }
                    FxResetButton {
                        enabled: pitchToggle.isOn
                        defaultValue: 0.0
                        sliderValue: pitchSlider.controlValue
                        onReset: val => musicModel.setStdPitchSemitones(val)
                    }
                }

                // REVERB
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: reverbToggle
                        title: "REVERB"
                        isOn: musicModel.reverb_active
                        onToggled: musicModel.toggleStdReverb()
                    }

                    ReverbModeSelector {
                        id: reverbModeSelector
                        boxEnabled: reverbToggle.isOn && musicModel.dsp_enabled
                        Layout.fillWidth: true
                    }

                    ReverbAmountBox {
                        id: reverbAmountEditor
                        boxEnabled: reverbToggle.isOn && musicModel.dsp_enabled
                        currentValue: musicModel.reverb_amount
                        onValueChanged: val => musicModel.set_reverb_amount(Math.round(val))
                    }

                    FxResetButton {
                        enabled: reverbToggle.isOn && musicModel.dsp_enabled
                        useNoArgReset: true
                        onResetNoArg: {
                            musicModel.set_reverb_mode(1);
                            musicModel.set_reverb_amount(50);
                        }
                    }
                }
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
                property bool isActive: musicModel.active_preset_index === index
                Layout.fillWidth: true
                Layout.preferredHeight: 20
                contentItem: Text {
                    text: modelData
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    color: defBtn.hovered ? "#00ffa2" : "#57caab"
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
                background: Rectangle {
                    radius: 2
                    color: theme.colormap.dspgridbg
                    border.width: 1
                    border.color: defBtn.isActive ? theme.colormap.dspeqpresetactive : theme.colormap.dspborder
                }
                onClicked: {
                    dspContent.loadPresetByIndex(index);
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
                property bool isActive: musicModel.active_preset_index === index + 6
                Layout.fillWidth: true
                Layout.preferredHeight: 20
                contentItem: Text {
                    text: modelData
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    color: pBtn.hovered ? "#00ffa2" : "#57caab"
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
                background: Rectangle {
                    radius: 2
                    color: theme.colormap.dspgridbg
                    border.width: 1
                    border.color: pBtn.isActive ? theme.colormap.dspeqpresetactive : theme.colormap.dspborder
                }
                onClicked: {
                    dspContent.loadPresetByIndex(index + 6);
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
                dspContent.resetEQ();
            }

            background: Rectangle {
                color: resetBtn.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspgridbg
                border.color: theme.colormap.dspborder
                radius: 2
            }

            contentItem: Text {
                text: "RESET ALL"
                font.family: kodeMono.name
                font.pixelSize: 10
                font.bold: true
                color: resetBtn.hovered ? "black" : theme.colormap.dspfxtext
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
                saveDialog.presetName = "";
                saveDialog.open();
            }

            background: Rectangle {
                color: saveBtn.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspgridbg
                border.color: theme.colormap.dspborder
                radius: 2
            }

            contentItem: Text {
                text: "SAVE AS"
                font.family: kodeMono.name
                font.pixelSize: 10
                font.bold: true
                color: saveBtn.hovered ? "black" : theme.colormap.dspfxtext
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
    }

    // Save Preset Dialog
    component SavePresetDialog: Popup {
        id: saveDialog
        width: 250
        height: 100
        modal: true
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
        anchors.centerIn: parent

        property alias presetName: nameInput.text

        background: Rectangle {
            color: theme.colormap.dspgridbg
            border.color: theme.colormap.dspborder
            border.width: 1
            radius: 2
        }

        ColumnLayout {
            anchors.margins: 10
            spacing: 8
            anchors.fill: parent

            Text {
                text: "SAVE PRESET"
                font.family: kodeMono.name
                font.pixelSize: 12
                font.bold: true
                color: theme.colormap.dspfxtext
            }

            TextInput {
                id: nameInput
                Layout.fillWidth: true
                Layout.preferredHeight: 24
                maximumLength: 10
                font.family: kodeMono.name
                font.pixelSize: 12
                color: theme.colormap.dspfxtext
                verticalAlignment: Text.AlignVCenter
                validator: RegularExpressionValidator {
                    regularExpression: /[a-zA-Z0-9 ]*/
                }

                Rectangle {
                    anchors.fill: parent
                    color: theme.colormap.dspgridbg
                    border.color: theme.colormap.dspborder
                    border.width: 1
                    radius: 2
                    z: -1
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Button {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 24
                    text: "SAVE"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    onClicked: {
                        var result = musicModel.save_user_preset(presetName);
                        if (result >= 0) {
                            dspContent.refreshUserPresetNames();
                            saveDialog.close();
                        }
                    }

                    background: Rectangle {
                        color: parent.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspgridbg
                        border.color: theme.colormap.dspborder
                        radius: 2
                    }

                    contentItem: Text {
                        text: parent.text
                        font: parent.font
                        color: parent.hovered ? "black" : theme.colormap.dspfxtext
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }

                Button {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 24
                    text: "CANCEL"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    onClicked: {
                        saveDialog.close();
                    }

                    background: Rectangle {
                        color: parent.hovered ? theme.colormap.dspeqpresetactive : theme.colormap.dspgridbg
                        border.color: theme.colormap.dspborder
                        radius: 2
                    }

                    contentItem: Text {
                        text: parent.text
                        font: parent.font
                        color: parent.hovered ? "black" : theme.colormap.dspfxtext
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }
            }
        }
    }

    // Toggle box - name with toggle at beginning
    component FxToggleBox: Rectangle {
        id: rootItem
        property string title: ""
        property bool isOn: false
        property bool boxEnabled: true
        signal toggled

        Layout.fillWidth: false
        Layout.preferredWidth: 150
        Layout.preferredHeight: 20
        color: boxEnabled ? theme.colormap.dspgridbg : theme.colormap.dspfxtext + "33"
        radius: 2
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
                color: boxEnabled ? (isOn ? theme.colormap.dspfxhover : theme.colormap.dspfxtext) : theme.colormap.dspfxtext + "66"
                Layout.preferredWidth: 30
                MouseArea {
                    id: toggleIconArea
                    enabled: rootItem.boxEnabled
                    anchors.fill: parent
                    onClicked: rootItem.toggled()
                }
            }

            Text {
                text: title
                font.family: kodeMono.name
                font.pixelSize: 11
                color: boxEnabled ? (isOn ? theme.colormap.dspfxtext : theme.colormap.dspfxtext) : theme.colormap.dspfxtext + "66"
                Layout.preferredWidth: 160
                elide: Text.ElideRight
                MouseArea {
                    enabled: rootItem.boxEnabled
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
        color: theme.colormap.dspgridbg
        radius: 2
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
                color: theme.colormap.dspfxtext
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
            if (svdSld) {
                svdSld.value = controlValue;
                rootItem.currentValue = controlValue;
            }
        }

        Layout.fillWidth: true
        Layout.preferredHeight: 20
        color: theme.colormap.dspgridbg
        radius: 2
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
                color: theme.colormap.dspfxtext
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
        color: theme.colormap.dspgridbg
        radius: 2
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: modeLabel
            font.family: kodeMono.name
            font.pixelSize: 11
            font.bold: isActive
            color: isActive ? theme.colormap.dspfxtext : theme.colormap.dspfxtext
        }

        MouseArea {
            anchors.fill: parent
            onClicked: rootItem.clicked()
        }
    }

    // Bass mode selector with state
    component BassModeSelector: Item {
        id: bassModeRoot
        property int selectedMode: musicModel.bass_mode
        property bool boxEnabled: true

        Layout.fillWidth: true
        Layout.preferredHeight: 20

        RowLayout {
            anchors.fill: parent
            spacing: 3
            enabled: bassModeRoot.boxEnabled

            FxBassModeButton {
                modeLabel: "Deep"
                isActive: bassModeRoot.selectedMode === 0
                onClicked: musicModel.set_bass_mode(0)
            }
            FxBassModeButton {
                modeLabel: "Soft"
                isActive: bassModeRoot.selectedMode === 1
                onClicked: musicModel.set_bass_mode(1)
            }
            FxBassModeButton {
                modeLabel: "Punch"
                isActive: bassModeRoot.selectedMode === 2
                onClicked: musicModel.set_bass_mode(2)
            }
            FxBassModeButton {
                modeLabel: "Warm"
                isActive: bassModeRoot.selectedMode === 3
                onClicked: musicModel.set_bass_mode(3)
            }
        }
    }

    // Reverb mode button - just label
    component FxReverbModeButton: Rectangle {
        id: rootItem
        property string modeLabel: ""
        property bool isActive: false
        signal clicked

        Layout.fillWidth: true
        Layout.preferredHeight: 20
        color: theme.colormap.dspgridbg
        radius: 2
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: modeLabel
            font.family: kodeMono.name
            font.pixelSize: 11
            font.bold: isActive
            color: isActive ? theme.colormap.dspfxtext : theme.colormap.dspfxtext
        }

        MouseArea {
            anchors.fill: parent
            onClicked: rootItem.clicked()
        }
    }

    // Reverb mode selector with state
    component ReverbModeSelector: Item {
        id: reverbModeRoot
        property int selectedMode: musicModel && musicModel.reverb_mode !== undefined ? musicModel.reverb_mode : 0
        property bool boxEnabled: musicModel.reverb_active

        Layout.fillWidth: true
        Layout.preferredHeight: 20

        RowLayout {
            anchors.fill: parent
            spacing: 3
            enabled: reverbModeRoot.boxEnabled

            FxReverbModeButton {
                modeLabel: "Studio"
                isActive: reverbModeRoot.selectedMode === 1
                onClicked: musicModel.set_reverb_mode(1)
            }
            FxReverbModeButton {
                modeLabel: "Stage"
                isActive: reverbModeRoot.selectedMode === 2
                onClicked: musicModel.set_reverb_mode(2)
            }
            FxReverbModeButton {
                modeLabel: "Stadium"
                isActive: reverbModeRoot.selectedMode === 3
                onClicked: musicModel.set_reverb_mode(3)
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
        color: theme.colormap.dspgridbg
        radius: 2
        antialiasing: false

        state: "display"

        Text {
            id: displayText
            anchors.centerIn: parent
            text: Math.round(rootItem.currentValue / rootItem.maxValue * 100) + "%"
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.dspfxtext
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
            onExited: displayText.color = theme.colormap.dspfxtext
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

    // Editable amount box for reverb
    component ReverbAmountBox: Rectangle {
        id: rootItem
        property real currentValue: 0.0
        property real minValue: 0.0
        property real maxValue: 100.0
        property bool boxEnabled: true
        signal valueChanged(real val)

        Layout.preferredWidth: 60
        Layout.preferredHeight: 20
        color: boxEnabled ? theme.colormap.dspfxbg : theme.colormap.dspfxtext + "33"
        radius: 2
        antialiasing: false

        state: "display"

        Text {
            id: displayText
            anchors.centerIn: parent
            text: Math.round(rootItem.currentValue) + "%"
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.dspfxtext
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
                    rootItem.currentValue = val;
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
            onExited: displayText.color = theme.colormap.dspfxtext
            onClicked: rootItem.state = "display"
            onDoubleClicked: {
                inputField.text = Math.round(rootItem.currentValue);
                rootItem.state = "edit";
                inputField.forceActiveFocus();
                inputField.selectAll();
            }
            onWheel: event => {
                var delta = event.angleDelta.y > 0 ? 2.0 : -2.0;
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
        color: theme.colormap.dspgridbg
        radius: 2
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
            color: theme.colormap.dspfxtext
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
        property bool showDbCompressor: false

        Layout.preferredWidth: 60
        Layout.preferredHeight: 20
        color: theme.colormap.dspgridbg
        radius: 2
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: {
                if (showDbCompressor) {
                    var db = -60.0 + (sliderValue * 60.0);
                    return Math.round(db) + " dB";
                } else if (showHz) {
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
            color: theme.colormap.dspfxtext
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
        color: theme.colormap.dspgridbg
        radius: 2
        antialiasing: false

        Text {
            anchors.centerIn: parent
            text: '󰜉'
            font.family: symbols.name
            font.pixelSize: 12
            color: theme.colormap.dspfxtext
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
        color: theme.colormap.dspgridbg
        radius: 2
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
                    color: theme.colormap.dspfxtext
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

    // EQ Number Box - row 1 (atas)
    component EqNumberBox: Rectangle {
        id: rootItem
        property string displayText: "0"

        Layout.preferredWidth: 20
        Layout.fillWidth: false
        Layout.fillHeight: true
        color: "transparent"

        Text {
            anchors.centerIn: parent
            text: rootItem.displayText
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.dspeqtext
        }
    }

    // EQ Slider Box - row 2 (tengah)
    component EqSliderBox: Rectangle {
        id: rootItem
        property real controlValue: 0.0
        property real currentValue: controlValue
        signal sliderChanged(real val)

        onControlValueChanged: {
            if (eqSld && !eqSld.pressed) {
                eqSld.value = controlValue;
                rootItem.currentValue = controlValue;
            }
        }

        Layout.preferredWidth: 20
        Layout.fillWidth: false
        Layout.preferredHeight: 50
        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
        color: "transparent"

        Slider {
            id: eqSld
            anchors.fill: parent
            anchors.margins: 0
            orientation: Qt.Vertical
            from: -20
            to: 20
            stepSize: 1
            value: rootItem.controlValue
            onValueChanged: {
                if (!pressed)
                    rootItem.currentValue = eqSld.value;
            }
            onMoved: rootItem.sliderChanged(eqSld.value)

            background: Rectangle {
                anchors.centerIn: parent
                width: 3
                height: parent.height
                radius: 1.5
                color: theme.colormap.dspeq10sliderbg
                Rectangle {
                    width: parent.width
                    y: eqSld.visualPosition * parent.height
                    height: parent.height - y
                    color: theme.colormap.dspeq10slider
                    radius: 1.5
                }
            }
            handle: Rectangle {
                anchors.horizontalCenter: parent.horizontalCenter
                y: eqSld.visualPosition * (eqSld.availableHeight - height)
                width: 10
                height: 10
                radius: 5
color: eqSld.pressed ? theme.colormap.dspeq10slider : theme.colormap.dspeq10handle
            }
            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.NoButton
                onWheel: function (wheel) {
                    var step = 1;
                    var delta = wheel.angleDelta.y > 0 ? step : -step;
                    var newVal = Math.max(-20, Math.min(20, eqSld.value + delta));
                    eqSld.value = newVal;
                    rootItem.sliderChanged(newVal);
                }
            }
        }
    }

    // EQ Name Box - row 3 (bawah)
    component EqNameBox: Rectangle {
        id: rootItem
        property string nameLabel: ""

        Layout.preferredWidth: 20
        Layout.fillWidth: false
        Layout.fillHeight: true
        color: "transparent"

        Text {
            anchors.centerIn: parent
            text: rootItem.nameLabel
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.dspeqtext
        }
    }

    SavePresetDialog {
        id: saveDialog
    }
}
