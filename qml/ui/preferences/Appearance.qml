import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Flickable {
    id: appFlick
    contentWidth: width
    // PENTING: Wajib pakai implicitHeight biar Scroll jalan dan layout gak rusak
    contentHeight: appColumn.implicitHeight 
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds

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