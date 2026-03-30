/* --- LOONIX-TUNES qml/ui/preferences/SettingTab.qml --- */

import QtQuick
import QtQuick.Layouts

Rectangle {
    id: tabRoot
    property string text: "Tab"
    property string icon: "󰋊" // Default icon
    property bool isActive: false
    signal clicked()

    Layout.fillWidth: true
    height: 36
    color: isActive ? theme.colormap["bgmain"] : (mouseArea.containsMouse ? "#1AFFFFFF" : "transparent")
    radius: 4

    // Aksen garis kiri kalau lagi aktif
    Rectangle {
        width: 3; height: parent.height - 12
        anchors.verticalCenter: parent.verticalCenter
        anchors.left: parent.left
        radius: 2
        color: theme.colormap["playeraccent"]
        visible: isActive
    }

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: tabRoot.text === "" ? 0 : 15
        anchors.rightMargin: tabRoot.text === "" ? 0 : 0
        spacing: tabRoot.text === "" ? 0 : 12

        Text {
            text: tabRoot.icon
            font.family: symbols.name // Pake font icon lo
            font.pixelSize: 16
            color: isActive ? theme.colormap["playeraccent"] : theme.colormap["playersubtext"]
            Layout.alignment: Qt.AlignHCenter
        }

        Text {
            text: tabRoot.text
            font.family: kodeMono.name
            font.pixelSize: 12
            font.bold: isActive
            color: isActive ? theme.colormap["playertitle"] : theme.colormap["playersubtext"]
            Layout.fillWidth: true
            visible: tabRoot.text !== ""
        }
    }

    MouseArea {
        id: mouseArea
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: tabRoot.clicked()
    }
}