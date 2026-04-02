import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Flickable {
    id: aboutFlick
    contentHeight: aboutColumn.height
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
        id: aboutColumn
        width: aboutFlick.width
        spacing: 12

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8

            ColumnLayout {
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignHCenter
                spacing: 12

                // Logo
                Rectangle {
                    Layout.alignment: Qt.AlignHCenter
                    Layout.preferredWidth: 100
                    Layout.preferredHeight: 100
                    color: "transparent"

                    Image {
                        anchors.centerIn: parent
                        source: "qrc:/assets/LoonixTunes.png"
                        width: 100
                        height: 100
                        fillMode: Image.PreserveAspectFit
                    }
                }

                Text {
                    text: "LOONIX TUNES"
                    Layout.alignment: Qt.AlignHCenter
                    color: theme.colormap["playertitle"]
                    font.family: kodeMono.name
                    font.pixelSize: 24
                    font.bold: true
                    font.letterSpacing: 2
                }

                Text {
                    text: "v1.0.0 | HBD release"
                    Layout.alignment: Qt.AlignHCenter
                    color: theme.colormap["playersubtext"]
                    font.family: kodeMono.name
                    font.pixelSize: 11
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.topMargin: 4
                    height: 1
                    color: theme.colormap["graysolid"]
                    opacity: 0.3
                }

                Text {
                    text: "A high-performance audio engine built with Rust and Qt Quick. Designed for audiophiles who value speed, modularity, and a compact desktop footprint."
                    Layout.fillWidth: true
                    color: theme.colormap["playlisttext"]
                    font.family: kodeMono.name
                    font.pixelSize: 13
                    wrapMode: Text.WordWrap
                    horizontalAlignment: Text.AlignHCenter
                    lineHeight: 1.2
                }

                Text {
                    text: "\u201CMusik bukan sekadar suara\u2014ia adalah ruang di mana waktu berhenti dan perasaan berbicara.\u201D"
                    Layout.fillWidth: true
                    color: theme.colormap["playersubtext"]
                    font.family: kodeMono.name
                    font.pixelSize: 12
                    font.italic: true
                    wrapMode: Text.WordWrap
                    horizontalAlignment: Text.AlignHCenter
                    lineHeight: 1.3
                    opacity: 0.9
                }

                Text {
                    text: "Developed by citzeye"
                    Layout.alignment: Qt.AlignHCenter
                    color: theme.colormap["playeraccent"]
                    font.family: kodeMono.name
                    font.pixelSize: 12
                    font.italic: true
                }
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8

            Text {
                text: musicModel.update_status
                Layout.alignment: Qt.AlignHCenter
                color: musicModel.update_available ? theme.colormap["playeraccent"] : theme.colormap["playersubtext"]
                font.family: kodeMono.name
                font.pixelSize: 11
            }

            RowLayout {
                Layout.alignment: Qt.AlignHCenter
                spacing: 12

                Item { Layout.fillWidth: true }

                Rectangle {
                    Layout.preferredWidth: updateBtnText.implicitWidth + 30
                    Layout.preferredHeight: 30
                    radius: 4
                    color: updateBtnArea.containsMouse ? Qt.lighter(theme.colormap["playeraccent"], 1.1) : theme.colormap["playeraccent"]
                    Behavior on color { ColorAnimation { duration: 150 } }

                    Text {
                        id: updateBtnText
                        anchors.centerIn: parent
                        text: "Check Update"
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        font.bold: true
                        color: theme.colormap["bgmain"]
                    }
                    MouseArea {
                        id: updateBtnArea
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        hoverEnabled: true
                        onClicked: musicModel.check_for_updates()
                    }
                }

                Rectangle {
                    visible: musicModel.update_available
                    Layout.preferredWidth: getBtnText.implicitWidth + 30
                    Layout.preferredHeight: 30
                    radius: 4
                    color: "transparent"
                    border.color: getBtnArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                    border.width: 1
                    Behavior on color { ColorAnimation { duration: 150 } }
                    Behavior on border.color { ColorAnimation { duration: 150 } }

                    Text {
                        id: getBtnText
                        anchors.centerIn: parent
                        text: "Get Update"
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        color: getBtnArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["playlisttext"]
                    }
                    MouseArea {
                        id: getBtnArea
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        hoverEnabled: true
                        onClicked: Qt.openUrlExternally("https://github.com/citz/loonix-tunes/releases")
                    }
                }
                Item { Layout.fillWidth: true }
            }
        }

        Item { Layout.fillHeight: true; Layout.minimumHeight: 20 }
    }
}
