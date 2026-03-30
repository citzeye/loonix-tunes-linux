/* --- LOONIX-TUNES qml/ui/preferences/SettingDropdown.qml --- */

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    id: dropdownRoot

    property string label: ""
    property string description: ""
    property var model: []
    property int currentIndex: 0
    signal optionSelected(int index, string value)

    readonly property bool isNarrow: dropdownRoot.width < 350

    Layout.fillWidth: true
    spacing: 4
    Layout.bottomMargin: 8

    ColumnLayout {
        Layout.fillWidth: true
        spacing: 0

        Text {
            text: label
            color: theme.colormap["playlisttext"]
            font.family: kodeMono.name
            font.pixelSize: 12
            font.bold: true
            Layout.fillWidth: true
        }

        Text {
            text: description
            color: theme.colormap["playersubtext"]
            font.family: kodeMono.name
            font.pixelSize: 10
            Layout.fillWidth: true
            wrapMode: Text.NoWrap
            elide: Text.ElideRight
            opacity: 0.6
            visible: description !== ""
        }
    }

    ComboBox {
        id: control
        model: dropdownRoot.model
        currentIndex: dropdownRoot.currentIndex

        Layout.fillWidth: isNarrow
        Layout.preferredWidth: isNarrow ? -1 : 180
        Layout.preferredHeight: 28
        Layout.alignment: isNarrow ? Qt.AlignLeft : Qt.AlignRight

        onActivated: (index) => dropdownRoot.optionSelected(index, currentText)

        background: Rectangle {
            color: theme.colormap["bgoverlay"]
            border.color: control.hovered ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
            border.width: 1
            radius: 4
            antialiasing: false
        }

        contentItem: Text {
            leftPadding: 8
            text: control.displayText
            font.family: kodeMono.name
            font.pixelSize: 11
            color: theme.colormap["playertitle"]
            verticalAlignment: Text.AlignVCenter
        }

        indicator: Text {
            x: control.width - width - 8
            y: (control.height - height) / 2
            text: "" // icon dropdown 
            font.family: "Nerd Font"
            font.pixelSize: 14
            color: theme.colormap["playersubtext"]
        }

        popup: Popup {
            y: control.height + 2
            width: control.width
            implicitHeight: contentItem.implicitHeight
            padding: 2
            background: Rectangle {
                color: theme.colormap["bgoverlay"]
                border.color: theme.colormap["graysolid"]
                radius: 4
                antialiasing: false
            }
            contentItem: ListView {
                clip: true
                implicitHeight: contentHeight
                model: control.popup.visible ? control.delegateModel : null
            }
        }

        delegate: ItemDelegate {
            width: control.width - 4
            height: 26
            contentItem: Text {
                text: modelData
                color: highlighted ? theme.colormap["bgmain"] : theme.colormap["playlisttext"]
                font.family: kodeMono.name
                font.pixelSize: 11
                verticalAlignment: Text.AlignVCenter
                leftPadding: 8
            }
            background: Rectangle {
                radius: 3
                color: highlighted ? theme.colormap["playeraccent"] : "transparent"
            }
        }
    }
}
