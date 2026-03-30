import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    Layout.fillWidth: true
    height: 80
    color: "transparent"

    RowLayout {
        anchors.fill: parent
        anchors.margins: 20
        spacing: 15

        // Tombol Back (Hanya muncul pas Editor VST terbuka)
        Rectangle {
            Layout.preferredWidth: 40; Layout.preferredHeight: 40; radius: 20
            color: isEditorOpen ? theme.colormap["playeraccent"] : "transparent"
            visible: isEditorOpen
            Text { anchors.centerIn: parent; text: "󰁍"; font.family: "Nerd Font"; font.pixelSize: 18; color: theme.colormap["bgmain"] }
            MouseArea { anchors.fill: parent; onClicked: isEditorOpen = false }
        }

        ColumnLayout {
            spacing: 2
            Text { 
                text: isEditorOpen ? activePluginName.toUpperCase() : "VST CONTROL"
                font.pixelSize: 18; font.bold: true; color: theme.colormap["playlisttext"] 
            }
        }

        Item { Layout.fillWidth: true } // Spacer

        // Kotak Search / Path Input (Manual)
        Rectangle {
            Layout.preferredWidth: 350; Layout.preferredHeight: 38; radius: 8
            color: theme.colormap["bgmain"]; border.color: theme.colormap["graysolid"]
            border.width: 1
            
            RowLayout {
                anchors.fill: parent; anchors.leftMargin: 12; anchors.rightMargin: 8
                TextInput {
                    id: pathInput; Layout.fillWidth: true; color: theme.colormap["playlisttext"]; font.pixelSize: 12
                    verticalAlignment: Text.AlignVCenter; clip: true
                    Text { text: "Search or paste VST3 path..."; color: theme.colormap["graysolid"]; visible: !pathInput.text }
                }
            }
        }

        // --- TOMBOL + PATH (Browse Folder) ---
        Rectangle {
            Layout.preferredWidth: 38; Layout.preferredHeight: 38; radius: 8
            color: theme.colormap["bgmain"]; opacity: addPathArea.containsMouse ? 0.3 : 0.15
            border.color: theme.colormap["graysolid"]; border.width: 1

            Text {
                anchors.centerIn: parent
                text: "" 
                font.family: "Nerd Font"; font.pixelSize: 18; color: theme.colormap["playlisttext"]
            }

            MouseArea {
                id: addPathArea
                anchors.fill: parent
                hoverEnabled: true
                onClicked: musicModel.add_vst3_path("") // Kirim kosong biar Rust buka File Picker
            }
        }

        // Tombol Refresh / Scan
        Button {
            text: "󰑐"; font.family: "Nerd Font"
            onClicked: musicModel.refresh_vst3_plugins()
            background: Rectangle { 
                implicitWidth: 38; implicitHeight: 38; radius: 8; 
                color: theme.colormap["bgmain"]; opacity: parent.hovered ? 0.3 : 0.15
                border.color: theme.colormap["graysolid"]; border.width: 1
            }
        }
    }
}
