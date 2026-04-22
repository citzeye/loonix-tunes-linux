/* --- loonixtunesv2/qml/ui/pref/PrefReportBug.qml | Report Bug --- */
import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15

Rectangle {
    id: root
    color: theme.colormap.bgmain
    radius: 6
    clip: true

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 15
        spacing: 12

        Text {
            text: "REPORT BUG / FEEDBACK"
            font.family: kodeMono.name
            font.pixelSize: 14
            font.bold: true
            color: theme.colormap.playeraccent
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 35
            color: theme.colormap.bgoverlay
            border.color: titleInput.activeFocus ? theme.colormap.playeraccent : theme.colormap.graysolid
            
            TextInput {
                id: titleInput
                anchors.fill: parent
                anchors.margins: 8
                color: theme.colormap.tabtext
                font.pixelSize: 12
                verticalAlignment: Text.AlignVCenter
                clip: true
                Text {
                    text: "Bug title..."
                    color: theme.colormap.playersubtext
                    visible: !parent.text && !parent.activeFocus
                    anchors.fill: parent
                    verticalAlignment: Text.AlignVCenter
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: theme.colormap.bgoverlay
            border.color: descInput.activeFocus ? theme.colormap.playeraccent : theme.colormap.graysolid

            Flickable {
                anchors.fill: parent
                contentWidth: width
                contentHeight: descInput.implicitHeight
                clip: true

                TextEdit {
                    id: descInput
                    width: parent.width
                    padding: 8
                    color: theme.colormap.tabtext
                    font.pixelSize: 12
                    wrapMode: TextEdit.Wrap
                    selectByMouse: true
                    
                    Text {
                        text: "Describe the bug..."
                        color: theme.colormap.playersubtext
                        visible: !parent.text && !parent.activeFocus
                        x: 8; y: 8
                    }
                }
            }
        }

        Rectangle {
            Layout.alignment: Qt.AlignRight
            width: 120
            height: 35
            radius: 4
            color: (titleInput.text && descInput.text) ? theme.colormap.playeraccent : theme.colormap.graysolid
            
            Text {
                anchors.centerIn: parent
                text: "OPEN GITHUB"
                font.bold: true
                font.pixelSize: 11
                color: theme.colormap.bgmain
            }

            MouseArea {
                anchors.fill: parent
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    if (titleInput.text && descInput.text) {
                        theme.report_bug_on_github(titleInput.text, descInput.text)
                        titleInput.text = ""
                        descInput.text = ""
                    }
                }
            }
        }
    }
}
