/* --- LOONIX-TUNES qml/ui/Tab.qml --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Qt.labs.platform

// ==========================================
// SECTION: TAB BAR (UNIFIED DECORATION)
// ==========================================
Rectangle {
  id: tabBarSection
  Layout.fillWidth: true
  implicitHeight: leftGroup.height + 10
  color: theme.colormap.bgoverlay

  // Folder Dialog (for adding new folders)
  FolderDialog {
    id: folderPicker
    title: 'Select Music Folder'
    onAccepted: {
      var path = folderPicker.folder.toString()
      if (path.startsWith('file://')) {
        path = path.substring(7)
      }
      if (path.endsWith('/')) {
        path = path.substring(0, path.length - 1)
      }
      musicModel.add_folder_tab(path)
    }
  }

  // --- CONTAINER TABS (Flow wraps automatically) ---
  Flow {
    id: leftGroup
    anchors.left: parent.left
    anchors.right: addTabBtnRight.left
    anchors.top: parent.top
    anchors.leftMargin: 16
    anchors.rightMargin: 10
    anchors.topMargin: 5
    spacing: 5

    // --- 0. TAB DEFAULT (EXTERNAL FILES) ---
    Rectangle {
      id: tabdefault
      visible: musicModel.external_files_count > 0
      width: 30
      height: 20
      property bool isActive: musicModel.current_folder_qml === 'EXTERNAL_FILES'
      color: isActive || tabdefaultMA.containsMouse ? theme.colormap.bgoverlay : 'transparent'
      radius: 4
      border.width: 0.5
      border.color: isActive || tabdefaultMA.containsMouse
        ? theme.colormap.tabhover
        : theme.colormap.tabborder
      antialiasing: false

      Text {
        anchors.centerIn: parent
        text: '\udb81\uddd8'
        font.family: symbols.name
        font.pixelSize: 12
        font.bold: parent.isActive
        color: theme.colormap.tabtext
      }

      MouseArea {
        id: tabdefaultMA
        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        onClicked: function(mouse) {
          if (mouse.button === Qt.LeftButton) {
            musicModel.switch_to_external_files()
          } else if (mouse.button === Qt.RightButton) {
            root.popupMenuVisible = false
            root.tabContextMenuVisible = false
            root.playlistContextMenuVisible = false
            root.popupX = mouse.x + tabdefault.x + tabBarSection.x
            root.popupY = mouse.y + tabdefault.y + tabBarSection.y + tabdefault.height
            root.externalFilesContextMenuVisible = true
          }
        }
      }
    }

    // --- 0.5. TAB QUEUE ---
    TabQueue {
      id: queueTab
    }

    // --- 1. TAB FAVORITES ---
    TabFavorites {
      id: favoritesTab
    }

    // --- 2. TAB STATIS (MUSIC) ---
    TabMusic {
      id: staticTab
    }

    // --- 3. TAB DINAMIS (CUSTOM FOLDERS) ---
    Repeater {
      model: musicModel.custom_folder_count
      delegate: TabCustom {}
    }
  }

  // --- 4. GRUP KANAN (ADD BUTTON) ---
  Rectangle {
    id: addTabBtnRight
    width: 24
    height: 20
    anchors.right: parent.right
    anchors.rightMargin: 16
    anchors.top: parent.top
    anchors.topMargin: 5
    color: addMARight.containsMouse ? theme.colormap.bgoverlay : 'transparent'
    radius: 4
    border.width: 0.5
    border.color: addMARight.containsMouse ? theme.colormap.tabhover : theme.colormap.tabborder
    antialiasing: false

    Text {
      anchors.centerIn: parent
      text: '+'
      font.family: kodeMono.name
      font.pixelSize: 14
      font.bold: true
      color: theme.colormap.tabtext
    }

    MouseArea {
      id: addMARight
      anchors.fill: parent
      hoverEnabled: true
      acceptedButtons: Qt.LeftButton | Qt.RightButton
      onClicked: function(mouse) {
        root.tabContextMenuIndex = -1
        root.popupX = mouse.x + addTabBtnRight.x + tabBarSection.x
        root.popupY = mouse.y + addTabBtnRight.y + tabBarSection.y + addTabBtnRight.height
        folderPicker.open()
      }
    }
  }
}
