/* --- LOONIX-TUNES qml/ui/PlaylistContextMenu.qml --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Qt.labs.platform

Item {
  id: playlistCtxRoot
  z: 1001
  visible: root.playlistContextMenuVisible
  x: root.playlistContextMenuX
  y: root.playlistContextMenuY
  width: menuGrid.width + 16
  height: menuGrid.height + 16

  // Background overlay - closes menu when clicking outside
  Rectangle {
    anchors.fill: parent
    color: 'transparent'
    z: -1
    
    MouseArea {
      anchors.fill: parent
      onClicked: root.playlistContextMenuVisible = false
    }
  }

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
    z: -2
    anchors.fill: parent
    anchors.margins: -10000
    hoverEnabled: true
    acceptedButtons: Qt.LeftButton | Qt.RightButton
    onClicked: {
      root.playlistContextMenuVisible = false
    }
  }

  GridLayout {
    id: menuGrid
    anchors.centerIn: parent
    columns: 3
    rowSpacing: 2
    columnSpacing: 2

    // TILE 1
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
          root.playlistContextMenuVisible = false
          musicFilePicker.open()
        }
      }
    }

    // TILE 2
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
          text: '󰉗'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile2MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Folder'
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
          root.playlistContextMenuVisible = false
          musicFolderPicker.open()
        }
      }
    }

    // TILE 3
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
          color: tile3MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Remove'
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
          if (root.playlistContextItemIndex >= 0) {
            musicModel.remove_song(root.playlistContextItemIndex)
          }
          root.playlistContextMenuVisible = false
        }
      }
    }

    // TILE 4
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
          text: '󰗨'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile4MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Del'
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
          if (root.playlistContextItemIndex >= 0) {
            deleteConfirmDialog.open()
          } else {
            root.playlistContextMenuVisible = false
          }
        }
      }
    }

    // TILE 5
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
          text: '󱕱'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile5MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Queue'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile5MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile5MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          if (root.playlistContextItemIndex >= 0) {
            var name = String(root.playlistContextItemName || "")
            var path = String(root.playlistContextItemPath || "")
            musicModel.add_to_queue(path, name)
          }
          root.playlistContextMenuVisible = false
        }
      }
    }

    // TILE 6
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
          text: '󰓎'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile6MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Fav'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile6MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile6MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          if (root.playlistContextItemIndex >= 0) {
            var name = String(root.playlistContextItemName || "")
            var path = String(root.playlistContextItemPath || "")
            musicModel.toggle_favorite(path, name)
          }
          root.playlistContextMenuVisible = false
        }
      }
    }

    // TILE 7
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
          text: '󰋼'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile7MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Info'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile7MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile7MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          if (root.playlistContextItemIndex >= 0) {
            musicModel.load_track_info(root.playlistContextItemPath)
          }
          root.playlistContextMenuVisible = false
        }
      }
    }

    // TILE 8
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
          text: '󰒓'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile8MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Setting'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: tile8MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
      }
      MouseArea {
        id: tile8MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          root.playlistContextMenuVisible = false
        }
      }
    }

    // TILE 9
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
        id: tile9MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: {
          root.playlistContextMenuVisible = false
        }
      }
    }
  }

  // Dialogs
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
    }
  }

  FolderDialog {
    id: musicFolderPicker
    title: "Select Music Folder"
    onAccepted: {
      var path = musicFolderPicker.folder.toString()
      if (path.startsWith("file://")) {
        path = path.substring(7)
      }
      if (path.endsWith("/")) {
        path = path.substring(0, path.length - 1)
      }
      musicModel.add_temporary_folder(path)
    }
  }

  MessageDialog {
    id: deleteConfirmDialog
    title: "Delete Permanent"
    text: "This will permanently delete the selected item from your system. Are you sure?"
    buttons: MessageDialog.Yes | MessageDialog.No
    onAccepted: {
      if (root.playlistContextItemIndex >= 0) {
        var path = String(root.playlistContextItemPath || "")
        var isFolder = root.playlistContextIsFolder
        musicModel.delete_item(path, isFolder)
      }
      root.playlistContextMenuVisible = false
    }
    onRejected: {
      root.playlistContextMenuVisible = false
    }
  }
}
