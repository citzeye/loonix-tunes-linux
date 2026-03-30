import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Flickable {
    id: donateFlick
    contentWidth: width
    contentHeight: donateColumn.height
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds

    ColumnLayout {
        id: donateColumn
        width: donateFlick.width
        spacing: 12

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8

            Text {
                text: "Keep the Engine Running"
                Layout.alignment: Qt.AlignHCenter
                color: theme.colormap["playertitle"]
                font.family: kodeMono.name
                font.pixelSize: 20
                font.bold: true
            }

            Text {
                text: "Developing a low-latency audio engine in Rust takes a lot of time and even more coffee. Support the project to keep it ad-free and open-source."
                Layout.fillWidth: true
                color: theme.colormap["playlisttext"]
                font.family: kodeMono.name
                font.pixelSize: 13
                wrapMode: Text.WordWrap
                horizontalAlignment: Text.AlignHCenter
            }

            RowLayout {
                Layout.alignment: Qt.AlignHCenter
                Layout.topMargin: 10
                spacing: 15

                Rectangle {
                    Layout.preferredWidth: saweriaText.implicitWidth + 40
                    Layout.preferredHeight: 40
                    radius: 6
                    color: "transparent"
                    border.color: saweriaArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                    border.width: 1
                    Behavior on border.color { ColorAnimation { duration: 150}}

                    Text {
                        id: saweriaText
                        anchors.centerIn: parent
                        text: "Saweria"
                        color: "white"
                        font.family: kodeMono.name
                        font.pixelSize: 13
                        font.bold: true
                    }
                    MouseArea {
                        id: saweriaArea
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        hoverEnabled: true
                        onClicked: Qt.openUrlExternally("https://saweria.co/citzeye")
                    }
                }

                Rectangle {
                    Layout.preferredWidth: kofiText.implicitWidth + 40
                    Layout.preferredHeight: 40
                    radius: 6
                    color: "transparent"
                    border.color: kofiArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                    border.width: 1
                    Behavior on border.color { ColorAnimation { duration: 150 } }

                    Text {
                        id: kofiText
                        anchors.centerIn: parent
                        text: "Ko-fi"
                        color: kofiArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["playlisttext"]
                        font.family: kodeMono.name
                        font.pixelSize: 13
                        font.bold: true
                        Behavior on color { ColorAnimation { duration: 150 } }
                    }
                    MouseArea {
                        id: kofiArea
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        hoverEnabled: true
                        onClicked: Qt.openUrlExternally("https://ko-fi.com/citzeye")
                    }
                }
            }
        }

        Item { Layout.fillHeight: true; Layout.minimumHeight: 20 }
    }
}
