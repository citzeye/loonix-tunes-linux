/* --- LOONIX-TUNES qml/ui/HeaderMenu.qml --- */
import QtQuick

Item {
  anchors.fill: parent
  // --- HAMBURGER MENU (Custom Popup) ---
  Rectangle {
    id: popupMenuOverlay
    visible: root.popupMenuVisible
    z: 1000
    anchors.fill: parent
    color: '#40000000'
    MouseArea {
      anchors.fill: parent
      hoverEnabled: true
      acceptedButtons: Qt.LeftButton | Qt.RightButton
      onPressed: root.popupMenuVisible = false
    }
  }

  Rectangle {
    id: popupMenuContainer
    z: 1001
    visible: root.popupMenuVisible
    width: 160
    height: menuColumn.height
    x: popupX
    y: popupY
    color: theme.colormap['bgmain']
    border.color: theme.colormap['tabborder']
    border.width: 1
    radius: 4
    antialiasing: false
    focus: true

    Column {
      id: menuColumn
      width: parent.width

      Repeater {
        model: popupMenu.menu_items

        delegate: Rectangle {
          width: 160
          height: 30
          color: 'transparent'

          Text {
            anchors.fill: parent
            verticalAlignment: Text.AlignVCenter
            leftPadding: 10
            text: modelData.text
            font.family: kodeMono.name
            font.pixelSize: 12
            color: menuItemMouse.containsMouse
              ? theme.colormap['headerhover']
              : theme.colormap['playersubtext']
          }

          MouseArea {
            id: menuItemMouse
            anchors.fill: parent
            enabled: modelData.enabled
            hoverEnabled: true
            onClicked: {
              if (modelData.action !== '') {
                popupMenu.trigger_action(modelData.action)
                root.popupMenuVisible = false
              }
            }
          }
        }
      }
    }
  }

  Connections {
    target: popupMenu
    function onAction_triggered() {
      let action = popupMenu.selected_action

      if (action === 'about') {
        root.aboutDialogVisible = !root.aboutDialogVisible
        root.donateDialogVisible = false
      } else if (action === 'donate') {
        root.donateDialogVisible = !root.donateDialogVisible
        root.aboutDialogVisible = false
      } else if (action === 'SUBMENU_SETTINGS' || action === 'settings') {
        root.settingsDialogVisible = true
      }

      root.popupMenuVisible = false
    }
  }

}
