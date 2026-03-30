/* --- LOONIX-TUNES qml/ui/preferences/SettingFooter.qml --- */

import QtQuick
import QtQuick.Layouts

ColumnLayout {
    id: footerRoot
    property bool isCompact: false
    signal closeClicked()

    Layout.fillWidth: true
    spacing: 5

    Text {
        text: "← Close"
        color: footerMA.containsMouse
            ? theme.colormap["playeraccent"]
            : theme.colormap["playersubtext"]
        font.family: kodeMono.name
        font.pixelSize: 11
        font.bold: true
        font.letterSpacing: 1

        MouseArea {
            id: footerMA
            anchors.fill: parent
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            onClicked: footerRoot.closeClicked()
        }
    }
}
