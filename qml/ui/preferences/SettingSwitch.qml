/* --- LOONIX-TUNES qml/ui/preferences/SettingSwitch.qml --- */

import QtQuick
import QtQuick.Layouts

RowLayout {
    property string label: "Setting Name"
    property string description: ""
    property bool checked: false
    signal toggled()

    Layout.fillWidth: true
    spacing: 15

    ColumnLayout {
        Layout.fillWidth: true
        spacing: 2
        Text {
            text: label
            color: theme.colormap["playlisttext"]
            font.family: kodeMono.name
            font.pixelSize: 13
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
        }
        Text {
            text: description
            color: theme.colormap["playersubtext"]
            font.family: kodeMono.name
            font.pixelSize: 11
            visible: description !== ""
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
        }
    }

    // Custom Switch UI
    Rectangle {
        Layout.alignment: Qt.AlignVCenter
        width: 40; height: 22
        radius: 11
        color: checked ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]

        Rectangle {
            width: 18; height: 18
            radius: 9
            color: theme.colormap["bgmain"]
            y: 2
            x: checked ? parent.width - width - 2 : 2
            Behavior on x { NumberAnimation { duration: 150; easing.type: Easing.InOutQuad } }
        }

        MouseArea {
            anchors.fill: parent
            cursorShape: Qt.PointingHandCursor
            onClicked: { checked = !checked; toggled() }
        }
    }
}
