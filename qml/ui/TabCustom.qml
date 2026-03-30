/* --- LOONIX-TUNES qml/ui/TabCustom.qml --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Qt.labs.platform

Rectangle {
  id: customTabItem
  width: Math.min(tabText.implicitWidth + 30, 100)
  height: 20
  property bool isActive: musicModel.current_folder_qml.toUpperCase() ===
    musicModel.get_custom_folder_name(index).toUpperCase()
  property int tabIndex: index
  property Item customRow: parent
  color: isActive || tabMA_custom.containsMouse ? theme.colormap.bgoverlay : 'transparent'
  radius: 4
  border.width: isActive ? 0.5 : 0.5
  border.color: isActive || tabMA_custom.containsMouse
    ? theme.colormap.tabhover
    : theme.colormap.tabborder

  Text {
    id: tabText
    anchors.centerIn: parent
    text: musicModel.get_custom_folder_name(index)
    font.family: kodeMono.name
    font.pixelSize: 10
    font.bold: parent.isActive
    color: theme.colormap.tabtext
    anchors.leftMargin: 0
    anchors.rightMargin: 0
  }

  MouseArea {
    id: tabMA_custom
    anchors.fill: parent
    hoverEnabled: true
    acceptedButtons: Qt.LeftButton | Qt.RightButton
    onClicked: function(mouse) {
      if (mouse.button === Qt.LeftButton) {
        musicModel.switch_to_folder(musicModel.get_custom_folder_path(index))
      } else if (mouse.button === Qt.RightButton) {
        root.popupMenuVisible = false
        root.playlistContextMenuVisible = false
        root.externalFilesContextMenuVisible = false
        root.tabContextMenuIndex = index
        root.tabContextMenuType = 'custom'
        var itemRect = parent
        root.popupX = itemRect.x + customRow.x + tabBarSection.x
        root.popupY = itemRect.y + customRow.y + tabBarSection.y + itemRect.height
        root.tabContextMenuVisible = true
      }
    }
  }
}