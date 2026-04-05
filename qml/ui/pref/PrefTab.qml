/* --- LOONIX-TUNES qml/ui/pref/PrefTab.qml --- */

import QtQuick
import QtQuick.Layouts

Item {
    property string text: "Tab"
    property string icon: "󰋊"
    property bool isActive: false
    signal clicked()
    width: 120
    height: 16

    Text {
        text: parent.text
        font.family: kodeMono.name
        font.pixelSize: 12
        color: parent.isActive ? theme.colormap.playeraccent : (parent.isHovered ? theme.colormap.tabhover : theme.colormap.headertext)
        anchors.left: parent.left
        anchors.verticalCenter: parent.verticalCenter
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onEntered: parent.isHovered = true
        onExited: parent.isHovered = false
        onClicked: parent.clicked()
    }
}