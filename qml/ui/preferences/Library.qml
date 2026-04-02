import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform

Flickable {
    id: libFlick
    contentHeight: libColumn.height
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

    FolderDialog {
        id: manualScanFolderPicker
        title: "Select Folder to Scan"
        onAccepted: {
            var path = manualScanFolderPicker.folder.toString()
            if (path.startsWith('file://')) {
                path = path.substring(7)
            }
            if (path.endsWith('/')) {
                path = path.substring(0, path.length - 1)
            }
            musicModel.scan_folder(path)
        }
    }

    ColumnLayout {
        id: libColumn
        width: libFlick.width - 15
        spacing: 24

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8
            SettingHeader { title: "FILE EXPLORER" }

            SettingSwitch {
                label: "Prioritize Folders"
                description: "Always show folders at the top of the list, before loose files."
                checked: true
            }
            SettingSwitch {
                label: "Auto-Scan ~/Music"
                description: "Watch for new files added to your default music directory."
                checked: true
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8
            SettingHeader { title: "MANUAL SCAN" }

            Text {
                Layout.fillWidth: true
                text: "Scan a specific folder to load its music without adding it as a permanent tab."
                font.family: kodeMono.name
                font.pixelSize: 11
                color: theme.colormap["playersubtext"]
                wrapMode: Text.WordWrap
            }

            RowLayout {
                Layout.fillWidth: true
                Layout.topMargin: 5
                spacing: 12

                Rectangle {
                    Layout.preferredWidth: scanFolderText.implicitWidth + 24
                    Layout.preferredHeight: 30
                    radius: 4
                    color: theme.colormap["bgoverlay"]
                    border.color: scanFolderArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                    border.width: 1

                    Text {
                        id: scanFolderText
                        anchors.centerIn: parent
                        text: "Scan Folder..."
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        color: scanFolderArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["playlisttext"]
                    }
                    MouseArea {
                        id: scanFolderArea
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        hoverEnabled: true
                        onClicked: manualScanFolderPicker.open()
                    }
                }

                Rectangle {
                    Layout.preferredWidth: rescanText.implicitWidth + 24
                    Layout.preferredHeight: 30
                    radius: 4
                    color: theme.colormap["bgoverlay"]
                    border.color: rescanArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                    border.width: 1

                    Text {
                        id: rescanText
                        anchors.centerIn: parent
                        text: "Rescan ~/Music"
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        color: rescanArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["playlisttext"]
                    }
                    MouseArea {
                        id: rescanArea
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        hoverEnabled: true
                        onClicked: musicModel.scan_music()
                    }
                }
            }
        }
    }
}
