/* --- loonixtunesv2/qml/ui/RenameDialog.qml | Rename Dialog --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Loader {
    id: renameDialogContainer
    active: root.renameDialogVisible
    anchors.fill: parent
    z: 9999

    sourceComponent: Component {
        Item {
            anchors.fill: parent

            Rectangle {
                anchors.fill: parent
                color: theme.colormap["overlay"]
                MouseArea {
                    anchors.fill: parent
                    onClicked: root.renameDialogVisible = false
                }
            }

            Rectangle {
                id: renameDialogBox
                anchors.centerIn: parent
                width: 240
                height: 80
                color: theme.colormap.bgmain
                border.color: theme.colormap.playeraccent
                radius: 4

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 8

                    TextInput {
                        id: renameInput
                        Layout.fillWidth: true
                        Layout.preferredHeight: 28
                        text: musicModel.get_current_rename_name(root.renameDialogIndex)
                        font.family: kodeMono.name
                        font.pixelSize: 12
                        color: theme.colormap.playeraccent
                        verticalAlignment: Text.AlignVCenter
                        maximumLength: 10
                        activeFocusOnPress: true
                        selectByMouse: true

                        onAccepted: {
                            if (text.trim().length > 0) {
                                musicModel.rename_folder(root.renameDialogIndex, text.trim())
                            }
                            root.renameDialogVisible = false
                        }
                        Component.onCompleted: {
                            forceActiveFocus()
                            selectAll()
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 16

                        Text {
                            text: 'CANCEL'
                            font.family: kodeMono.name
                            font.pixelSize: 10
                            color: renameCancelMA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
                            MouseArea {
                                id: renameCancelMA
                                anchors.fill: parent
                                hoverEnabled: true
                                onClicked: root.renameDialogVisible = false
                            }
                        }

                        Item { Layout.fillWidth: true }

                        Text {
                            text: 'SAVE'
                            font.family: kodeMono.name
                            font.pixelSize: 10
                            color: renameSaveMA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
                            MouseArea {
                                id: renameSaveMA
                                anchors.fill: parent
                                hoverEnabled: true
                                onClicked: {
                                    if (renameInput.text.trim().length > 0) {
                                        musicModel.rename_folder(root.renameDialogIndex, renameInput.text.trim())
                                    }
                                    root.renameDialogVisible = false
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}