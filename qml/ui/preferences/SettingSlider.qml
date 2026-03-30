/* --- LOONIX-TUNES qml/ui/preferences/SettingSlider.qml --- */

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    id: sliderRoot

    property string label: "Slider"
    property string valueText: ""
    property string description: ""
    property real fromValue: 0.0
    property real toValue: 100.0
    property real stepValue: 1.0
    property real currentValue: 50.0
    property real defaultValue: 50.0
    signal moved(real value)
    signal resetToDefault()

    Layout.fillWidth: true
    spacing: 2

    // Label kiri atas
    Text {
        text: sliderRoot.label
        color: theme.colormap["playlisttext"]
        font.family: kodeMono.name
        font.pixelSize: 12
        Layout.fillWidth: true
        wrapMode: Text.WordWrap
    }

    // Slider + Value sejajar
    RowLayout {
        Layout.fillWidth: true
        spacing: 10

        Slider {
            id: slider
            Layout.fillWidth: true
            Layout.preferredHeight: 30
            Layout.maximumWidth: 250
            from: sliderRoot.fromValue
            to: sliderRoot.toValue
            stepSize: sliderRoot.stepValue
            value: sliderRoot.currentValue
            live: true
            onMoved: sliderRoot.moved(value)

            WheelHandler {
                target: slider
                acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                property: "position"
                orientation: Qt.Vertical
                onWheel: function(event) {
                    var step = sliderRoot.stepValue
                    var delta = event.angleDelta.y > 0 ? step : -step
                    var newVal = Math.max(sliderRoot.fromValue, Math.min(sliderRoot.toValue, slider.value + delta))
                    slider.value = newVal
                    sliderRoot.moved(newVal)
                }
            }

            background: Rectangle {
                x: slider.leftPadding
                y: slider.topPadding + slider.availableHeight / 2 - height / 2
                width: slider.availableWidth
                height: 4
                radius: 2
                color: theme.colormap["graysolid"]
                Rectangle {
                    width: slider.visualPosition * parent.width
                    height: 4
                    radius: 2
                    color: theme.colormap["playeraccent"]
                }
            }
            handle: Rectangle {
                x: slider.leftPadding + slider.visualPosition * (slider.availableWidth - width)
                y: slider.topPadding + slider.availableHeight / 2 - height / 2
                width: 14
                height: 14
                radius: 7
                color: slider.pressed ? theme.colormap["playerhover"] : theme.colormap["playeraccent"]
                border.color: theme.colormap["playeraccent"]
            }
        }

        Text {
            text: sliderRoot.valueText
            color: theme.colormap["playeraccent"]
            font.family: kodeMono.name
            font.pixelSize: 12
            font.bold: true
        }

        Item { Layout.fillWidth: true }
    }

    Text {
        text: sliderRoot.description
        color: theme.colormap["playersubtext"]
        font.family: kodeMono.name
        font.pixelSize: 10
        wrapMode: Text.WordWrap
        Layout.fillWidth: true
        visible: text !== ""
    }
}
