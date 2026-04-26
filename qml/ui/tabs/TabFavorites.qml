/* --- loonixtunesv2/qml/ui/TabFavorites.qml | Favorites Tab --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Qt.labs.platform

Rectangle {
  id: favoritesTab
  width: 30
  height: 20
  property bool isActive: musicModel.current_folder_qml.toUpperCase() === 'FAVORITES'
  color: isActive || tabMA_favorites.containsMouse ? theme.colormap.bgoverlay : 'transparent'
  radius: 4
  border.width: 0.5
  border.color: isActive || tabMA_favorites.containsMouse
    ? theme.colormap.tabhover
    : theme.colormap.tabborder

  Text {
    anchors.centerIn: parent
    text: ''
    font.family: symbols.name
    font.pixelSize: 12
    font.bold: parent.isActive
    color: theme.colormap.tabtext
  }

  MouseArea {
    id: tabMA_favorites
    anchors.fill: parent
    hoverEnabled: true
    acceptedButtons: Qt.LeftButton
    onClicked: function(mouse) {
      if (mouse.button === Qt.LeftButton) {
        musicModel.switch_to_favorites()
      }
    }
  }


}
