/* --- LOONIX-TUNES qml/ui/TabContextMenu.qml --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Qt.labs.platform

Item {
  id: tabContextMenuRoot
  z: 1001
  visible: root.tabContextMenuVisible
  x: root.popupX
  y: root.popupY
  width: menuGrid.width + 16
  height: menuGrid.height + 16

  Rectangle {
    id: borderRect
    anchors.fill: parent
    anchors.margins: 7
    color: 'transparent'
    radius: 4
    border.color: theme.colormap.tabborder
    border.width: 1
    antialiasing: false
  }

  Rectangle {
    anchors.fill: parent
    anchors.margins: 8
    color: theme.colormap.bgmain
    radius: 4
  }

  MouseArea {
    anchors.fill: parent
    anchors.margins: -10000
    hoverEnabled: true
    onClicked: {
      root.tabContextMenuVisible = false
      root.playlistContextMenuVisible = false
      root.popupMenuVisible = false
      root.externalFilesContextMenuVisible = false
    }
  }

  GridLayout {
    id: menuGrid
    anchors.centerIn: parent
    columns: 3
    rowSpacing: 2
    columnSpacing: 2

    // TILE 1 - Add Files
    Rectangle {
      Layout.preferredWidth: 50
      Layout.preferredHeight: 50
      radius: 4
      color: theme.colormap.bgmain
      Column {
        anchors.centerIn: parent
        spacing: 4
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: '󰷞'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile1MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'File'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile1MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile1MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          musicFilePicker.open()
        }
      }
    }

    // TILE 2 - Change Folder
    Rectangle {
      Layout.preferredWidth: 50
      Layout.preferredHeight: 50
      radius: 4
      color: theme.colormap.bgmain
      Column {
        anchors.centerIn: parent
        spacing: 4
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: ''
          font.family: symbols.name
          font.pixelSize: 18
          color: tile2MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Change'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile2MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile2MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          folderPicker.open()
        }
      }
    }

    // TILE 3 - Rename
    Rectangle {
      Layout.preferredWidth: 50
      Layout.preferredHeight: 50
      radius: 4
      color: theme.colormap.bgmain
      Column {
        anchors.centerIn: parent
        spacing: 4
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: '󰑕'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile3MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Rename'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile3MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile3MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          root.renameDialogIndex = root.tabContextMenuIndex
          root.renameDialogVisible = true
          root.tabContextMenuVisible = false
        }
      }
    }

    // TILE 4 - Lock/Unlock
    Rectangle {
      Layout.preferredWidth: 50
      Layout.preferredHeight: 50
      radius: 4
      color: theme.colormap.bgmain
      Column {
        anchors.centerIn: parent
        spacing: 4
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: {
            musicModel.folder_lock_version
            var idx = root.tabContextMenuIndex
            var locked = idx >= 0 ? musicModel.is_folder_locked(idx) : false
            return locked ? '󰌾' : '󰌿'
          }
          font.family: symbols.name
          font.pixelSize: 18
          color: {
            musicModel.folder_lock_version
            var idx = root.tabContextMenuIndex
            var locked = idx >= 0 ? musicModel.is_folder_locked(idx) : false
            if (locked) return theme.colormap.footerwarning
            return tile4MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
          }
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: {
            musicModel.folder_lock_version
            var idx = root.tabContextMenuIndex
            var locked = idx >= 0 ? musicModel.is_folder_locked(idx) : false
            if (idx >= 0) {
              return locked ? 'Unlock' : 'Lock'
            }
            return 'Lock'
          }
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile4MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile4MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          musicModel.toggle_folder_lock(root.tabContextMenuIndex)
        }
      }
    }

    // TILE 5 - Remove Tab
    Rectangle {
      Layout.preferredWidth: 50
      Layout.preferredHeight: 50
      radius: 4
      color: theme.colormap.bgmain
      Column {
        anchors.centerIn: parent
        spacing: 4
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: ''
          font.family: symbols.name
          font.pixelSize: 18
          color: tile5MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Remove'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile5MA.containsMouse ? theme.colormap.playlisticon
            : (root.tabContextMenuIndex >= 0 &&
              !musicModel.is_folder_locked(root.tabContextMenuIndex))
              ? theme.colormap.tabtext
              : theme.colormap.footerwarning
        }
      }
      MouseArea {
        id: tile5MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          if (root.tabContextMenuIndex >= 0 &&
            !musicModel.is_folder_locked(root.tabContextMenuIndex)
          ) {
            musicModel.remove_custom_folder(root.tabContextMenuIndex)
          }
          root.tabContextMenuVisible = false
        }
      }
    }

    // TILE 6 - Close
    Rectangle {
      Layout.preferredWidth: 50
      Layout.preferredHeight: 50
      radius: 4
      color: theme.colormap.bgmain
      Column {
        anchors.centerIn: parent
        spacing: 4
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: ''
          font.family: symbols.name
          font.pixelSize: 18
          color: theme.colormap.footeralert
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Close'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: theme.colormap.footeralert
        }
      }
      MouseArea {
        id: tile6MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          root.tabContextMenuVisible = false
          root.playlistContextMenuVisible = false
          root.popupMenuVisible = false
          root.externalFilesContextMenuVisible = false
        }
      }
    }
  }

  FolderDialog {
    id: folderPicker
    title: "Select Music Folder"
    onAccepted: {
      var path = folderPicker.folder.toString()
      if (path.startsWith("file://")) {
        path = path.substring(7)
      }
      if (path.endsWith("/")) {
        path = path.substring(0, path.length - 1)
      }
      musicModel.change_folder(root.tabContextMenuIndex, path)
      root.tabContextMenuVisible = false
    }
  }

  FileDialog {
    id: musicFilePicker
    title: "Select Music File"
    fileMode: FileDialog.OpenFiles
    nameFilters: ["Audio files (*.mp3 *.wav *.flac *.ogg *.m4a *.aac)"]
    onAccepted: {
      var files = musicFilePicker.files
      for (var i = 0; i < files.length; i++) {
        var filePath = files[i].toString()
        if (filePath.startsWith("file://")) {
          filePath = filePath.substring(7)
        }
        musicModel.add_song(filePath)
      }
      root.tabContextMenuVisible = false
    }
  }

  Connections {
    target: musicModel
    function onFolder_lock_changed() {
      // Force refresh
    }
  }
}
