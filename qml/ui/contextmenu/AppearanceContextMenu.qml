/* --- LOONIX-TUNES qml/ui/contextmenu/AppearanceContextMenu.qml --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

Item {
  id: appearanceCtxRoot
  z: 10001
  visible: prefPage.appearanceMenuVisible
  property int presetIndex: -1
  implicitWidth: menuGrid.width + 16
  implicitHeight: menuGrid.height + 16

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

  

  GridLayout {
    id: menuGrid
    anchors.centerIn: parent
    columns: 2
    rowSpacing: 2
    columnSpacing: 2

    // TILE 1 - Edit
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
          text: ''
          font.family: symbols.name
          font.pixelSize: 18
          color: tile1MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Edit'
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
          var idx = prefPage.appearanceMenuIndex
          var themeName = theme.get_custom_theme_name(idx)
          theme.set_theme(themeName)
          prefPage.appearanceMenuVisible = false
          root.prefThemeEditorProfileTarget = idx
          root.prefThemeEditorVisible = true
        }
      }
    }

    // TILE 2 - Rename
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
          color: tile2MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Rename'
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
          prefPage.openRenameDialog()
        }
      }
    }

    // TILE 3 - Cancel
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
          color: theme.colormap.playlistactive
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Cancel'
          font.family: kodeMono.name
          font.pixelSize: 10
          color: theme.colormap.playlistactive
        }
      }
      MouseArea {
        id: tile3MA
        anchors.fill: parent
        hoverEnabled: true
        onClicked: prefPage.appearanceMenuVisible = false
      }
    }

    // TILE 4 - Reset
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
          text: '⟲'
          font.family: symbols.name
          font.pixelSize: 18
          color: tile4MA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
        }
        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'Reset'
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
          prefPage.appearanceMenuVisible = false
          theme.set_custom_theme_colors(prefPage.appearanceMenuIndex, theme.get_default_colors())
        }
      }
    }
  }
}