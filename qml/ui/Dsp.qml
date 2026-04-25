/* --- loonixtunesv2/qml/ui/Dsp.qml | DSP Panel --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Popup {
    id: dspRoot
    width: 520
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

        function loadPresetByIndex(index) {
            if (index < 0 || index >= 12) {
                return;
            }
            dspModel.load_preset(index);
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
                    width:240
                    columns: 12
                    rows: 3
                    rowSpacing: 2
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

                    // Row 2: Sliders (tengah) - bound to dspModel.eq_bands via reactive property
                    EqSliderBox {
                        id: eqPreamp
                        controlValue: dspModel.get_preamp_gain()
                        onSliderChanged: val => dspModel.set_preamp_gain(val)
                    }
                    EqSliderBox {
                        id: eq31
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[0] : 0
                        onSliderChanged: val => dspModel.set_eq_band(0, val)
                    }
                    EqSliderBox {
                        id: eq62
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[1] : 0
                        onSliderChanged: val => dspModel.set_eq_band(1, val)
                    }
                    EqSliderBox {
                        id: eq125
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[2] : 0
                        onSliderChanged: val => dspModel.set_eq_band(2, val)
                    }
                    EqSliderBox {
                        id: eq250
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[3] : 0
                        onSliderChanged: val => dspModel.set_eq_band(3, val)
                    }
                    EqSliderBox {
                        id: eq500
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[4] : 0
                        onSliderChanged: val => dspModel.set_eq_band(4, val)
                    }
                    EqSliderBox {
                        id: eq1k
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[5] : 0
                        onSliderChanged: val => dspModel.set_eq_band(5, val)
                    }
                    EqSliderBox {
                        id: eq2k
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[6] : 0
                        onSliderChanged: val => dspModel.set_eq_band(6, val)
                    }
                    EqSliderBox {
                        id: eq4k
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[7] : 0
                        onSliderChanged: val => dspModel.set_eq_band(7, val)
                    }
                    EqSliderBox {
                        id: eq8k
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[8] : 0
                        onSliderChanged: val => dspModel.set_eq_band(8, val)
                    }
                    EqSliderBox {
                        id: eq16k
                        controlValue: dspModel.eq_bands && dspModel.eq_bands.length > 0 ? dspModel.eq_bands[9] : 0
                        onSliderChanged: val => dspModel.set_eq_band(9, val)
                    }
                    EqSliderBox {
                        id: eqFader
                        controlValue: dspModel.fader_offset
                        onSliderChanged: val => dspModel.set_fader(val)
                    }

                    // Row 3: Names (bawah)
                    // amp
                    EqNameBox {
                        nameLabel: "󰯫"
                        tooltipText: "Preamp"
                        fontFamily: symbols.name
                        fontSize: 14
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
                    // fader
                    EqNameBox {
                        nameLabel: "󰯺"
                        tooltipText: "Fader"
                        fontFamily: symbols.name
                        fontSize: 14
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
                        isOn: dspModel.compressor_active
                        onToggled: dspModel.toggle_compressor()
                    }

                    FxSliderBox {
                        id: compSlider
                        enabled: compToggle.isOn && dspModel.dsp_enabled
                        controlValue: dspModel.compressor_threshold
                        sliderRange: "db"
                        onSliderChanged: val => dspModel.set_compressor_threshold(val)
                    }
                    FxValueBox {
                        enabled: compToggle.isOn && dspModel.dsp_enabled
                        sliderValue: compSlider.currentValue
                        showDbCompressor: true
                        linkSlider: compSlider
                    }
                    FxResetButton {
                        enabled: compToggle.isOn && dspModel.dsp_enabled
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_compressor()
                    }
                }

                // SURROUND
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: surrToggle
                        title: "SURROUND"
                        isOn: dspModel.surround_active
                        onToggled: dspModel.toggle_surround()
                    }

                    FxSliderBox {
                        id: surrSlider
                        enabled: surrToggle.isOn && dspModel.dsp_enabled
                        controlValue: dspModel.surround_width / 2.0
                        onSliderChanged: val => {
                            dspModel.set_surround_width(val * 2.0);
                        }
                    }
                    FxValueBox {
                        enabled: surrToggle.isOn && dspModel.dsp_enabled
                        sliderValue: surrSlider.currentValue
                        linkSlider: surrSlider
                    }
                    FxResetButton {
                        enabled: surrToggle.isOn && dspModel.dsp_enabled
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_surround()
                    }
                }

                // MONO - STEREO
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: monoToggle
                        title: "MONO - STEREO"
                        isOn: dspModel.mono_active
                        onToggled: dspModel.toggle_stereo_width()
                    }

                    FxSliderBox {
                        id: monoSlider
                        enabled: monoToggle.isOn && dspModel.dsp_enabled
                        controlValue: dspModel.mono_width
                        onSliderChanged: val => dspModel.set_stereo_width_amount(val)
                    }
                    FxValueBox {
                        enabled: monoToggle.isOn && dspModel.dsp_enabled
                        sliderValue: monoSlider.currentValue
                        linkSlider: monoSlider
                    }
                    FxResetButton {
                        enabled: monoToggle.isOn && dspModel.dsp_enabled
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_stereo_width()
                    }
                }

                // MIDDLE CLARITY
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: midToggle
                        title: "MIDDLE CLARITY"
                        isOn: dspModel.middle_active
                        onToggled: dspModel.toggle_middle_clarity()
                    }

                    FxSliderBox {
                        id: midSlider
                        enabled: midToggle.isOn
                        controlValue: dspModel.middle_amount
                        onSliderChanged: val => dspModel.set_middle_clarity_amount(val)
                    }
                    FxValueBox {
                        enabled: midToggle.isOn
                        sliderValue: midSlider.currentValue
                        linkSlider: midSlider
                    }
                    FxResetButton {
                        enabled: midToggle.isOn
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_middle_clarity()
                    }
                }

                // STEREO ENHANCE
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: stereoEnhToggle
                        title: "STEREO ENHANCER"
                        isOn: dspModel.stereo_active
                        onToggled: dspModel.toggle_stereo_enhance()
                    }

                    FxSliderBox {
                        id: stereoSlider
                        enabled: stereoEnhToggle.isOn
                        controlValue: dspModel.stereo_amount
                        onSliderChanged: val => dspModel.set_stereo_enhance_amount(val)
                    }
                    FxValueBox {
                        enabled: stereoEnhToggle.isOn
                        sliderValue: stereoSlider.currentValue
                        linkSlider: stereoSlider
                    }
                    FxResetButton {
                        enabled: stereoEnhToggle.isOn
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_stereo_enhance()
                    }
                }

                // HEADPHONE CROSSFEED
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: crossfeedToggle
                        title: "CROSSFEED"
                        isOn: dspModel.crossfeed_active
                        onToggled: dspModel.toggle_crossfeed()
                    }

                    FxSliderBox {
                        id: crossfeedSlider
                        enabled: crossfeedToggle.isOn
                        controlValue: dspModel.crossfeed_amount
                        onSliderChanged: val => dspModel.set_crossfeed_amount(val)
                    }
                    FxValueBox {
                        enabled: crossfeedToggle.isOn
                        sliderValue: crossfeedSlider.currentValue
                        linkSlider: crossfeedSlider
                    }
                    FxResetButton {
                        enabled: crossfeedToggle.isOn
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_crossfeed()
                    }
                }

                // CRYSTALIZER - 3 box layout
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: crystalToggle
                        title: "CRYSTALIZER"
                        isOn: dspModel.crystal_active
                        onToggled: dspModel.toggle_crystalizer()
                    }

                    FxSliderBox {
                        id: crystalAmtSlider
                        enabled: crystalToggle.isOn
                        controlValue: dspModel.crystal_amount
                        onSliderChanged: val => dspModel.set_crystalizer_amount(val)
                    }
                    FxValueBox {
                        enabled: crystalToggle.isOn
                        sliderValue: crystalAmtSlider.currentValue
                        linkSlider: crystalAmtSlider
                    }
                    FxResetButton {
                        enabled: crystalToggle.isOn
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_crystalizer()
                    }
                }

                // BASS BOOSTER - mode buttons with amount
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: bassToggle
                        title: "BASS BOOSTER"
                        isOn: dspModel.bass_active
                        onToggled: dspModel.toggle_bass_booster()
                    }

                    BassModeSelector {
                        id: bassModeSelector
                        boxEnabled: bassToggle.isOn && dspModel.dsp_enabled
                        Layout.fillWidth: true
                    }

                    FxBassAmountBox {
                        id: bassGainBox
                        boxEnabled: bassToggle.isOn && dspModel.dsp_enabled
                        currentValue: dspModel.bass_gain
                        onValueChanged: val => dspModel.set_bass_gain(val)

                        Connections {
                            target: dspModel
                            function onBass_gain_changed() {
                                bassGainBox.currentValue = dspModel.bass_gain
                            }
                        }
                    }

                    FxResetButton {
                        enabled: bassToggle.isOn && dspModel.dsp_enabled
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_bass()
                    }
                }

                // PITCH SHIFTER
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: pitchToggle
                        title: "PITCH SHIFTER"
                        isOn: dspModel.pitch_active
                        boxEnabled: true
                        onToggled: dspModel.toggle_pitch()
                    }

                    FxPitchSliderBox {
                        id: pitchSlider
                        enabled: pitchToggle.isOn
                        controlValue: dspModel.pitch_semitones
                        onSliderChanged: val => dspModel.set_pitch_semitones(val)
                    }
                    FxValueBox {
                        enabled: pitchToggle.isOn
                        sliderValue: pitchSlider.currentValue
                        showSemitones: true
                        linkSlider: pitchSlider
                    }
                    FxResetButton {
                        enabled: pitchToggle.isOn
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_pitch()
                    }
                }

                // REVERB
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 3

                    FxToggleBox {
                        id: reverbToggle
                        title: "REVERB"
                        isOn: dspModel.reverb_active
                        onToggled: dspModel.toggle_reverb()
                    }

                    ReverbModeSelector {
                        id: reverbModeSelector
                        boxEnabled: reverbToggle.isOn && dspModel.dsp_enabled
                        Layout.fillWidth: true
                    }

                    ReverbAmountBox {
                        id: reverbAmountEditor
                        boxEnabled: reverbToggle.isOn && dspModel.dsp_enabled
                        currentValue: dspModel.reverb_amount
                        onValueChanged: val => dspModel.set_reverb_amount(Math.round(val))

                        Connections {
                            target: dspModel
                            function onReverb_amount_changed() {
                                reverbAmountEditor.currentValue = dspModel.reverb_amount
                            }
                        }
                    }

                    FxResetButton {
                        enabled: reverbToggle.isOn && dspModel.dsp_enabled
                        useNoArgReset: true
                        onResetNoArg: dspModel.reset_reverb()
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
            model: dspModel.get_eq_preset_count()
            delegate: Button {
                id: defBtn
                property bool isActive: dspModel.active_preset_index === index
                Layout.fillWidth: true
                Layout.preferredHeight: 20
                contentItem: Text {
                    text: dspModel.get_eq_preset_name(index)
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    color: defBtn.isActive ? theme.colormap.dsptextactive : (defBtn.hovered ? theme.colormap.dsptexthover : theme.colormap.dsptext)
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
                background: Rectangle {
                    radius: 2
                    color: theme.colormap.dspgridbg
                    border.width: 1
                    border.color: defBtn.isActive ? theme.colormap.dsptextactive : theme.colormap.dspborder
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
            model: 6
            delegate: Button {
                id: pBtn
                property bool isActive: dspModel.active_preset_index === index + 6
                Layout.fillWidth: true
                Layout.preferredHeight: 20
                contentItem: Text {
                    text: dspModel.user_preset_names[index]
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    color: pBtn.isActive ? theme.colormap.dsptextactive : (pBtn.hovered ? theme.colormap.dsptexthover : theme.colormap.dsptext)
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
                background: Rectangle {
                    radius: 2
                    color: theme.colormap.dspgridbg
                    border.width: 1
                    border.color: pBtn.isActive ? theme.colormap.dsptextactive : theme.colormap.dspborder
                }
                onClicked: {
                    dspContent.loadPresetByIndex(index + 6);
                }
            }
        }
    }

    // EQ Controls (BYPASS - RESET - SAVE AS)
    RowLayout {
        Layout.fillWidth: true
        Layout.preferredHeight: 20
        spacing: 3

        Button {
            id: bypassBtn
            Layout.fillWidth: true
            Layout.preferredWidth: 1
            Layout.preferredHeight: 20

            onClicked: {
                dspModel.toggle_dsp()
            }

            background: Rectangle {
                color: dspModel.dsp_enabled ? theme.colormap.dspgridbg : theme.colormap.dspgridbg
                border.color: theme.colormap.dspborder
                radius: 2
            }

            contentItem: Text {
                text: dspModel.dsp_enabled ? "DSP ON" : "DSP OFF"
                font.family: kodeMono.name
                font.pixelSize: 10
                font.bold: true
                color: dspModel.dsp_enabled ? theme.colormap.dsptext : theme.colormap.dsptexthover
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }

        Button {
            id: resetBtn
            Layout.fillWidth: true
            Layout.preferredWidth: 1
            Layout.preferredHeight: 20

            onClicked: {
                dspModel.reset_all();
            }

            background: Rectangle {
                color: theme.colormap.dspgridbg
                border.color: theme.colormap.dspborder
                radius: 2
            }

            contentItem: Text {
                text: "RESET ALL"
                font.family: kodeMono.name
                font.pixelSize: 10
                font.bold: true
                color: resetBtn.hovered ? theme.colormap.dsptexthover : theme.colormap.dsptext
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
                color: theme.colormap.dspgridbg
                border.color: theme.colormap.dspborder
                radius: 2
            }

            contentItem: Text {
                text: "SAVE AS"
                font.family: kodeMono.name
                font.pixelSize: 10
                font.bold: true
                color: saveBtn.hovered ? theme.colormap.dsptexthover : theme.colormap.dsptext
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
    }

    // Save Preset Dialog
    component SavePresetDialog: Popup {
        id: saveDialog
        width: 250
        height: 120
        modal: true
        focus: true
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
        anchors.centerIn: parent

        property alias presetName: nameInput.text
        property int selectedSlot: -1

        onOpened: nameInput.forceActiveFocus()

        background: Rectangle {
            color: theme.colormap.dspbg
            border.color: theme.colormap.dspborder
            border.width: 1
            radius: 2
        }

        ColumnLayout {
            anchors.margins: 10
            spacing: 8
            anchors.fill: parent

            // Preset slot selector

            ComboBox {
                id: slotSelector
                model: dspModel.user_preset_names
                currentIndex: -1
                Layout.fillWidth: true
                Layout.preferredHeight: 24
                textRole: "display"
                font.family: kodeMono.name
                font.pixelSize: 12
                onCurrentIndexChanged: {
                    saveDialog.selectedSlot = currentIndex
                    if (currentIndex >= 0) {
                        nameInput.enabled = true
                        nameInput.forceActiveFocus()
                    } else {
                        nameInput.enabled = false
                    }
                }

                // Main button background
                background: Rectangle {
                    color: theme.colormap.dspgridbg
                }

                // Selected text display
                contentItem: Text {
                    leftPadding: 5
                    text: slotSelector.currentIndex >= 0 ? dspModel.user_preset_names[slotSelector.currentIndex] : "Select slot..."
                    font.family: kodeMono.name
                    font.pixelSize: 12
                    color: theme.colormap.dsptext
                    verticalAlignment: Text.AlignVCenter
                    elide: Text.ElideRight
                }

                // Dropdown arrow indicator
                indicator: Text {
                    x: slotSelector.width - width - 10
                    y: (slotSelector.height - height) / 2
                    text: "▼"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    color: theme.colormap.dsptext
                    opacity: slotSelector.pressed ? 0.5 : 1.0
                }

                // Popup dropdown container
                popup: Popup {
                    y: slotSelector.height
                    width: slotSelector.width
                    padding: 2

                    background: Rectangle {
                        color: theme.colormap.dspbg
                        border.color: theme.colormap.dspborder
                        border.width: 1
                        radius: 2
                    }

                    contentItem: ListView {
                        clip: true
                        implicitHeight: contentHeight
                        model: slotSelector.delegateModel
                        currentIndex: slotSelector.highlightedIndex

                        ScrollIndicator.vertical: ScrollIndicator { }
                    }
                }

                // Dropdown list items
                delegate: ItemDelegate {
                    width: slotSelector.width - 4
                    height: 22

                    contentItem: Text {
                        leftPadding: 8
                        text: String(modelData).length > 14 ? String(modelData).substring(0, 14) : String(modelData)
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        color: highlighted ? theme.colormap.dsptextactive : theme.colormap.dsptext
                        verticalAlignment: Text.AlignVCenter
                        elide: Text.ElideRight
                    }
                    background: Rectangle {
                        color: theme.colormap.dspgridbg
                    }
                }
            }

            Connections {
                target: dspModel
                function onUser_presets_changed() {
                    var oldIdx = slotSelector.currentIndex;
                    slotSelector.currentIndex = -1;
                    slotSelector.currentIndex = oldIdx;
                }
            }

            TextInput {
                id: nameInput
                Layout.fillWidth: true
                Layout.preferredHeight: 26
                focus: true
                enabled: true
                color: theme.colormap.dsptext
                font.family: kodeMono.name
                font.pixelSize: 12
                selectByMouse: true
                clip: true
                maximumLength: 10
                verticalAlignment: Text.AlignVCenter
                leftPadding: 5

                onAccepted: {
                    var result = dspModel.save_user_preset(saveDialog.selectedSlot, presetName);
                    if (result >= 0) {
                        saveDialog.close();
                    }
                }

                cursorDelegate: Rectangle {
                    width: 1
                    color: theme.colormap.dsptextactive
                }

                Rectangle {
                    z: -1
                    anchors.fill: parent
                    color: theme.colormap.dspgridbg
                    radius: 2
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Button {
                    Layout.fillWidth: false
                    Layout.preferredWidth: 100
                    Layout.preferredHeight: 24
                    text: "CANCEL"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    onClicked: {
                        saveDialog.close();
                    }

                    background: Rectangle {
                        color: theme.colormap.dspgridbg
                        radius: 2
                    }

                    contentItem: Text {
                        text: parent.text
                        font: parent.font
                        color: parent.hovered ? theme.colormap.dsptexthover : theme.colormap.dsptext
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }

                Button {
                    Layout.fillWidth: false
                    Layout.preferredWidth: 100
                    Layout.preferredHeight: 24
                    text: "SAVE"
                    font.family: kodeMono.name
                    font.pixelSize: 10
                    onClicked: {
                        var result = dspModel.save_user_preset(saveDialog.selectedSlot, presetName);
                        if (result >= 0) {
                            saveDialog.close();
                        }
                    }

                    background: Rectangle {
                        color: theme.colormap.dspgridbg
                        radius: 2
                    }

                    contentItem: Text {
                        text: parent.text
                        font: parent.font
                        color: parent.hovered ? theme.colormap.dsptexthover : theme.colormap.dsptext
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
        color: theme.colormap.dspgridbg
        radius: 2
        antialiasing: false

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 4
            anchors.rightMargin: 4
            spacing: 0

            Text {
                id: iconText
                text: isOn ? '󰔡' : '󰨙'
                font.family: symbols.name
                font.pixelSize: 16
                color: !rootItem.boxEnabled ? theme.colormap.dsptext + "66" :
                       (toggleIconArea.containsMouse ? theme.colormap.dsptexthover :
                       (isOn ? theme.colormap.dspfxicon : theme.colormap.dsptext))
                Layout.preferredWidth: 30

                MouseArea {
                    id: toggleIconArea
                    enabled: rootItem.boxEnabled
                    anchors.fill: parent
                    hoverEnabled: true
                    onClicked: rootItem.toggled()
                }
            }

            Text {
                id: titleText
                text: title
                font.family: kodeMono.name
                font.pixelSize: 11
                color: !rootItem.boxEnabled ? theme.colormap.dsptext + "66" :
                       (titleArea.containsMouse ? theme.colormap.dsptexthover :
                       (isOn ? theme.colormap.dsptextactive : theme.colormap.dsptext))
                Layout.preferredWidth: 160
                elide: Text.ElideRight

                MouseArea {
                    id: titleArea
                    enabled: rootItem.boxEnabled
                    anchors.fill: parent
                    hoverEnabled: true
                    onClicked: rootItem.toggled()
                }
            }
        }
    }

    // Slider content - label + slider only
    component FxSliderBox: Rectangle {
        id: rootItem
        property real controlValue: 0.0
        property alias currentValue: sld.value
        property string leftLabel: ""
        property string sliderRange: "linear" 
        signal sliderChanged(real val)

        Layout.fillWidth: true
        Layout.preferredHeight: 20
        color: theme.colormap.dspgridbg
        radius: 2
        antialiasing: false

        onControlValueChanged: {
            if (sld && !sld.pressed) {
                sld.value = controlValue
                rootItem.currentValue = sld.value
            }
        }

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 6
            anchors.rightMargin: 6
            spacing: 3

            Text {
                text: leftLabel
                font.family: kodeMono.name
                font.pixelSize: 11
                color: theme.colormap.dsptext
                visible: leftLabel !== ""
            }

            Slider {
                id: sld
                Layout.fillWidth: true
                Layout.fillHeight: true
                from: rootItem.sliderRange === "db" ? -60.0 : 0.0
                to: rootItem.sliderRange === "db" ? 0.0 : 1.0
                stepSize: rootItem.sliderRange === "db" ? 1.0 : 0.01
                value: rootItem.controlValue
                onMoved: rootItem.sliderChanged(sld.value)

                WheelHandler {
                    target: sld
                    acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                    orientation: Qt.Vertical
                    onWheel: function (event) {
                        var step = rootItem.sliderRange === "db" ? 1.0 : 0.05;
                        var minVal = rootItem.sliderRange === "db" ? -60.0 : 0.0;
                        var maxVal = rootItem.sliderRange === "db" ? 0.0 : 1.0;
                        var delta = event.angleDelta.y > 0 ? step : -step;
                        var newVal = Math.max(minVal, Math.min(maxVal, sld.value + delta));
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
                id: valText
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
                color: theme.colormap.dsptext
                Layout.preferredWidth: 60
                MouseArea {
                    anchors.fill: parent
                    hoverEnabled: true
                    onEntered: valText.color = theme.colormap.dsptexthover
                    onExited: valText.color = theme.colormap.dsptext
                    onWheel: event => {
                        var step = 0.01;
                        var delta = event.angleDelta.y > 0 ? step : -step;
                        var newVal = Math.max(0.0, Math.min(1.0, controlValue + delta));
                        controlValue = newVal;
                        rootItem.currentValue = newVal;
                        rootItem.sliderChanged(newVal);
                    }
                }
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
            id: modeText
            anchors.centerIn: parent
            text: modeLabel
            font.family: kodeMono.name
            font.pixelSize: 11
            font.bold: isActive
            color: buttonArea.containsMouse ? theme.colormap.dsptexthover : 
                   (isActive ? theme.colormap.dsptextactive : theme.colormap.dsptext)
        }

        MouseArea {
            id: buttonArea
            anchors.fill: parent
            hoverEnabled: true
            onClicked: rootItem.clicked()
        }
    }

    // Bass mode selector with state
    component BassModeSelector: Item {
        id: bassModeRoot
        property int selectedMode: dspModel.bass_mode
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
                onClicked: dspModel.set_bass_mode(0)
            }
            FxBassModeButton {
                modeLabel: "Soft"
                isActive: bassModeRoot.selectedMode === 1
                onClicked: dspModel.set_bass_mode(1)
            }
            FxBassModeButton {
                modeLabel: "Punch"
                isActive: bassModeRoot.selectedMode === 2
                onClicked: dspModel.set_bass_mode(2)
            }
            FxBassModeButton {
                modeLabel: "Warm"
                isActive: bassModeRoot.selectedMode === 3
                onClicked: dspModel.set_bass_mode(3)
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
        id: modeText
        anchors.centerIn: parent
        text: modeLabel
        font.family: kodeMono.name
        font.pixelSize: 11
        font.bold: isActive
        color: !rootItem.parent.parent.boxEnabled ? theme.colormap.dsptext + "66" : 
               (buttonArea.containsMouse ? theme.colormap.dsptexthover : 
               (isActive ? theme.colormap.dsptextactive : theme.colormap.dsptext))
    }

    MouseArea {
        id: buttonArea
        anchors.fill: parent
        hoverEnabled: true
        onClicked: rootItem.clicked()
    }
    }

    // Reverb mode selector with state
    component ReverbModeSelector: Item {
        id: reverbModeRoot
        property int selectedMode: dspModel && dspModel.reverb_mode !== undefined ? dspModel.reverb_mode : 0
        property bool boxEnabled: dspModel.reverb_active

        Layout.fillWidth: true
        Layout.preferredHeight: 20

        RowLayout {
            anchors.fill: parent
            spacing: 3
            enabled: reverbModeRoot.boxEnabled

            FxReverbModeButton {
                modeLabel: "Studio"
                isActive: reverbModeRoot.selectedMode === 1
                onClicked: dspModel.set_reverb_mode(1)
            }
            FxReverbModeButton {
                modeLabel: "Stage"
                isActive: reverbModeRoot.selectedMode === 2
                onClicked: dspModel.set_reverb_mode(2)
            }
            FxReverbModeButton {
                modeLabel: "Stadium"
                isActive: reverbModeRoot.selectedMode === 3
                onClicked: dspModel.set_reverb_mode(3)
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
            color: theme.colormap.dsptext
            visible: rootItem.state === "display"
        }

        TextInput {
            id: inputField
            anchors.centerIn: parent
            width: 35
            horizontalAlignment: TextInput.AlignHCenter
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.dsptext
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
            onEntered: displayText.color = theme.colormap.dsptexthover
            onExited: displayText.color = theme.colormap.dsptext
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
        color: theme.colormap.dspgridbg
        radius: 2
        antialiasing: false

        state: "display"

        Text {
        id: displayText
        anchors.centerIn: parent
        text: Math.round(rootItem.currentValue) + "%"
        font.family: sansSerif.name
        font.pixelSize: 11
        color: !rootItem.boxEnabled ? theme.colormap.dsptext + "66" : theme.colormap.dsptext
        visible: rootItem.state === "display"
    }

        TextInput {
            id: inputField
            anchors.centerIn: parent
            width: 35
            horizontalAlignment: TextInput.AlignHCenter
            font.family: sansSerif.name
            font.pixelSize: 11
            color: theme.colormap.dsptext
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
            onEntered: displayText.color = theme.colormap.dsptexthover
            onExited: displayText.color = theme.colormap.dsptext
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
            color: theme.colormap.dsptext
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
        property var linkSlider: null

        Layout.preferredWidth: 60
        Layout.preferredHeight: 20
        color: theme.colormap.dspgridbg
        radius: 2
        antialiasing: false

        Text {
            id: displayText
            anchors.centerIn: parent
            text: {
                if (showDbCompressor) {
                    return Math.round(sliderValue) + " dB";
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
            color: theme.colormap.dsptext
        }

        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            onEntered: displayText.color = theme.colormap.dsptexthover
            onExited: displayText.color = theme.colormap.dsptext
            onWheel: event => {
                var step = 0.01;
                var delta = event.angleDelta.y > 0 ? step : -step;
                var newVal = Math.max(0.0, Math.min(1.0, sliderValue + delta));
                sliderValue = newVal;
                if (linkSlider) {
                    linkSlider.controlValue = newVal;
                    linkSlider.sliderChanged(newVal);
                }
            }
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
            id: resetIcon
            anchors.centerIn: parent
            text: '󰑓'
            font.family: symbols.name
            font.pixelSize: 12
            color: theme.colormap.dsptext
        }

        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            onEntered: resetIcon.color = theme.colormap.dsptexthover
            onExited: resetIcon.color = theme.colormap.dsptext
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
                    color: theme.colormap.dsptext
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
            color: theme.colormap.dsptext
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
                    radius: 1.5
                    // Murni pakai warna dspeqslider, opacity yang main untuk greyout
                    color: theme.colormap.dspeqslider
                    opacity: dspModel.dsp_enabled ? 1.0 : 0.4
                }
            }
            
            handle: Rectangle {
                anchors.horizontalCenter: parent.horizontalCenter
                y: eqSld.visualPosition * (eqSld.availableHeight - height)
                width: 10
                height: 10
                radius: 5
                // Murni pakai warna dspeqhandle, tanpa ganti warna pas pressed
                color: theme.colormap.dspeqhandle
                opacity: dspModel.dsp_enabled ? 1.0 : 0.4
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
        property string tooltipText: ""
        property string fontFamily: sansSerif.name
        property int fontSize: 11

        Layout.preferredWidth: 20
        Layout.fillWidth: false
        Layout.fillHeight: true
        color: "transparent"

        Text {
            id: labelText
            anchors.centerIn: parent
            text: rootItem.nameLabel
            font.family: rootItem.fontFamily
            font.pixelSize: rootItem.fontSize
            color: theme.colormap.dsptext
            Rectangle {
                visible: ma.containsMouse
                y: -20
                width: labelText.width
                height: 16
                color: "transparent"
                Text {
                    anchors.centerIn: parent
                    text: rootItem.tooltipText
                    font.family: kodeMono.name
                    font.pixelSize: 13
                    color: theme.colormap.dsptexthover
                }
            }
            MouseArea {
                id: ma
                anchors.fill: parent
                hoverEnabled: true
            }
        }
    }

    SavePresetDialog {
        id: saveDialog
    }
}
