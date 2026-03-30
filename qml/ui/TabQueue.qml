/* --- LOONIX-TUNES qml/ui/TabQueue.qml --- */

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import Qt.labs.platform

Rectangle {
  id: queueTab
  width: 30
  height: 20
  visible: musicModel.queue_count > 0
  property bool isActive: musicModel.current_folder_qml.toUpperCase() === 'QUEUE'
  color: isActive || tabMA_queue.containsMouse ? theme.colormap.bgoverlay : 'transparent'
  radius: 4
  border.width: 0.5
  antialiasing: false
  border.color: isActive || tabMA_queue.containsMouse
    ? theme.colormap.tabhover
    : theme.colormap.tabborder

  Text {
    anchors.centerIn: parent
    text: '󰬘'
    font.family: symbols.name
    font.pixelSize: 12
    font.bold: parent.isActive
    color: theme.colormap.tabtext
  }

  MouseArea {
    id: tabMA_queue
    anchors.fill: parent
    hoverEnabled: true
    acceptedButtons: Qt.LeftButton
    onClicked: function(mouse) {
      if (mouse.button === Qt.LeftButton) {
        musicModel.switch_to_queue()
      }
    }
  }


}