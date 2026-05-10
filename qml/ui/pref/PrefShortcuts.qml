/* --- loonixtunesv2/qml/ui/pref/PrefShortcuts.qml | PrefShortcuts --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Flickable {
    id: shortcutsFlick
    contentHeight: shortcutsList.contentHeight
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    ScrollBar.vertical: ScrollBar {
        width: 6
        policy: ScrollBar.AsNeeded
        background: Rectangle { color: "transparent" }
        contentItem: Rectangle {
            implicitWidth: 6
            implicitHeight: Math.max(30, shortcutsFlick.height * shortcutsFlick.visibleArea.heightRatio)
            radius: 3
            color: theme.colormap.playeraccent
            opacity: active ? 1.0 : 0.5
        }
    }

    ListModel {
        id: groupedModel
        ListElement { isHeader: true; category: "Playback"; isLast: false }
        ListElement { isHeader: false; action: "Play/Pause"; key: "Space" }
        ListElement { isHeader: false; action: "Mute/Unmute"; key: "M" }
        ListElement { isHeader: false; action: "Stop"; key: "Ctrl+S" }
        ListElement { isHeader: false; action: "Next Track"; key: "Ctrl+Right" }
        ListElement { isHeader: false; action: "Previous Track"; key: "Ctrl+Left" }
        ListElement { isHeader: false; action: "Seek +5s"; key: "Shift+Right" }
        ListElement { isHeader: false; action: "Seek -5s"; key: "Shift+Left"; isLast: true }
        ListElement { isHeader: true; category: "Volume"; isLast: false }
        ListElement { isHeader: false; action: "Volume Up +5%"; key: "Ctrl+Up" }
        ListElement { isHeader: false; action: "Volume Down -5%"; key: "Ctrl+Down"; isLast: true }
        ListElement { isHeader: true; category: "Modes"; isLast: false }
        ListElement { isHeader: false; action: "Toggle Shuffle"; key: "Ctrl+H" }
        ListElement { isHeader: false; action: "Toggle Repeat"; key: "Ctrl+R" }
        ListElement { isHeader: false; action: "Toggle A-B Loop"; key: "Ctrl+L"; isLast: true }
        ListElement { isHeader: true; category: "Navigation"; isLast: false }
        ListElement { isHeader: false; action: "Music Tab"; key: "1" }
        ListElement { isHeader: false; action: "Favorites Tab"; key: "2" }
        ListElement { isHeader: false; action: "Queue Tab"; key: "Q" }
        ListElement { isHeader: false; action: "External Tab"; key: "3"; isLast: true }
        ListElement { isHeader: true; category: "Queue"; isLast: false }
        ListElement { isHeader: false; action: "Add to Queue"; key: "Ctrl+Shift+A" }
        ListElement { isHeader: false; action: "Remove from Queue"; key: "Delete" }
        ListElement { isHeader: false; action: "Clear Queue"; key: "Ctrl+Shift+C"; isLast: true }
        ListElement { isHeader: true; category: "Library"; isLast: false }
        ListElement { isHeader: false; action: "Focus Search"; key: "Ctrl+F" }
        ListElement { isHeader: false; action: "Scan Music"; key: "F5" }
        ListElement { isHeader: false; action: "Add Song"; key: "Ctrl+N" }
        ListElement { isHeader: false; action: "Toggle Favorite"; key: "Shift+F"; isLast: true }
        ListElement { isHeader: true; category: "DSP"; isLast: false }
        ListElement { isHeader: false; action: "Toggle DSP Master"; key: "D" }
        ListElement { isHeader: false; action: "Reset All DSP"; key: "Ctrl+0" }
        ListElement { isHeader: false; action: "Bass Booster"; key: "B" }
        ListElement { isHeader: false; action: "Reverb"; key: "R" }
        ListElement { isHeader: false; action: "Surround"; key: "S" }
        ListElement { isHeader: false; action: "Crystalizer"; key: "C" }
        ListElement { isHeader: false; action: "Compressor"; key: "X" }
        ListElement { isHeader: false; action: "Middle Clarity"; key: "L" }
        ListElement { isHeader: false; action: "Stereo Width"; key: "W" }
        ListElement { isHeader: false; action: "Normalizer"; key: "N"; isLast: true }
        ListElement { isHeader: true; category: "Presets"; isLast: false }
        ListElement { isHeader: false; action: "Load FX Preset"; key: "Ctrl+P" }
        ListElement { isHeader: false; action: "Load EQ Preset"; key: "Ctrl+E" }
        ListElement { isHeader: false; action: "Save User Preset"; key: "Ctrl+Shift+S"; isLast: true }
        ListElement { isHeader: true; category: "UI"; isLast: false }
        ListElement { isHeader: false; action: "Cycle Theme"; key: "T" }
        ListElement { isHeader: false; action: "Open Preferences"; key: "Ctrl+," }
        ListElement { isHeader: false; action: "Close Dialogs"; key: "Escape" }
        ListElement { isHeader: false; action: "Context Menu"; key: "Shift+F10"; isLast: true }
        ListElement { isHeader: true; category: "Window"; isLast: false }
        ListElement { isHeader: false; action: "Minimize"; key: "Ctrl+M" }
        ListElement { isHeader: false; action: "Quit"; key: "Ctrl+W" }
        ListElement { isHeader: false; action: "Toggle Fullscreen"; key: "F11"; isLast: true }
    }

    ListView {
        id: shortcutsList
        anchors.fill: parent
        anchors.leftMargin: 10
        anchors.rightMargin: 10
        anchors.topMargin: 10
        model: groupedModel

        delegate: Loader {
            id: loader
            width: shortcutsList.width - 20
            property bool isHeader: model.isHeader
            property bool isLast: model.isLast
            property string category: model.category
            property string action: model.action
            property string key: model.key
            sourceComponent: isHeader ? sectionComp : rowComp
        }

        Component {
            id: sectionComp
            ColumnLayout {
                spacing: 4
                Text {
                    text: category
                    font.family: kodeMono.name
                    font.pixelSize: 12
                    font.bold: true
                    color: theme.colormap.playeraccent
                    Layout.fillWidth: true
                }
                Rectangle {
                    width: shortcutsList.width - 20
                    height: 1
                    color: theme.colormap.graysolid
                }
            }
        }

        Component {
            id: rowComp
            ColumnLayout {
                spacing: 0
                RowLayout {
                    height: 20
                    spacing: 4
                    Text {
                        text: action
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        color: theme.colormap.playlisttext
                        Layout.fillWidth: true
                    }
                    Rectangle {
                        Layout.preferredWidth: 80
                        Layout.preferredHeight: 20
                        radius: 4
                        color: theme.colormap.bgoverlay
                        Text {
                            text: key
                            font.family: kodeMono.name
                            font.pixelSize: 10
                            color: theme.colormap.tabtext
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            anchors.centerIn: parent
                        }
                    }
                }
                Item { Layout.preferredHeight: isLast ? 16 : 0 }
            }
        }
    }
}