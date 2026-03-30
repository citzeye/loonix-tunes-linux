/* --- LOONIX-TUNES qml/ui/TrackInfo.qml --- */
import QtQuick
import QtQuick.Layouts

Item {
    id: trackInfoRoot
    visible: musicModel.track_info_visible
    anchors.fill: parent
    z: 9000

    Rectangle {
        anchors.fill: parent
        color: "#80000000"
        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.LeftButton | Qt.RightButton
            onClicked: musicModel.close_track_info()
        }
    }

    Rectangle {
        anchors.centerIn: parent
        anchors.leftMargin: 16
        anchors.rightMargin: 16
        anchors.topMargin: 40
        anchors.bottomMargin: 40
        width: Math.min(parent.width - 32, 380)
        height: Math.min(parent.height - 80, 460)
        radius: 6
        color: theme.colormap.bgoverlay
        border.color: theme.colormap.tabborder
        antialiasing: false

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 0
            spacing: 0

            // HEADER
            RowLayout {
                Layout.fillWidth: true
                Layout.preferredHeight: 42
                Layout.leftMargin: 16
                Layout.rightMargin: 8

                Text {
                    text: '󰋼'
                    font.family: symbols.name
                    font.pixelSize: 18
                    color: theme.colormap.playeraccent
                }

                Text {
                    text: "TRACK INFO"
                    font.family: kodeMono.name
                    font.pixelSize: 13
                    font.bold: true
                    color: theme.colormap.playeraccent
                    Layout.leftMargin: 6
                }

                Item { Layout.fillWidth: true }

                Text {
                    text: '    '
                    font.family: symbols.name
                    font.pixelSize: 18
                    color: theme.colormap.playersubtext
                    MouseArea {
                        anchors.fill: parent
                        hoverEnabled: true
                        onClicked: musicModel.close_track_info()
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 1
                Layout.leftMargin: 16
                Layout.rightMargin: 16
                color: theme.colormap.tabborder
            }

            // CONTENT
            Flickable {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.leftMargin: 16
                Layout.rightMargin: 16
                Layout.topMargin: 12
                Layout.bottomMargin: 12
                contentHeight: contentCol.height
                clip: true
                interactive: true

                ColumnLayout {
                    id: contentCol
                    anchors.left: parent.left
                    anchors.right: parent.right
                    spacing: 4

                    // TITLE
                    InfoRow {
                        label: "TITLE"
                        value: musicModel.track_info_title
                    }

                    // ARTIST
                    InfoRow {
                        label: "ARTIST"
                        value: musicModel.track_info_artist
                    }

                    // ALBUM
                    InfoRow {
                        label: "ALBUM"
                        value: musicModel.track_info_album
                    }

                    // YEAR
                    InfoRow {
                        label: "YEAR"
                        value: musicModel.track_info_year
                    }

                    // GENRE
                    InfoRow {
                        label: "GENRE"
                        value: musicModel.track_info_genre
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 1
                        Layout.topMargin: 6
                        Layout.bottomMargin: 6
                        color: theme.colormap.tabborder
                        opacity: 0.5
                    }

                    // DURATION
                    InfoRow {
                        label: "DURATION"
                        value: musicModel.track_info_duration
                    }

                    // BITRATE
                    InfoRow {
                        label: "BITRATE"
                        value: musicModel.track_info_bitrate
                    }

                    // SAMPLE RATE
                    InfoRow {
                        label: "SAMPLE RATE"
                        value: musicModel.track_info_sample_rate
                    }

                    // CHANNELS
                    InfoRow {
                        label: "CHANNELS"
                        value: musicModel.track_info_channels
                    }

                    // CODEC
                    InfoRow {
                        label: "CODEC"
                        value: musicModel.track_info_codec
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 1
                        Layout.topMargin: 6
                        Layout.bottomMargin: 6
                        color: theme.colormap.tabborder
                        opacity: 0.5
                    }

                    // FILE SIZE
                    InfoRow {
                        label: "FILE SIZE"
                        value: musicModel.track_info_file_size
                    }

                    // FILE PATH
                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 2

                        Text {
                            text: "PATH"
                            font.family: kodeMono.name
                            font.pixelSize: 11
                            color: theme.colormap.graysolid
                        }

                        Text {
                            text: musicModel.track_info_file_path
                            font.family: kodeMono.name
                            font.pixelSize: 11
                            color: theme.colormap.playersubtext
                            wrapMode: Text.NoWrap
                            elide: Text.ElideMiddle
                            Layout.fillWidth: true
                        }
                    }
                }
            }
        }
    }

    // REUSABLE ROW COMPONENT
    component InfoRow: RowLayout {
        property string label
        property string value
        readonly property bool isEmpty: value === "" || value.toLowerCase() === "unknown"
        Layout.fillWidth: true
        spacing: 8

        Text {
            text: label
            font.family: kodeMono.name
            font.pixelSize: 11
            color: theme.colormap.graysolid
            Layout.preferredWidth: 110
        }

        Text {
            text: isEmpty ? "-" : value
            font.family: kodeMono.name
            font.pixelSize: 12
            color: isEmpty ? theme.colormap.graysolid : theme.colormap.tabtext
            Layout.fillWidth: true
            elide: Text.ElideRight
        }
    }
}
