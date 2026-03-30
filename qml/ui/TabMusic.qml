/* --- LOONIX-TUNES qml/ui/TabMusic.qml --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Qt.labs.platform

Rectangle {
  id: staticTab
  width: 60
  height: 20
  property bool isActive: musicModel.current_folder_qml === '' ||
    musicModel.current_folder_qml.toUpperCase() === 'MUSIC'
  color: isActive ? theme.colormap.bgoverlay : 'transparent'
  radius: 4
  border.width: isActive ? 0.5 : 0.5
  border.color: isActive ? theme.colormap.tabhover : theme.colormap.tabborder

  // Folder Dialog for adding folders
  FolderDialog {
    id: musicFolderPicker
    title: 'Select Music Folder'
    onAccepted: {
      var path = musicFolderPicker.folder.toString()
      if (path.startsWith('file://')) {
        path = path.substring(7)
      }
      if (path.endsWith('/')) {
        path = path.substring(0, path.length - 1)
      }
      musicModel.add_folder_tab(path)
    }
  }

  // File Dialog for adding individual songs
  FileDialog {
    id: musicFilePicker
    title: 'Select Music File'
    fileMode: FileDialog.OpenFiles
    nameFilters: ['Audio files (*.mp3 *.wav *.flac *.ogg *.m4a *.aac)']
    onAccepted: {
      var files = musicFilePicker.files
      for (var i = 0; i < files.length; i++) {
        var filePath = files[i].toString()
        if (filePath.startsWith('file://')) {
          filePath = filePath.substring(7)
        }
        // Add individual song to playlist
        musicModel.add_song(filePath)
      }
    }
  }

  // Menu for choosing between folder or file
  Menu {
    id: addMenu
    title: 'Add Music'
    MenuItem {
      text: 'Add Folder'
      onTriggered: musicFolderPicker.open()
    }
    MenuItem {
      text: 'Add Song'
      onTriggered: musicFilePicker.open()
    }
  }

  Text {
    anchors.centerIn: parent
    text: 'MUSIC'
    font.family: kodeMono.name
    font.pixelSize: 10
    font.bold: parent.isActive
    color: theme.colormap.tabtext
    anchors.leftMargin: 0
    anchors.rightMargin: 0
  }

  MouseArea {
    id: tabMA_static
    anchors.fill: parent
    hoverEnabled: true
    acceptedButtons: Qt.LeftButton | Qt.RightButton
    onClicked: function(mouse) {
      if (mouse.button === Qt.LeftButton) {
        musicModel.scan_music()
      } else if (mouse.button === Qt.RightButton) {
        addMenu.popup()
      }
    }
  }


}