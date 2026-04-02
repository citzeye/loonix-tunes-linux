import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Flickable {
    id: appFlick
    contentHeight: appColumn.implicitHeight 
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    ScrollBar.vertical: ScrollBar {
        policy: ScrollBar.AsNeeded
        width: 6
        z: 1
        background: Rectangle { implicitWidth: 6; color: theme.colormap.bgmain; opacity: 0.0 }
        contentItem: Rectangle {
            implicitWidth: 6
            radius: 3
            color: parent.pressed ? theme.colormap.playeraccent : (parent.hovered ? theme.colormap.playerhover : theme.colormap.graysolid)
            Behavior on color { ColorAnimation { duration: 200 } }
        }
    }

    ColumnLayout {
        id: appColumn
        width: appFlick.width
        spacing: 12

        ColumnLayout {
            // KUNCI: Tarik seluruh grup (pembungkus) kotak ini ke tengah layar
            Layout.alignment: Qt.AlignHCenter 
            spacing: 4

            Repeater {
                model: ["Blue", "Green", "Monochrome", "Orange", "Pink", "Red", "Yellow", "Default"]

                delegate: Rectangle {
                    Layout.preferredWidth: 200
                    Layout.preferredHeight: 32
                    // Pastikan item di dalamnya juga ikut aturan rata tengah
                    Layout.alignment: Qt.AlignHCenter 
                    radius: 4
                    color: modelData === theme.current_theme ? theme.colormap["playeraccent"] : theme.colormap["bgoverlay"]
                    border.color: themeItemArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                    border.width: 1

                    Behavior on color { ColorAnimation { duration: 150 } }
                    Behavior on border.color { ColorAnimation { duration: 150 } }

                    Text {
                        anchors.centerIn: parent
                        text: modelData
                        font.family: kodeMono.name
                        font.pixelSize: 12
                        color: modelData === theme.current_theme ? theme.colormap["bgmain"] : theme.colormap["playlisttext"]
                        font.bold: modelData === theme.current_theme
                    }

                    MouseArea {
                        id: themeItemArea
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        hoverEnabled: true
                        onClicked: theme.set_theme(modelData)
                    }
                }
            }
        }
    }
}