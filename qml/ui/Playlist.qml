/* --- loonixtunesv2/qml/ui/Playlist.qml | Playlist --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls

Rectangle {
  id: playlistSection
  Layout.fillWidth: true
  Layout.fillHeight: true
  color: 'transparent'

  // BORDER ATAS: Tempel di TOP, kunci HEIGHT
  Rectangle {
    height: 8
    anchors.top: parent.top
    anchors.left: parent.left
    anchors.right: parent.right
    color: theme.colormap.bgmain
  }

  // BORDER BAWAH SPACER: bgmain, 8px di atas border
  Rectangle {
    height: 8
    anchors.bottom: parent.bottom
    anchors.bottomMargin: 8
    anchors.left: parent.left
    anchors.right: parent.right
    color: theme.colormap.bgmain
  }

  // BORDER KIRI: Tempel di LEFT, kunci WIDTH
  Rectangle {
    width: 8
    anchors.left: parent.left
    anchors.top: parent.top
    anchors.bottom: parent.bottom
    color: theme.colormap.bgoverlay
  }

  // BORDER KANAN: Tempel di RIGHT, kunci WIDTH
  Rectangle {
    width: 8
    anchors.right: parent.right
    anchors.top: parent.top
    anchors.bottom: parent.bottom
    color: theme.colormap.bgoverlay
  }

  ListView {
    id: playlistView
    anchors.fill: parent
    anchors.leftMargin: 16
    anchors.rightMargin: 8
    anchors.topMargin: 8
    anchors.bottomMargin: 16
    topMargin: 0
    bottomMargin: 0
    clip: true
    model: musicModel

    ScrollBar.vertical: ScrollBar {
        id: vBar
        width: 6
        policy: ScrollBar.AsNeeded

        background: Rectangle {
            color: "transparent"
        }

        contentItem: Rectangle {
            implicitWidth: 6
            implicitHeight: Math.max(30, playlistView.height * playlistView.visibleArea.heightRatio)
            radius: 3
            color: vBar.pressed ? theme.colormap.playeraccent : 
                   vBar.hovered ? theme.colormap.headerhover : 
                   theme.colormap.playeraccent
            opacity: vBar.active ? 1.0 : 0.5

            Behavior on color { ColorAnimation { duration: 150 } }
            Behavior on opacity { NumberAnimation { duration: 150 } }
        }
    }

    // Empty state message
    Rectangle {
      anchors.centerIn: parent
      width: parent.width - 40
      height: 80
      color: 'transparent'
      visible: playlistView.count === 0

      Column {
        anchors.centerIn: parent
        spacing: 8

        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: 'No music found'
          font.family: kodeMono.name
          font.pixelSize: 14
          color: theme.colormap.playersubtext
        }

        Text {
          anchors.horizontalCenter: parent.horizontalCenter
          text: "Click + to add folder.\nRight Click song and choose 'fav' to add your file into favorite tab."
          font.family: kodeMono.name
          font.pixelSize: 11
          color: theme.colormap.graysolid
          wrapMode: Text.WordWrap
          horizontalAlignment: Text.AlignHCenter
        }
      }
    }

    Component.onCompleted: {
      musicModel.current_index_changed.connect(function() {
        if (musicModel.current_index >= 0) {
          playlistView.positionViewAtIndex(musicModel.current_index, ListView.Center)
        }
      })
    }

    delegate: Component {
      id: playlistDelegate
      Rectangle {
        width: playlistView.width
        height: 26
        color: 'transparent'
        property bool isPlayingNow: !model.is_folder && model.name === musicModel.current_title
        property bool isInFolder: model.parent_folder !== '' &&
          model.parent_folder !== undefined &&
          model.parent_folder !== null
        property bool isHovered: false

        // Check if this item is at root level for the current tab
        property bool isAtRootLevel: model.parent_folder === musicModel.current_tab_root || 
                                   (model.parent_folder === '' && musicModel.current_tab_root !== '')


        // WRAPPER UNTUK PADDING DINAMIS
        Item {
          anchors.fill: parent
           anchors.leftMargin: {
               // Jika dia item paling luar (gak punya parent_folder), 
               // atau jika kita bukan di Tab Music, margin WAJIB 0.
               if (model.parent_folder === "" || musicModel.current_tab_root !== "MUSIC") {
                   return 0;
               }
               // Jika dia di Tab Music DAN dia punya parent (hasil expand), kasih 15.
               return 15;
           }

          

          // Left border 1px grey untuk item yang di-expand
          // Hanya visible untuk item di folder yang di-expand
          Rectangle {
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            width: 1
            color: theme.colormap["graysolid"]
            visible: isInFolder && musicModel.is_folder_expanded(model.parent_folder)
          }

          Text {
            id: playlistIcon
            anchors.left: parent.left
            anchors.verticalCenter: parent.verticalCenter
            text: isPlayingNow ? '󰶻' : model.is_folder ? '󱍙' : '󰽷'
            font.family: symbols.name
            color: isPlayingNow || isHovered || root.rightClickedIndex === index
              ? theme.colormap.playlistactive
              : theme.colormap.playlisticon
            font.pixelSize: model.is_folder ? 20 : 14
            leftPadding: 6
          }

          Text {
            text: model.name
            color: {
              if (isPlayingNow || isHovered || root.rightClickedIndex === index) return theme.colormap.playlistactive
              if (model.is_folder) return theme.colormap.playlistfolder
              return theme.colormap.playlisttext
            }
            font.family: kodeMono.name
            font.pixelSize: model.is_folder ? 14 : 13
            font.bold: isPlayingNow
            Layout.fillWidth: true
            elide: Text.ElideRight
            anchors.left: playlistIcon.right
            leftPadding: 6
            anchors.right: parent.right
            anchors.rightMargin: 4
            anchors.verticalCenter: parent.verticalCenter
          }
        }

        MouseArea {
          anchors.fill: parent
          hoverEnabled: true
          acceptedButtons: Qt.LeftButton | Qt.RightButton
          onEntered: parent.isHovered = true
          onExited: parent.isHovered = false

            onClicked: function(mouse) {
              if (mouse.button === Qt.LeftButton) {
                if (model.is_folder) {
                  musicModel.toggle_folder(index)
                } else {
                  musicModel.play_at(index)
                }
              } else if (mouse.button === Qt.RightButton) {
               parent.isHovered = false
               root.popupMenuVisible = false
               root.tabContextMenuVisible = false
               root.externalFilesContextMenuVisible = false
               root.rightClickedIndex = index
               root.playlistContextItemIndex = index
               root.playlistContextItemName = String(model.name || "")
               root.playlistContextItemPath = String(model.path || "")
               root.playlistContextIsFolder = Boolean(model.is_folder)

              var menuHeight = 170
              var menuWidth = 170
              var bottomPos = parent.mapToItem(null, 0, parent.height)
              var topPos = parent.mapToItem(null, 0, 0)

              var spaceBelow = root.height - bottomPos.y
              var spaceAbove = topPos.y

              if (spaceBelow >= menuHeight) {
                root.playlistContextMenuY = bottomPos.y | 0
              } else if (spaceAbove >= menuHeight) {
                root.playlistContextMenuY = (topPos.y - menuHeight) | 0
              } else {
                root.playlistContextMenuY = bottomPos.y | 0
              }

              if (bottomPos.x + menuWidth > root.width) {
                root.playlistContextMenuX = (root.width - menuWidth) | 0
              } else {
                root.playlistContextMenuX = bottomPos.x | 0
              }

              root.playlistContextMenuVisible = true
            }
          }

          onPressAndHold: {
            root.popupX = mouseX
            root.popupY = mouseY
            root.popupMenuVisible = true
          }
        }
      }
    }
  }
    // BORDER BAWAH: Tempel di BOTTOM, kunci HEIGHT
  Rectangle {
    height: 8
    anchors.bottom: parent.bottom
    anchors.left: parent.left
    anchors.right: parent.right
    color: theme.colormap.bgoverlay
  }
}
