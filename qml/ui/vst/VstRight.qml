import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    Layout.fillWidth: true
    Layout.fillHeight: true
    color: theme.colormap["bgmain"]

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 15
        visible: !isEditorOpen

        Text {
            text: "󰓠"
            font.family: "Nerd Font"; font.pixelSize: 60
            color: theme.colormap["graysolid"]
            Layout.alignment: Qt.AlignHCenter
        }

        Text {
            text: "SELECT A PLUGIN FROM THE LIBRARY TO LOAD EDITOR"
            font.family: kodeMono.name; font.pixelSize: 12
            color: theme.colormap["graysolid"]
            Layout.alignment: Qt.AlignHCenter
        }
    }

    // Placeholder Area buat VST UI aslinya
    Item {
        anchors.fill: parent
        visible: isEditorOpen
        
        Text {
            anchors.top: parent.top
            anchors.horizontalCenter: parent.horizontalCenter
            anchors.topMargin: 20
            text: "NATIVE VST UI LOADED IN WINDOW ID: " + fxWindow.winId
            color: theme.colormap["playeraccent"]
            font.family: kodeMono.name; font.pixelSize: 10
        }
    }
}
