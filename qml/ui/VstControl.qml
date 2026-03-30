import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
import "vst"

Window {
    id: fxWindow
    title: "LOONIX-TUNES VST ENGINE"
    width: 1100
    height: 750
    visible: true
    color: theme.colormap["bgmain"]

    // State logic
    property bool isEditorOpen: false
    property string activePluginName: ""
    property string activePluginPath: ""

    // Fungsi buat buka VST (Logika alur user)
    function openPlugin(path, name) {
        activePluginPath = path
        activePluginName = name
        isEditorOpen = true
        musicModel.load_vst3_plugin(path)
        musicModel.open_vst_editor(fxWindow.winId) 
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 20
        spacing: 20
        
        VstHead {
            id: vstHeader
        }

        // Main Content Area
        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: "transparent"
            border.color: theme.colormap["graysolid"]
            border.width: 1
            radius: 10
            clip: true

            RowLayout {
                anchors.fill: parent
                spacing: 0

                VstLeft {
                    id: vstLeft
                }

                VstRight {
                    id: vstRight
                }
            }
        }

        VstFoot {
            id: vstFoot
        }
    }
}
