import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    Layout.preferredWidth: parent.width * 0.10
    Layout.fillHeight: true
    color: theme.colormap["bgoverlay"]
    
    // Garis Pemisah Vertikal
    Rectangle {
        anchors.right: parent.right
        width: 1; height: parent.height
        color: theme.colormap["graysolid"]
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10
        
        Text {
            text: "LIBRARY"
            font.bold: true; font.pixelSize: 10
            color: theme.colormap["playeraccent"]
            Layout.bottomMargin: 10
        }

        ListView {
            id: treeView
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            model: [
                { "name": "~/.vst3", "isPath": true },
                { "name": "GuitarRig.vst3", "isPath": false },
                { "name": "Serum.so", "isPath": false },
                { "name": "/usr/lib/vst3", "isPath": true },
                { "name": "FabFilter.vst3", "isPath": false }
            ]

            delegate: ItemDelegate {
                width: parent.width
                height: 28
                padding: 0

                contentItem: RowLayout {
                    spacing: 8
                    Item { Layout.preferredWidth: modelData.isPath ? 0 : 15 }
                    
                    Text {
                        text: modelData.isPath ? "" : "󰈔"
                        font.family: "Nerd Font"
                        font.pixelSize: 12
                        color: modelData.isPath ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                    }
                    
                    Text {
                        text: modelData.name
                        font.family: kodeMono.name
                        font.pixelSize: 10
                        color: modelData.isPath ? theme.colormap["playlisttext"] : theme.colormap["playlisttext"]
                        elide: Text.ElideRight
                        Layout.fillWidth: true
                    }
                }
                
                background: Rectangle {
                    color: highlighted ? theme.colormap["playeraccent"] : "transparent"
                    opacity: highlighted ? 0.3 : 0
                }

                onClicked: {
                    if (!modelData.isPath) {
                        openPlugin(modelData.name, modelData.name)
                    }
                }
            }
        }
    }
}
