/* --- LOONIX-TUNES qml/ui/preferences/SettingHeader.qml --- */

import QtQuick
import QtQuick.Layouts

ColumnLayout {
    property string title: "HEADER"
    Layout.fillWidth: true
    spacing: 5

    Text {
        text: title
        color: theme.colormap["playeraccent"]
        font.family: kodeMono.name
        font.pixelSize: 11
        font.bold: true
        font.letterSpacing: 1
    }

    Rectangle {
        Layout.fillWidth: true
        height: 1
        color: theme.colormap["graysolid"]
        opacity: 0.3
    }
}