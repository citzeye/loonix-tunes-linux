/* --- LOONIX-TUNES qml/ui/preferences/SettingButton.qml --- */

import QtQuick
import QtQuick.Layouts

Rectangle {
    property string text: "Button"
    property bool enabled: true
    property bool accent: false
    property bool compact: false
    
    Layout.preferredWidth: compact ? implicitWidth : -1
    Layout.fillWidth: !compact
    implicitWidth: btnText.implicitWidth + (compact ? 16 : 24)
    implicitHeight: 30
    
    radius: 4
    color: {
        if (!enabled) return theme.colormap["graysolid"]
        if (accent) return theme.colormap["playeraccent"]
        return "transparent"
    }
    border.color: {
        if (!enabled) return theme.colormap["graysolid"]
        if (accent) return theme.colormap["playeraccent"]
        return theme.colormap["graysolid"]
    }
    border.width: accent ? 0 : 1
    opacity: enabled ? (btnArea.containsMouse ? 0.8 : 1.0) : 0.5

    Text {
        id: btnText
        anchors.centerIn: parent
        text: parent.text
        font.family: kodeMono.name
        font.pixelSize: 11
        font.bold: accent
        color: accent ? theme.colormap["bgmain"] : theme.colormap["playlisttext"]
    }
    
    MouseArea {
        id: btnArea
        anchors.fill: parent
        enabled: parent.enabled
        cursorShape: Qt.PointingHandCursor
        hoverEnabled: true
    }
}
