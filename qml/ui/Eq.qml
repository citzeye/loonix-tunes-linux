/* --- LOONIX-TUNES qml/ui/EqPopup.qml --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Popup {
    id: eqRoot
    width: 500
    height: implicitHeight
    modal: true
    focus: true
    closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside

    background: Rectangle {
        color: theme.colormap.bgmain
        border.color: theme.colormap.eqborder
        border.width: 1
        radius: 4
        antialiasing: false
    }

    contentItem: Item {
        id: eqContentItem
        anchors.fill: parent

        readonly property int activeWidth: width - 16

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
                newNames.push(name !== "" ? name : "User " + (i+1));
            }
            eqContentItem.userPresets = newNames;
        }

        Component.onCompleted: {
            loadDefaultPresets()
            refreshUserPresetNames()
            activePresetIndex = musicModel.get_active_preset_index()
            if (activePresetIndex >= 0) {
                loadPresetByIndex(activePresetIndex)
            }
        }

        property int activePresetIndex: -1

        onActivePresetIndexChanged: {
            musicModel.set_active_preset_index(activePresetIndex)
        }

        function loadPresetByIndex(index) {
            activePresetIndex = index
            var gains
            var macroVal = 0
            var dryVal = 100
            var wetVal = 100
            if (index >= 0 && index < 6) {
                gains = eqContentItem.defaultPresetValues[index]
            } else if (index >= 6 && index < 12) {
                var slot = index - 6
                gains = musicModel.get_user_eq_gains(slot)
                macroVal = musicModel.get_user_eq_macro(slot)
                dryVal = musicModel.get_user_eq_dry(slot)
                wetVal = musicModel.get_user_eq_wet(slot)
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
            musicModel.set_eq_dry(dryVal)
            musicModel.set_eq_wet(wetVal)
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
            musicModel.set_eq_dry(100)
            musicModel.set_eq_wet(100)
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 8
            spacing: 8

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 120
                color: theme.colormap.bgoverlay
                radius: 4
                border.color: theme.colormap.eqborder

                RowLayout {
                    anchors.top: parent.top
                    anchors.bottom: parent.bottom
                    anchors.topMargin: 4
                    anchors.bottomMargin: 4
                    anchors.horizontalCenter: parent.horizontalCenter
                    spacing: 8

                    ColumnLayout {
                        Layout.preferredWidth: 28
                        Layout.fillHeight: true
                        spacing: 2

                        Text {
                            Layout.alignment: Qt.AlignHCenter
                            text: Math.round(gainSlider.value)
                            color: gainSlider.pressed ? theme.colormap.eqgain : theme.colormap.playersubtext
                            font.family: sansSerif.name
                            font.pixelSize: 10
                        }

                        Item {
                            Layout.alignment: Qt.AlignHCenter
                            Layout.fillHeight: true
                            Layout.preferredWidth: 20

                            Slider {
                                id: gainSlider
                                anchors.fill: parent
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
                                    width: 3; height: parent.height; radius: 1.5; color: "#111"
                                    Rectangle {
                                        width: parent.width; y: gainSlider.visualPosition * parent.height
                                        height: parent.height - y; color: theme.colormap.eqgain; radius: 1.5; opacity: 0.6
                                    }
                                }
                                handle: Rectangle {
                                    anchors.horizontalCenter: parent.horizontalCenter
                                    y: gainSlider.topPadding + gainSlider.visualPosition * (gainSlider.availableHeight - height)
                                    width: 10; height: 10; radius: 5; color: gainSlider.pressed ? theme.colormap.playerhover : theme.colormap.eqgain
                                    border.color: theme.colormap.eqborder; border.width: 1
                                }
                            }

                            MouseArea {
                                anchors.fill: parent
                                hoverEnabled: true
                                acceptedButtons: Qt.NoButton
                                onWheel: (wheel) => {
                                    let delta = wheel.angleDelta.y > 0 ? 1 : -1
                                    let clampedNewValue = Math.max(-20, Math.min(20, gainSlider.value + delta))
                                    let actualDelta = clampedNewValue - gainSlider.value

                                    if (actualDelta !== 0) {
                                        gainSlider.value = clampedNewValue
                                        for (let i = 0; i < 10; ++i) {
                                            let slider = eqRepeater.itemAt(i).children[0].children[1]
                                            let newVal = Math.max(-20, Math.min(20, slider.value + actualDelta))
                                            slider.value = newVal
                                            musicModel.set_eq_band(i, newVal)
                                        }
                                    }
                                }
                            }
                        }

                        Text {
                            Layout.alignment: Qt.AlignHCenter
                            text: "FADER"
                            color: theme.colormap.playersubtext
                            font.family: kodeMono.name
                            font.pixelSize: 9
                        }
                    }

                    RowLayout {
                        Layout.fillHeight: true
                        spacing: 2

                        Repeater {
                            id: eqRepeater
                            model: 10
                            delegate: Item {
                                Layout.preferredWidth: 28
                                Layout.fillHeight: true

                                ColumnLayout {
                                    anchors.fill: parent
                                    spacing: 2

                                    Text {
                                        Layout.alignment: Qt.AlignHCenter
                                        text: Math.round(innerSlider.value)
                                        color: innerSlider.pressed ? theme.colormap.eqslider : theme.colormap.playersubtext
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
                                            color: "#111"

                                            Rectangle {
                                                width: parent.width
                                                y: innerSlider.visualPosition * parent.height
                                                height: parent.height - y
                                                color: theme.colormap.eqslider
                                                radius: 1.5
                                                opacity: 0.6
                                            }
                                        }

                                        handle: Rectangle {
                                            anchors.horizontalCenter: parent.horizontalCenter
                                            y: innerSlider.topPadding + innerSlider.visualPosition * (innerSlider.availableHeight - height)
                                            width: 10; height: 10; radius: 5
                                            color: innerSlider.pressed ? theme.colormap.playerhover : theme.colormap.eqslider
                                            border.color: theme.colormap.eqborder
                                            border.width: 1
                                        }
                                    }

                                    Text {
                                        Layout.alignment: Qt.AlignHCenter
                                        text: eqContentItem.freqLabels[index]
                                        color: theme.colormap.playersubtext
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

                    ColumnLayout {
                        Layout.fillHeight: true
                        spacing: 2

                        Text {
                            Layout.alignment: Qt.AlignHCenter
                            text: "MIX"
                            color: theme.colormap.eqmix
                            font.family: kodeMono.name
                            font.pixelSize: 10
                            font.bold: true
                        }

                        RowLayout {
                            Layout.fillHeight: true
                            spacing: 2

                            ColumnLayout {
                                Layout.preferredWidth: 20
                                Layout.fillHeight: true
                                spacing: 2

                                Text {
                                    Layout.alignment: Qt.AlignHCenter
                                    text: Math.round(drySlider.value)
                                    color: drySlider.pressed ? theme.colormap.eqmix : theme.colormap.playersubtext
                                    font.family: sansSerif.name
                                    font.pixelSize: 10
                                }

                                Slider {
                                    id: drySlider
                                    Layout.alignment: Qt.AlignHCenter
                                    Layout.fillHeight: true
                                    Layout.preferredWidth: 16
                                    orientation: Qt.Vertical
                                    from: 0; to: 100
                                    value: musicModel.eq_dry
                                    onMoved: musicModel.set_eq_dry(drySlider.value)
                                    stepSize: 1
                                    padding: 0

                                    background: Rectangle {
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        width: 3
                                        height: parent.height
                                        radius: 1.5
                                        color: "#111"

                                        Rectangle {
                                            width: parent.width
                                            y: drySlider.visualPosition * parent.height
                                            height: parent.height - y
                                            color: theme.colormap.eqmix
                                            radius: 1.5
                                            opacity: 0.6
                                        }
                                    }

                                    handle: Rectangle {
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        y: drySlider.topPadding + drySlider.visualPosition * (drySlider.availableHeight - height)
                                        width: 10; height: 10; radius: 5
                                        color: drySlider.pressed ? theme.colormap.playerhover : theme.colormap.eqmix
                                        border.color: theme.colormap.eqborder
                                        border.width: 1
                                    }

                                    MouseArea {
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        acceptedButtons: Qt.NoButton
                                        onWheel: function(wheel) {
                                            let delta = wheel.angleDelta.y > 0 ? 1 : -1
                                            let newVal = Math.max(0, Math.min(100, drySlider.value + delta))
                                            drySlider.value = newVal
                                            musicModel.set_eq_dry(newVal)
                                        }
                                    }
                                }

                                Text {
                                    Layout.alignment: Qt.AlignHCenter
                                    text: "DRY"
                                    color: theme.colormap.playersubtext
                                    font.family: kodeMono.name
                                    font.pixelSize: 10
                                }
                            }

                            ColumnLayout {
                                Layout.preferredWidth: 28
                                Layout.fillHeight: true
                                spacing: 2

                                Text {
                                    Layout.alignment: Qt.AlignHCenter
                                    text: Math.round(wetSlider.value)
                                    color: wetSlider.pressed ? theme.colormap.eqmix : theme.colormap.playersubtext
                                    font.family: sansSerif.name
                                    font.pixelSize: 10
                                }

                                Slider {
                                    id: wetSlider
                                    Layout.alignment: Qt.AlignHCenter
                                    Layout.fillHeight: true
                                    Layout.preferredWidth: 20
                                    orientation: Qt.Vertical
                                    from: 0; to: 100
                                    value: musicModel.eq_wet
                                    onMoved: musicModel.set_eq_wet(wetSlider.value)
                                    stepSize: 1
                                    padding: 0

                                    background: Rectangle {
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        width: 3
                                        height: parent.height
                                        radius: 1.5
                                        color: "#111"

                                        Rectangle {
                                            width: parent.width
                                            y: wetSlider.visualPosition * parent.height
                                            height: parent.height - y
                                            color: theme.colormap.eqmix
                                            radius: 1.5
                                            opacity: 0.6
                                        }
                                    }

                                    handle: Rectangle {
                                        anchors.horizontalCenter: parent.horizontalCenter
                                        y: wetSlider.topPadding + wetSlider.visualPosition * (wetSlider.availableHeight - height)
                                        width: 10; height: 10; radius: 5
                                        color: wetSlider.pressed ? theme.colormap.playerhover : theme.colormap.eqmix
                                        border.color: theme.colormap.eqborder
                                        border.width: 1
                                    }

                                    MouseArea {
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        acceptedButtons: Qt.NoButton
                                        onWheel: function(wheel) {
                                            let delta = wheel.angleDelta.y > 0 ? 1 : -1
                                            let newVal = Math.max(0, Math.min(100, wetSlider.value + delta))
                                            wetSlider.value = newVal
                                            musicModel.set_eq_wet(newVal)
                                        }
                                    }
                                }

                                Text {
                                    Layout.alignment: Qt.AlignHCenter
                                    text: "WET"
                                    color: theme.colormap.playersubtext
                                    font.family: kodeMono.name
                                    font.pixelSize: 10
                                }
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                Layout.preferredHeight: 22
                spacing: 6

                Button {
                    id: eqOnOffBtn
                    Layout.fillWidth: true
                    Layout.preferredWidth: 1
                    Layout.fillHeight: true

                    onClicked: {
                        musicModel.set_eq_enabled(!musicModel.eq_enabled)
                    }

                    background: Rectangle {
                        color: eqOnOffBtn.hovered ? theme.colormap.eqslider : theme.colormap.bgoverlay
                        border.color: theme.colormap.eqborder
                        radius: 2
                    }

                    contentItem: Text {
                        text: musicModel.eq_enabled ? "ON" : "OFF"
                        font.family: kodeMono.name
                        font.pixelSize: 10
                        font.bold: true

                        color: eqOnOffBtn.hovered ? "black" :
                               (musicModel.eq_enabled ? theme.colormap.tabhover : theme.colormap.playersubtext)

                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }

                Button {
                    id: resetBtn
                    Layout.fillWidth: true
                    Layout.preferredWidth: 1
                    Layout.fillHeight: true

                    onClicked: {
                        eqContentItem.resetEQ()
                    }

                    background: Rectangle {
                        color: resetBtn.hovered ? theme.colormap.eqslider : theme.colormap.bgoverlay
                        border.color: theme.colormap.eqborder
                        radius: 2
                    }

                    contentItem: Text {
                        text: "RESET"
                        font.family: kodeMono.name
                        font.pixelSize: 10
                        font.bold: true
                        color: resetBtn.hovered ? "black" : theme.colormap.playersubtext
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }

                Button {
                    id: saveBtn
                    Layout.fillWidth: true
                    Layout.preferredWidth: 1
                    Layout.fillHeight: true

                    onClicked: {
                        nameInput.text = ""
                        saveEqDialog.open()
                    }

                    background: Rectangle {
                        color: saveBtn.hovered ? theme.colormap.eqslider : theme.colormap.bgoverlay
                        border.color: theme.colormap.eqborder
                        radius: 2
                    }

                    contentItem: Text {
                        text: "SAVE AS"
                        font.family: kodeMono.name
                        font.pixelSize: 10
                        font.bold: true
                        color: saveBtn.hovered ? "black" : theme.colormap.playersubtext
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                }
            }

            GridLayout {
                columns: width > 450 ? 6 : 4
                Layout.fillWidth: true
                columnSpacing: 4
                rowSpacing: 4

                Repeater {
                    model: eqContentItem.defaultPresets.concat(eqContentItem.userPresets)
                    delegate: Button {
                        id: pBtn
                        property bool isActive: eqContentItem.activePresetIndex === index
                        Layout.fillWidth: true
                        Layout.preferredHeight: 20
                        contentItem: Text {
                            text: modelData
                            font.family: kodeMono.name
                            font.pixelSize: 10
                            color: pBtn.isActive ? theme.colormap.tabhover : (pBtn.hovered ? "black" : theme.colormap.playersubtext)
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                        background: Rectangle {
                            color: pBtn.hovered ? theme.colormap.eqslider : theme.colormap.bgoverlay
                            border.color: pBtn.isActive ? theme.colormap.tabhover : theme.colormap.eqborder
                            radius: 2
                        }

                        onClicked: {
                            eqContentItem.loadPresetByIndex(index)
                        }
                    }
                }
            }
        }

        Popup {
            id: saveEqDialog
    width: 450
            height: 160
            x: (eqContentItem.width - width) / 2
            y: (eqContentItem.height - height) / 2
            modal: false
            focus: true
            closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside

            background: Rectangle {
                color: theme.colormap.bgmain
                border.color: theme.colormap.eqslider
                border.width: 1
                radius: 6
                Rectangle {
                    anchors.fill: parent
                    color: "transparent"
                    border.color: "#30000000"
                    border.width: 2
                    radius: 6
                    z: -1
                    anchors.margins: -1
                }
            }

            contentItem: ColumnLayout {
                anchors.fill: parent
                anchors.margins: 15
                spacing: 12

                Text {
                    text: "SAVE PRESET"
                    font.family: kodeMono.name
                    font.pixelSize: 12
                    font.bold: true
                    color: theme.colormap.eqslider
                }

                ComboBox {
                    id: slotCombo
                    Layout.fillWidth: true
                    Layout.preferredHeight: 32
                    model: [
                        "1: " + eqContentItem.userPresets[0],
                        "2: " + eqContentItem.userPresets[1],
                        "3: " + eqContentItem.userPresets[2],
                        "4: " + eqContentItem.userPresets[3],
                        "5: " + eqContentItem.userPresets[4],
                        "6: " + eqContentItem.userPresets[5]
                    ]

                    contentItem: Text {
                        leftPadding: 10
                        text: slotCombo.displayText
                        font.family: kodeMono.name
                        font.pixelSize: 10
                        color: theme.colormap.tabtext
                        verticalAlignment: Text.AlignVCenter
                    }

                    background: Rectangle {
                        color: theme.colormap.eqsliderbg
                        border.color: theme.colormap.eqborder
                        radius: 2
                    }

                    delegate: ItemDelegate {
                        width: slotCombo.width
                        hoverEnabled: true

                        contentItem: Text {
                            text: modelData
                            font.family: kodeMono.name
                            font.pixelSize: 10
                            verticalAlignment: Text.AlignVCenter
                            color: (hovered || highlighted) ? theme.colormap.eqslider : theme.colormap.tabtext

                            Behavior on color { ColorAnimation { duration: 100 } }
                        }

                        background: Rectangle {
                            color: theme.colormap.eqsliderbg
                        }
                    }

                    popup: Popup {
                        y: slotCombo.height - 1
                        width: slotCombo.width
                        implicitHeight: contentItem.implicitHeight
                        padding: 1

                        contentItem: ListView {
                            clip: true
                            implicitHeight: contentHeight
                            model: slotCombo.popup.visible ? slotCombo.delegateModel : null
                            currentIndex: slotCombo.highlightedIndex
                        }

                        background: Rectangle {
                            color: theme.colormap.eqsliderbg
                            border.color: theme.colormap.eqborder
                            radius: 2
                        }
                    }
                }

                TextField {
                    id: nameInput
                    Layout.fillWidth: true
                    Layout.preferredHeight: 32
                    maximumLength: 10
                    placeholderText: "NAME..."
                    placeholderTextColor: theme.colormap.graysolid
                    color: theme.colormap.tabtext
                    font.family: kodeMono.name
                    font.pixelSize: 11
                    verticalAlignment: Text.AlignVCenter
                    leftPadding: 10
                    selectByMouse: true

                    onAccepted: {
                        musicModel.save_user_eq(slotCombo.currentIndex, text, gainSlider.value, musicModel.eq_dry, musicModel.eq_wet)
                        refreshUserPresetNames()
                        saveEqDialog.close()
                    }

                    background: Rectangle {
                        color: theme.colormap.bgoverlay
                        border.color: nameInput.activeFocus ? theme.colormap.eqslider : theme.colormap.eqborder
                        radius: 2
                    }
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 16

                    Text {
                        text: "CANCEL"
                        font.family: kodeMono.name
                        font.pixelSize: 10
                        color: cancelMA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
                        MouseArea {
                            id: cancelMA
                            anchors.fill: parent
                            hoverEnabled: true
                            onClicked: saveEqDialog.close()
                        }
                    }

                    Item { Layout.fillWidth: true }

                    Text {
                        text: "SAVE"
                        font.family: kodeMono.name
                        font.pixelSize: 10
                        color: saveMA.containsMouse ? theme.colormap.playlisticon : theme.colormap.eqslider
                        MouseArea {
                            id: saveMA
                            anchors.fill: parent
                            hoverEnabled: true
                            onClicked: {
                                musicModel.save_user_eq(slotCombo.currentIndex, nameInput.text, gainSlider.value, musicModel.eq_dry, musicModel.eq_wet)
                                refreshUserPresetNames()
                                saveEqDialog.close()
                            }
                        }
                    }
                }
            }
        }
    }
}
