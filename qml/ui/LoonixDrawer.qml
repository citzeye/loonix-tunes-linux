/* --- LOONIX-TUNES qml/ui/LoonixDrawer.qml --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Drawer {
  id: root
  property string title: 'PANEL'
  property Component content: null
  property int edgePosition: Qt.RightEdge

  edge: edgePosition
  width: edge === Qt.LeftEdge || edge === Qt.RightEdge ? 350 : parent.width
  height: edge === Qt.TopEdge || edge === Qt.BottomEdge ? parent.height * 0.55 : parent.height
  interactive: false

  background: Rectangle {
    color: theme.colormap['bgmain']
    opacity: 0.95
    border.color: theme.colormap['tabborder']
    border.width: 1
    antialiasing: false
  }

  ColumnLayout {
    anchors.fill: parent
    anchors.margins: 20
    spacing: 15

    // Header Panel
    RowLayout {
      Layout.fillWidth: true
      Text {
        text: root.title
        font.family: kodeMono.name
        font.pixelSize: 16
        font.bold: true
        color: theme.colormap['playeraccent']
        Layout.fillWidth: true
      }
      Text {
        text: '󰅖'
        font.family: symbols.name
        color: theme.colormap['playersubtext']
        MouseArea {
          anchors.fill: parent
          onClicked: root.close()
        }
      }
    }

    // Garis Pemisah
    Rectangle {
      Layout.fillWidth: true
      Layout.preferredHeight: 1
      color: theme.colormap['tabborder']
      opacity: 0.3
    }

    // Tempat konten dimuat
    Loader {
      id: contentLoader
      Layout.fillWidth: true
      Layout.fillHeight: true
      sourceComponent: root.content
    }
  }

  ScrollBar.vertical: ScrollBar {
    width: 6
    z: 1
    policy: ScrollBar.AsNeeded
    background: Rectangle { implicitWidth: 6; color: theme.colormap.bgmain; opacity: 0.0 }
    contentItem: Rectangle {
      implicitWidth: 6
      radius: 3
      color: parent.pressed ? theme.colormap.playeraccent : (parent.hovered ? theme.colormap.playerhover : theme.colormap.graysolid)
      Behavior on color { ColorAnimation { duration: 200 } }
    }
  }
}
