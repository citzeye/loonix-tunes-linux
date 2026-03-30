/* --- LOONIX-TUNES qml/ui/preferences/CollapsibleSection.qml --- */

import QtQuick
import QtQuick.Layouts

ColumnLayout {
    id: root
    property string title: "SECTION"
    property bool isOpen: false
    Layout.fillWidth: true
    spacing: 0

    // Clickable header
    Rectangle {
        Layout.fillWidth: true
        Layout.preferredHeight: 32
        color: headerMA.containsMouse ? theme.colormap["graysolid"] : "transparent"
        radius: 3

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 4
            anchors.rightMargin: 8
            spacing: 8

            Text {
                text: root.isOpen ? "▾" : "▸"
                color: theme.colormap["playeraccent"]
                font.family: kodeMono.name
                font.pixelSize: 16
                font.bold: true
            }

            Text {
                text: root.title
                color: theme.colormap["playeraccent"]
                font.family: kodeMono.name
                font.pixelSize: 14
                font.bold: true
                font.letterSpacing: 1.5
                Layout.fillWidth: true
            }

            Text {
                text: root.isOpen ? "hide" : "show"
                color: theme.colormap["playersubtext"]
                font.family: kodeMono.name
                font.pixelSize: 9
            }
        }

        MouseArea {
            id: headerMA
            anchors.fill: parent
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            onClicked: root.isOpen = !root.isOpen
        }
    }

    // Content container (children go here)
    default property alias content: contentColumn.data
    ColumnLayout {
        id: contentColumn
        Layout.fillWidth: true
        visible: root.isOpen
        spacing: 8
    }

    // Bottom separator
    Rectangle {
        Layout.fillWidth: true
        Layout.topMargin: 4
        height: 1
        color: theme.colormap["graysolid"]
        opacity: 0.2
    }
}
